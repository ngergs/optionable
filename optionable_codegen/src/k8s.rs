use crate::helper::{error, is_serde};
use crate::parsed_input::FieldHandling::{OptionedOnly, Required};
use crate::parsed_input::{FieldParsed, StructParsed};
use crate::{TypeHelperAttributesK8sOpenapi, TypeHelperAttributesKube};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::BTreeSet;
use std::borrow::Cow;
use syn::{
    parse_quote, Attribute, Data, Error, Field, ImplGenerics, Path, TypeGenerics, WhereClause,
};

/// We have two useful `Resource` traits and dependent on the user needs we will use one
/// of them for adding the API envelope `apiVersion` and `kind` fields when serializing.
pub(crate) enum ResourceType {
    K8sOpenApi,
    Kube,
}

/// Validates the resource configuration and returns the resource type.
/// Returns `None` for `kube` if no serde derives are present as all we are doing
/// is adding a `PhantomData` field to serialize/deserialize the API-Envelope.
pub(crate) fn k8s_resource_type(
    attrs_k8s_openapi: Option<&TypeHelperAttributesK8sOpenapi>,
    attrs_kube: Option<&TypeHelperAttributesKube>,
    derive: &BTreeSet<String>,
) -> Result<Option<ResourceType>, Error> {
    if attrs_k8s_openapi.is_some() && attrs_kube.is_some() {
        return error(
            "Conflicting configuration. Only one of the `#[optionable(k8s_openapi)]` or `#[optionable(kube)]` attribute is allowed at once per type.",
        );
    }
    if attrs_k8s_openapi
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
    {
        return Ok(Some(ResourceType::K8sOpenApi));
    }
    if attrs_kube
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
        && derive.iter().any(|el| is_serde(el.as_str()))
    {
        return Ok(Some(ResourceType::Kube));
    }
    Ok(None)
}

/// Additional derives to set for `k8s-openapi` types.
pub(crate) fn k8s_openapi_derives(data: &Data) -> Option<Vec<String>> {
    match data {
        Data::Struct(_) => Some(vec![
            "Clone".to_owned(),
            "std::fmt::Debug".to_owned(),
            "Default".to_owned(),
            "PartialEq".to_owned(),
            "serde::Serialize".to_owned(),
            "serde::Deserialize".to_owned(),
        ]),
        Data::Enum(_) => Some(vec![
            "Clone".to_owned(),
            "std::fmt::Debug".to_owned(),
            "PartialEq".to_owned(),
            "serde::Serialize".to_owned(),
            "serde::Deserialize".to_owned(),
        ]),
        Data::Union(_) => None,
    }
}

/// Adjust the struct fields according to the `Kubernetes` use case.
pub(crate) fn k8s_adjust_fields(
    input_crate: Option<&Ident>,
    struct_parsed: &mut StructParsed,
    attr_k8s_openapi: Option<&TypeHelperAttributesK8sOpenapi>,
    attr_kube: Option<&TypeHelperAttributesKube>,
    resource_type: Option<&ResourceType>,
    crate_name: &Path,
) -> Result<(), Error> {
    if attr_k8s_openapi.is_some() {
        k8s_openapi_adjust_field_serde_renames(struct_parsed)?;
    }
    if attr_k8s_openapi.is_some_and(|attr| attr.metadata.is_some())
        || attr_kube.is_some_and(|attr| attr.resource.is_some())
    {
        k8s_openapi_set_metadata_required(struct_parsed);
    }
    if let Some(k8s_resource_type) = &resource_type {
        k8s_openapi_field_resource_add_api_envelope( input_crate,struct_parsed, k8s_resource_type, crate_name);
    }
    Ok(())
}

/// Adjust the parsed struct for the `k8s_openapi::Metadata` requirements.
fn k8s_openapi_set_metadata_required(struct_parsed: &mut StructParsed) {
    struct_parsed
        .fields
        .iter_mut()
        .filter(|f| f.field.ident.as_ref().is_some_and(|f| *f == "metadata"))
        .for_each(|f| f.handling = Required);
}

