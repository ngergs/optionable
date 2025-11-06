use crate::helper::error;
use crate::parsed_input::FieldHandling::{OptionedOnly, Required};
use crate::parsed_input::{FieldParsed, StructParsed};
use crate::TypeHelperAttributes;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_quote, Attribute, Data, DeriveInput, Error, ImplGenerics, Path, TypeGenerics, WhereClause,
};

/// We have two useful `Resource` traits and dependent on the user needs we will use one
/// of them for adding the API envelope `apiVersion` and `kind` fields when serializing.
pub(crate) enum ResourceType {
    K8sOpenApi,
    Kube,
}

/// Validates the resource configuration and returns the resource type.
pub(crate) fn k8s_resource_type(
    attrs: &TypeHelperAttributes,
) -> Result<Option<ResourceType>, Error> {
    if attrs.k8s_openapi.is_some() && attrs.kube.is_some() {
        return error(
            "Conflicting configuration. Only one of the `#[optionable(k8s_openapi)]` or `#[optionable(kube)]` attribute is allowed at once per type.",
        );
    }
    if attrs
        .k8s_openapi
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
    {
        return Ok(Some(ResourceType::K8sOpenApi));
    }
    if attrs
        .kube
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
    {
        return Ok(Some(ResourceType::Kube));
    }
    Ok(None)
}

/// Additional derives to set for `k8s` types.
pub(crate) fn k8s_derives(input: &DeriveInput) -> Option<Vec<Path>> {
    match input.data {
        Data::Struct(_) => Some(vec![
            Path::from_string("Clone").unwrap(),
            Path::from_string("std::fmt::Debug").unwrap(),
            Path::from_string("Default").unwrap(),
            Path::from_string("PartialEq").unwrap(),
            Path::from_string("serde::Serialize").unwrap(),
            Path::from_string("serde::Deserialize").unwrap(),
        ]),
        Data::Enum(_) => Some(vec![
            Path::from_string("Clone").unwrap(),
            Path::from_string("std::fmt::Debug").unwrap(),
            Path::from_string("PartialEq").unwrap(),
            Path::from_string("serde::Serialize").unwrap(),
            Path::from_string("serde::Deserialize").unwrap(),
        ]),
        Data::Union(_) => None,
    }
}

/// The field type helper attributes for an optioned `Struct` or `Enum`.
pub(crate) fn k8s_type_attr(input: &DeriveInput) -> Option<Attribute> {
    match input.data {
        Data::Struct(_) => Some(parse_quote!(#[serde(rename_all="camelCase")])),
        Data::Enum(_) => Some(parse_quote!(#[serde(rename_all="camelCase",untagged)])),
        Data::Union(_) => None,
    }
}

/// Adjust the parsed struct for the `k8s_openapi::Metadata` requirements.
pub(crate) fn k8s_openapi_set_metadata_required(struct_parsed: &mut StructParsed) {
    struct_parsed
        .fields
        .iter_mut()
        .filter(|f| f.field.ident.as_ref().is_some_and(|f| *f == "metadata"))
        .for_each(|f| f.handling = Required);
}

/// Adjust the parsed struct for the `k8s_openapi::Resource` requirements.
pub(crate) fn k8s_openapi_field_resource_adjust(
    struct_parsed: &mut StructParsed,
    crate_name: &Path,
    ty_ident_opt: &Ident,
    ty_generics: &TypeGenerics,
    resource_type: &ResourceType,
) {
    let mut serialize_fn_name = crate_name.to_token_stream().to_string();
    match resource_type {
        ResourceType::K8sOpenApi => {
            serialize_fn_name.push_str("::k8s_openapi::serialize_api_envelope");
        }
        ResourceType::Kube => {
            serialize_fn_name.push_str("::kube::serialize_api_envelope");
        }
    }
    let field = parse_quote!(
                   #[optionable_attr(serde(flatten,serialize_with=#serialize_fn_name,skip_deserializing))]
                   pub phantom: std::marker::PhantomData<#ty_ident_opt #ty_generics>
    );
    struct_parsed.fields.push(FieldParsed {
        field,
        handling: OptionedOnly,
    });
}

/// Derives `k8s_openapi::Resource` for the `T::Optioned` type from the non-optioned type.
/// It has to be ensured that the given `T` implements `k8s_openapi::Resource`.
pub(crate) fn k8s_openapi_impl_resource(
    ty_ident: &Path,
    ty_ident_opt: &Ident,
    impl_generics: &ImplGenerics,
    ty_generics: &TypeGenerics,
    where_clause_impl_optionable: &WhereClause,
) -> TokenStream {
    quote!(
        impl #impl_generics k8s_openapi::Resource for #ty_ident_opt #ty_generics #where_clause_impl_optionable {
            const API_VERSION: &'static str = <#ty_ident #ty_generics as k8s_openapi::Resource>::API_VERSION;
            const GROUP: &'static str = <#ty_ident #ty_generics as k8s_openapi::Resource>::GROUP;
            const KIND: &'static str = <#ty_ident #ty_generics as k8s_openapi::Resource>::KIND;
            const VERSION: &'static str = <#ty_ident #ty_generics as k8s_openapi::Resource>::VERSION;
            const URL_PATH_SEGMENT: &'static str =
               <#ty_ident #ty_generics as k8s_openapi::Resource>::URL_PATH_SEGMENT;
            type Scope = <#ty_ident #ty_generics as k8s_openapi::Resource>::Scope;
        }
    )
}

/// Derives `k8s_openapi::Metadata` for the `T::Optioned` type from the non-optioned type.
/// It has to be ensured that the given `T` implements `k8s_openapi::Metadata`.
pub(crate) fn k8s_openapi_impl_metadata(
    ty_ident: &Path,
    ty_ident_opt: &Ident,
    impl_generics: &ImplGenerics,
    ty_generics: &TypeGenerics,
    where_clause_impl_optionable: &WhereClause,
) -> TokenStream {
    quote!(
        impl #impl_generics k8s_openapi::Metadata for #ty_ident_opt #ty_generics #where_clause_impl_optionable {
            type Ty = <#ty_ident #ty_generics as k8s_openapi::Metadata>::Ty;

            fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
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
pub(crate) fn error_missing_features(attrs: &TypeHelperAttributes) -> Result<(), Error> {
    #[cfg(not(feature = "k8s_openapi"))]
    if attrs
        .k8s_openapi
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
    {
        return error(
            "helper attributes `#[optionable(k8s_openapi(resource))] require one of the `k8s_openapi_*` features to be enabled for the `optionable` crate.",
        );
    }
    #[cfg(not(feature = "kube"))]
    if attrs
        .kube
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
    {
        return error(
            "helper attributes `#[optionable(kube(resource)] require the `kube` feature to be enabled for the `optionable` crate.",
        );
    }
    Ok(())
}