/// Adjust the serde handling of specific fields to handle fields deviating from just `camelCase`.
/// See the [`get_rust_ident`](https://docs.rs/k8s-openapi-codegen-common/latest/k8s_openapi_codegen_common/fn.get_rust_ident.html) used by `k8s_openapi`.
fn k8s_openapi_adjust_field_serde_renames(struct_parsed: &mut StructParsed) -> Result<(), Error> {
    struct_parsed.fields.iter_mut().try_for_each(|f| {
        if let Some(name) = &f.field.ident
            && let Some(name_serialized) = k8s_openapi_serde_rename(name.to_string().as_ref())
        {
            f.field
                .attrs
                .push(parse_quote!(#[optionable_attr(serde(rename=#name_serialized))]));
        }
        Ok(())
    })
}

/// Reverts some special case handling from `k8s_openapi` and also follows the naming schema from upstream k8s.
/// Testing via the roundtrip test for k8s-openapi serialization (generated for each type via codegen),
/// Return `Some(..)` and the rename value for special cases and `None` if it is not a special case.
/// See also the [`get_rust_ident`](https://docs.rs/k8s-openapi-codegen-common/latest/k8s_openapi_codegen_common/fn.get_rust_ident.html) used by `k8s_openapi`.
#[allow(clippy::missing_panics_doc)] // see discussion below regarding why using `next` and `pop` is safe.
fn k8s_openapi_serde_rename(input: &str) -> Option<Cow<'static, str>> {
    const UPPERCASE_WORDS: &[&str; 15] = &[
        "api", "cidr", "cpu", "fqdn", "id", "io", "ip", "ipc", "pid", "tls", "uid", "uuid", "uri",
        "url", "wwn",
    ];
    const PLURAL_WORDS: &[&str; 6] = &["cidrs", "ids", "ips", "uris", "urls", "wwns"];
    match input {
        "ref_path" => Some("$ref".into()),
        "schema" => Some("$schema".into()),
        "as_" => Some("as".into()),
        "continue_" => Some("continue".into()),
        "enum_" => Some("enum".into()),
        "ref_" => Some("ref".into()),
        "type_" => Some("type".into()),
        other => {
            if !other.split('_').enumerate().any(|(i, el)| {
                i != 0 && (UPPERCASE_WORDS.contains(&el) || PLURAL_WORDS.contains(&el))
            }) {
                return None;
            }
            let name: String = other
                .split('_')
                .enumerate()
                .map(|(i, el)| {
                    // first word stays lowercase
                    if i == 0 || el.is_empty() {
                        return Cow::Borrowed(el);
                    }
                    if UPPERCASE_WORDS.contains(&el) {
                        return el.to_uppercase().into();
                    }
                    if PLURAL_WORDS.contains(&el) {
                        // capitalize everything except the last char
                        let mut chars: Vec<char> = el.chars().collect();
                        // unwrap on the very first `pop()` is fine here, we checked in the beginning that el isn't empty
                        let last = chars.pop().unwrap().to_string();
                        return (chars.into_iter().collect::<String>().to_uppercase() + &last)
                            .into();
                    }
                    // capitalize first char
                    let mut chars = el.chars();
                    // unwrap on the very first `next()` is fine here, we checked in the beginning that el isn't empty
                    (chars.next().unwrap().to_uppercase().to_string() + chars.as_str()).into()
                })
                .collect();
            Some(name.into())
        }
    }
}

#[test]
fn roundtrip_k8s_openapi_adjust_field_serde() {
    const SPECIAL_KEYS: &[&str] = &[
        "clusterIP",
        "clusterIPs",
        "diskURI",
        "externalIP",
        "externalIPs",
        "hostIP",
        "hostIPs",
        "nonResourceURL",
        "nonResourceURLs",
        "pdID",
        "podCIDR",
        "podCIDRs",
        "podIP",
        "podIPs",
        "serverAddressByClientCIDR",
        "serverAddressByClientCIDRs",
        "targetWWN",
        "targetWWNs",
        "volumeID",
        "$ref",
        "$schema",
        "as",
        "continue",
        "enum",
        "ref",
        "type",
    ];
    for key in SPECIAL_KEYS {
        let rust_ident = k8s_openapi_codegen_common::get_rust_ident(key);
        let key_roundtrip = k8s_openapi_serde_rename(rust_ident.as_ref());
        assert!(key_roundtrip.is_some());
        assert_eq!(key.to_owned(), key_roundtrip.unwrap().into_owned());
    }
}

/// The field type helper attributes for an optioned `Struct` or `Enum`.
pub(crate) fn k8s_type_attr(data: &Data) -> Option<Attribute> {
    match data {
        Data::Struct(_) => Some(parse_quote!(#[serde(rename_all="camelCase",deny_unknown_fields)])),
        Data::Enum(_) => {
            Some(parse_quote!(#[serde(rename_all_fields="camelCase",deny_unknown_fields)]))
        }
        Data::Union(_) => None,
    }
}

/// Adjust the parsed struct for the `k8s_openapi::Resource` or `kube::Resource` requirements by adding
/// a `PhantomData` field with serialization helper attributes that serializes the apiGroup/kind
/// envelope from the `k8s_openapi::Metadata` or `kube::Resource` information.
/// Also, validation for deserialization is added.
fn k8s_openapi_field_resource_add_api_envelope(
    input_crate: Option<&Ident>,
    struct_parsed: &mut StructParsed,
    resource_type: &ResourceType,
    crate_name: &Path,
) {
    let mut envelope_serde_path = crate_name.to_token_stream().to_string();
    match resource_type {
        ResourceType::K8sOpenApi => {
            envelope_serde_path.push_str("::");
            envelope_serde_path.push_str(input_crate.to_token_stream().to_string().as_str());
        }
        ResourceType::Kube => {
            envelope_serde_path.push_str("::kube");
        }
    }
    let serialize_api_version_fn =
        with_suffix(envelope_serde_path.clone(), "::serialize_api_version");
    let serialize_kind_fn = with_suffix(envelope_serde_path.clone(), "::serialize_kind");
    let deserialize_api_version_fn =
        with_suffix(envelope_serde_path.clone(), "::deserialize_api_version");
    let deserialize_kind_fn = with_suffix(envelope_serde_path, "::deserialize_kind");
    let api_version_field: Field = parse_quote!(
                   #[optionable_attr(serde(serialize_with=#serialize_api_version_fn,deserialize_with=#deserialize_api_version_fn))]
                   pub api_version: std::marker::PhantomData<Self>
    );
    let kind_field: Field = parse_quote! (
                   #[optionable_attr(serde(serialize_with=#serialize_kind_fn,deserialize_with=#deserialize_kind_fn))]
                   pub kind: std::marker::PhantomData<Self>
    );
    struct_parsed.fields.splice(
        0..0,
        vec![
            FieldParsed {
                field: api_version_field,
                handling: OptionedOnly,
            },
            FieldParsed {
                field: kind_field,
                handling: OptionedOnly,
            },
        ],
    );
}

/// Adds a suffix to a string. Helper to reduce lines of code in the function above
fn with_suffix(mut val: String, suffix: &'static str) -> String {
    val.push_str(suffix);
    val
}

/// Derives `k8s_openapi::Resource` for the `T::Optioned` type from the non-optioned type.
/// It has to be ensured that the given `T` implements `k8s_openapi::Resource`.
pub(crate) fn k8s_openapi_impl_resource(
    input_crate: Option<&Ident>,
    ty_ident: &Path,
    ty_ident_opt: &Ident,
    impl_generics: &ImplGenerics,
    ty_generics: &TypeGenerics,
    where_clause_impl_optionable: &WhereClause,
) -> TokenStream {
    quote!(
        impl #impl_generics #input_crate::Resource for #ty_ident_opt #ty_generics #where_clause_impl_optionable {
            const API_VERSION: &'static str = <#ty_ident #ty_generics as #input_crate::Resource>::API_VERSION;
            const GROUP: &'static str = <#ty_ident #ty_generics as #input_crate::Resource>::GROUP;
            const KIND: &'static str = <#ty_ident #ty_generics as #input_crate::Resource>::KIND;
            const VERSION: &'static str = <#ty_ident #ty_generics as #input_crate::Resource>::VERSION;
            const URL_PATH_SEGMENT: &'static str =
               <#ty_ident #ty_generics as #input_crate::Resource>::URL_PATH_SEGMENT;
            type Scope = <#ty_ident #ty_generics as #input_crate::Resource>::Scope;
        }
    )
}

/// Derives `k8s_openapi::Metadata` for the `T::Optioned` type from the non-optioned type.
/// It has to be ensured that the given `T` implements `k8s_openapi::Metadata`.
pub(crate) fn k8s_openapi_impl_metadata(
    input_crate: Option<&Ident>,
    ty_ident: &Path,
    ty_ident_opt: &Ident,
    impl_generics: &ImplGenerics,
    ty_generics: &TypeGenerics,
    where_clause_impl_optionable: &WhereClause,
) -> TokenStream {
    quote!(
        impl #impl_generics #input_crate::Metadata for #ty_ident_opt #ty_generics #where_clause_impl_optionable {
            type Ty = <#ty_ident #ty_generics as #input_crate::Metadata>::Ty;

            fn metadata(&self) -> &<Self as #input_crate::Metadata>::Ty {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut <Self as #input_crate::Metadata>::Ty {
                &mut self.metadata
            }
        }
    )
}

/// Errors if `k8s_openapi_*`  or `kube_*` type attributes are set without the corresponding feature being enabled.
/// The feature does nothing besides this on this crate but is used to track that the required features are enabled
/// on the user-facing `optionable`-crate.
// false positive depending on the features enabled
#[allow(unused_variables)]
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn error_missing_features(
    attrs_k8s_openapi: Option<&TypeHelperAttributesK8sOpenapi>,
    attrs_kube: Option<&TypeHelperAttributesKube>,
) -> Result<(), Error> {
    #[cfg(not(feature = "k8s_openapi"))]
    if attrs_k8s_openapi
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
    {
        return error(
            "helper attributes `#[optionable(k8s_openapi(resource))] require one of the `k8s_openapi_*` features to be enabled for the `optionable` crate.",
        );
    }
    #[cfg(not(feature = "kube"))]
    if attrs_kube
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
    {
        return error(
            "helper attributes `#[optionable(kube(resource)] require the `kube` feature to be enabled for the `optionable` crate.",
        );
    }
    Ok(())
}
