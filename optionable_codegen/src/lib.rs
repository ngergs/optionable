//! The relevant main crate is [optionable](https://crates.io/crates/optionable). The docs can be found there.
//!
//! # Purpose
//! This code generation `proc_macro2` library serves two purposes:
//! - Used by [optionable_derive](https://crates.io/crates/optionable_derive) to implement the `#[derive(Optionable)]`-macro
//!   re-exported by [optionable](https://crates.io/crates/optionable_derive).
//! - Used by the [bin/codegen.rs](bin/codegen.rs) crate to support generating `Optionable`-implementations for external packages.
//!   Due to the orphan rule  the generated code has to be added to the `Optionable`-package (PRs welcome).
//!
//! It has to be a separate crate from [optionable_derive](https://crates.io/crates/optionable_derive) as the proc-macro crates
//! can't export its non-macro functions (even the `proc_macro2` ones) for the usage by the codegen part.

mod helper;
mod k8s_openapi;
mod parsed_input;
mod where_clause;

use crate::helper::{destructure, error, error_on_helper_attributes, is_serialize, struct_wrapper};
use crate::k8s_openapi::{
    error_missing_features, k8s_adjust_fields, k8s_derives, k8s_openapi_impl_metadata,
    k8s_openapi_impl_resource, k8s_resource_type, k8s_type_attr,
};
use crate::parsed_input::{
    into_field_handling, FieldHandling, FieldParsed, StructParsed, StructType,
};
use crate::where_clause::{where_clauses, WhereClauses};
use darling::util::PathList;
use darling::{FromAttributes, FromDeriveInput, FromMeta};
use itertools::MultiUnzip;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::default::Default;
use std::mem::take;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_quote, Attribute, Data, DeriveInput, Error, Field, Fields, LitStr, Meta, MetaList, Path, Token,
    Type, TypePath, WhereClause,
};

const HELPER_IDENT: &str = "optionable";
const HELPER_ATTR_IDENT: &str = "optionable_attr";
const ERR_MSG_HELPER_ATTR_ENUM_VARIANTS: &str =
    "#[optionable] helper attributes not supported on enum variant level.";

#[derive(FromDeriveInput)]
#[darling(attributes(optionable))]
/// Helper attributes on the type definition level (attached to the `struct` or `enum` itself).
pub(crate) struct TypeHelperAttributes {
    /// Forwards the specified `attr` to the definition of the optioned type.
    /// If any `key` is set it filters the configuration of the forwarded attributes for the specified configuration names.
    /// E.g. `#[optionable(attr_copy(attr=serde,key=rename)]` will only forward the `rename`sub-attribute of serde.
    #[darling(multiple)]
    attr_copy: Vec<FieldAttributeToCopy>,
    /// Derive-macros that should be added to the optioned type
    #[darling(multiple)]
    derive: Vec<PathList>,
    /// Explicit suffix to use for the optioned type.
    suffix: Option<LitStr>,
    /// Skip generating `OptionableConvert` impl
    no_convert: Option<()>,
    /// Wrap a field that is already an `Option<T>` inside an outer `Option<Option<T>>` for the optioned type.
    /// Useful for struct patching but not helpful once serializing to e.g. JSON is involved.
    option_wrap: Option<()>,
    /// Adjustments of the derived optioned type for fields that use the `k8s_openapi` serialization replacements.
    /// - Derives for the optioned type `Clone, Debug, PartialEq, Serialize, Deserialize` and additionally for Structs `Default`.
    /// - Takes the `k8s_openapi` serialization replacements into account for its own serialization.
    k8s_openapi: Option<TypeHelperAttributesK8sOpenapi>,
    /// Adjustments of the derived optioned type for structs that `kube::CustomResource` or respective subfields.
    /// - Derives for the optioned type `Clone, Debug, PartialEq, Serialize, Deserialize` and additionally for Structs `Default`.
    /// - Sets `#[serde(rename_all="camelCase",deny_unknown_fields)]`
    /// - Copy `#[(serde(rename="...")]` and `#[(serde(rename_all="...")]` attributes over to the optioned types.
    kube: Option<TypeHelperAttributesKube>,
}

#[derive(FromMeta)]
/// Adjustments of the derived optioned type for fields that use the `k8s_openapi` serialization replacements.
/// - Derives for the optioned type `Clone, Debug, PartialEq, Serialize, Deserialize` and additionally for Structs `Default`.
/// -
/// - Takes the `k8s_openapi` serialization replacements into account for its own serialization.
pub(crate) struct TypeHelperAttributesK8sOpenapi {
    /// Adjustments of the derived optioned type for `k8s_openapi::Metadata`-implementations.
    /// - Sets the `metadata` field as required for the optioned type.
    /// - Derives `k8s_openapi::Metadata` for the optioned type.
    metadata: Option<()>,
    /// Adjustments of the derived optioned type for `k8s_openapi::Resource`-implementations.
    /// - Adds `apiVersion` and `kind` to the serialization output with values from the trait constants of the given `k8s_openapi::Resource` implementation
    /// - Derives `k8s_openapi::resource` for the optioned type.
    resource: Option<()>,
}

#[derive(FromMeta)]
/// Adjustments of the derived optioned type for structs that `kube::CustomResource` or respective subfields.
/// - Derives for the optioned type `Clone, Debug, PartialEq, Serialize, Deserialize` and additionally for Structs `Default`.
pub(crate) struct TypeHelperAttributesKube {
    /// Adjustments of the derived optioned type for `kube::Resource`-implementations.
    /// - Sets the `metadata` field as required for the optioned type.
    /// - Adds `apiVersion` and `kind` to the serialization output with values from the trait constants of the given `kube::Resource` implementation
    /// - Derives `kube::Resource` for the optioned type.
    resource: Option<()>,
}

#[derive(FromMeta)]
/// Forwards the specified `attr` to the definition of the optioned type.
/// If `sub_names` is set it filters the configuration of the forwarded attribure for the specified configuration names.
/// E.g. `attr=serde,sub_names=rename` will only forward the `rename`sub-attribute of serde.
pub struct FieldAttributeToCopy {
    pub attr: Path,
    pub key: Option<Path>,
}

#[derive(FromAttributes)]
#[darling(attributes(optionable))]
/// Helper attributes on the type definition level (attached to the `struct` or `enum` itself).
struct FieldHelperAttributes {
    /// Given field won't be optioned, it will also be required for the derived optioned type.
    required: Option<()>,
    /// For circumvention of the orphan rule. Uses the provided type for the optioned variant of the field.
    /// If `no_convert` is not set the provided type has to implement `OptionedConvert<O>` where `O` is the type of this field in the original type.
    #[darling(rename = "optioned_type")]
    optioned_ty: Option<Path>,
}

#[derive(FromDeriveInput, Debug, Clone)]
#[darling(attributes(optionable))]
/// Settings that are only available to be set via the rust function signature of `derive_optionable`
/// but not via the derive macro (as they are not useful in that context and would lead to an
/// increased but not useful API surface of the derive macro helpers).
pub struct CodegenSettings {
    /// Name of the optionable crate to use for the generated code including potentially
    /// leading `::`. Useful to set when generating code for the `optionable`
    /// crate itself where one needs to refer to it as `crate`.
    pub optionable_crate_name: Path,
    /// Path prefix to prepend to the respective type. E.g. with a `ty_prefix` of `::mycrate`
    /// the output would be (simplified) `impl Optionable for ::mycrate::mytype`.
    pub ty_prefix: Option<Path>,
    /// Replacement for the keyword `crate` in the input type/enum definition or references.
    /// Useful when generating code for the `optionable` crate as pre-existing `crate` references
    /// need to be replaced with the concret crate name.
    pub input_crate_replacement: Option<Ident>,
}

/// Processes the input field attribute forwards to a `HashMap`.
/// Adds `attr=serde,key=rename` if the `kube` input attribute is present.
fn field_attr_copy_hashmap(
    input: Vec<FieldAttributeToCopy>,
    attr_kube: Option<&TypeHelperAttributesKube>,
) -> HashMap<Path, HashSet<Path>> {
    let mut result = HashMap::<Path, HashSet<Path>>::new();
    for el in input {
        if let Some(key) = el.key {
            if let Some(entry) = result.get_mut(&el.attr) {
                entry.insert(key);
            } else {
                result.insert(el.attr, HashSet::from([key]));
            }
        }
    }
    if attr_kube.is_some() {
        let path_serde = Path::from_string("serde").unwrap();
        let path_rename = Path::from_string("rename").unwrap();
        let path_rename_all = Path::from_string("rename_all").unwrap();
        let path_rename_all_fields = Path::from_string("rename_all_fields").unwrap();
        if let Some(entry) = result.get_mut(&path_serde) {
            // If the keys are empty everything will be copied, so don't restrict it here
            if !entry.is_empty() {
                entry.insert(path_rename);
                entry.insert(path_rename_all);
            }
        } else {
            result.insert(
                path_serde,
                HashSet::from([path_rename, path_rename_all, path_rename_all_fields]),
            );
        }
    }
    result
}

impl Default for CodegenSettings {
    fn default() -> Self {
        Self {
            optionable_crate_name: parse_quote!(::optionable),
            ty_prefix: None,
            input_crate_replacement: None,
        }
    }
}

/// Returns the attribute for opting-out of `OptionableConvert`-impl generation.
#[must_use]
pub fn attribute_no_convert() -> Attribute {
    parse_quote!(#[optionable(no_convert)])
}

/// Returns the attribute for setting a custom struct/enum name suffix instead of `Opt`.
#[must_use]
pub fn attribute_suffix(suffix: &str) -> Attribute {
    parse_quote!(#[optionable(suffix=#suffix)])
}

/// Returns the attribute for setting that the given identifiers should be added as
/// derive macro to the optioned struct/enum.
#[must_use]
pub fn attribute_derives(derives: &PathList) -> Attribute {
    parse_quote!(#[optionable(derive(#(#derives),*))])
}

/// Derives the `Optionable`-trait from the main `optionable`-library.
///
/// # Errors
/// - on misplaced helper attributes
/// - internal implementation errors
#[allow(clippy::too_many_lines)]
#[allow(clippy::items_after_statements)]
pub fn derive_optionable(
    input: DeriveInput,
    settings: Option<&CodegenSettings>,
) -> syn::Result<TokenStream> {
    let settings = settings.map(Cow::Borrowed).unwrap_or_default();
    let crate_name = &settings.optionable_crate_name;
    let TypeHelperAttributes {
        attr_copy,
        derive: attr_derive,
        suffix: attr_suffix,
        no_convert: attr_no_convert,
        option_wrap: attr_option_wrap,
        k8s_openapi: attr_k8s_openapi,
        kube: attr_kube,
    } = TypeHelperAttributes::from_derive_input(&input)?;
    let DeriveInput {
        attrs,
        vis,
        mut generics,
        data,
        ..
    } = input;
    error_missing_features(attr_k8s_openapi.as_ref(), attr_kube.as_ref())?;

    let ty_ident_opt = {
        let suffix = attr_suffix.map_or_else(
            || {
                if attr_kube.is_some() || attr_k8s_openapi.is_some() {
                    Cow::Borrowed("Ac")
                } else {
                    Cow::Borrowed("Opt")
                }
            },
            |s| Cow::Owned(s.value()),
        );
        format_ident!("{}{suffix}", &input.ident)
    };
    let ty_ident = if let Some(mut ty_prefix) = settings.ty_prefix.clone() {
        ty_prefix.segments.push(input.ident.into());
        ty_prefix
    } else {
        input.ident.into()
    };
    if let Data::Enum(e) = &data
        && e.variants
            .iter()
            .all(|el| matches!(el.fields, Fields::Unit))
    {
        // plain enum without nested structs/tuples
        return Ok(impl_optionable_self(
            crate_name,
            &ty_ident,
            attr_no_convert.is_some(),
        ));
    }

    let k8s_resource_type = k8s_resource_type(attr_k8s_openapi.as_ref(), attr_kube.as_ref())?;
    let k8s_openapi_attrs = attr_k8s_openapi.is_some().then(|| k8s_type_attr(&data));
    let attr_copy_identifier = field_attr_copy_hashmap(attr_copy, attr_kube.as_ref());
    let ty_attr_forwarded = forwarded_attributes(&attrs, &attr_copy_identifier)?;

    let mut derive = attr_derive
        .into_iter()
        .flat_map(|el| {
            el.iter()
                .map(|el| el.to_token_stream().to_string())
                .collect::<Vec<_>>()
        })
        .collect::<BTreeSet<_>>();
    if (attr_k8s_openapi.is_some() || attr_kube.is_some())
        && let Some(k8s_derives) = k8s_derives(&data)
    {
        for el in k8s_derives {
            derive.insert(el);
        }
    }

    let vis = vis;
    // basically split_for_impl, but we move the where clause out instead of just taking a reference
    let where_clause = take(&mut generics.where_clause);
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let generics_colon = (!generics.params.is_empty()).then(|| quote! {::});
    let skip_optionable_if_serde_serialize =
        (attr_k8s_openapi.is_some() // also sets #[derive(Serialize)]
            || derive.iter().any(|el| is_serialize(el.as_str())))
        .then(|| quote!(#[serde(skip_serializing_if = "Option::is_none")]));

    /// Helper to collect the enum/struct specific derived code aspects in a typesafe way
    struct Derived {
        /// Either `enum` or `struct`
        enum_struct: TokenStream,
        /// The generated fields, including wrapping brackets
        fields: TokenStream,
        /// The `where`-clause for the `Optionable`-impl
        where_clause_optionable: WhereClause,
        /// The `where`-clause  for the `OptionableConvert` and `OptionedConvert` impl
        where_clause_optionable_convert: Option<WhereClause>,
        /// The `OptionableConvert` implementation if not configured to be skipped, including the `impl`-wrapper
        impl_optionable_convert: Option<TokenStream>,
    }
    let Derived {
        enum_struct,
        fields,
        where_clause_optionable,
        where_clause_optionable_convert,
        impl_optionable_convert,
    } = match data {
        Data::Struct(s) => {
            let mut struct_parsed = into_field_handling(
                crate_name.to_owned(),
                s.fields,
                settings.input_crate_replacement.as_ref(),
                attr_option_wrap,
            )?;
            k8s_adjust_fields(
                settings.input_crate_replacement.as_ref(),
                &mut struct_parsed,
                attr_k8s_openapi.as_ref(),
                attr_kube.as_ref(),
                k8s_resource_type.as_ref(),
                crate_name,
            )?;
            let WhereClauses {
                optionable: where_clause_optionable,
                optionable_convert: where_clause_optionable_convert,
            } = where_clauses(
                where_clause,
                &generics.params,
                crate_name,
                settings.input_crate_replacement.as_ref(),
                &derive,
                attr_no_convert.is_some(),
                &struct_parsed.fields,
            )?;
            let unnamed_struct_semicolon =
                (struct_parsed.struct_type == StructType::Unnamed).then(|| quote!(;));
            let optioned_fields = optioned_fields(
                &struct_parsed,
                skip_optionable_if_serde_serialize.as_ref(),
                &attr_copy_identifier,
            )?;

            let impl_optionable_convert = attr_no_convert.is_none().then(|| {
                let into_optioned_fields = into_optioned(&struct_parsed, |selector| quote! { self.#selector });
                let try_from_optioned_fields =
                    try_from_optioned(&struct_parsed, |selector| quote! { value.#selector });
                let merge_fields = merge_fields(
                    &struct_parsed,
                    |selector| quote! { self.#selector },
                    |selector| quote! { other.#selector },
                    true);
                quote! {
                    #[automatically_derived]
                    impl #impl_generics #crate_name::OptionableConvert for #ty_ident #ty_generics #where_clause_optionable_convert{
                        fn into_optioned(self) -> #ty_ident_opt #ty_generics {
                            #ty_ident_opt #generics_colon #ty_generics #into_optioned_fields
                        }

                        fn try_from_optioned(value: #ty_ident_opt #ty_generics) -> Result<Self, #crate_name::Error>{
                            Ok(Self #try_from_optioned_fields)
                        }

                        fn merge(&mut self, other: #ty_ident_opt #ty_generics) -> Result<(), #crate_name::Error>{
                            #merge_fields
                            Ok(())
                        }
                    }
                }
            });
            Derived {
                enum_struct: quote! {struct},
                fields: quote! {#optioned_fields #unnamed_struct_semicolon},
                where_clause_optionable,
                where_clause_optionable_convert,
                impl_optionable_convert,
            }
        }
        Data::Enum(e) => {
            let self_prefix = quote! {self_};
            let other_prefix = quote! {other_};

            let variants = e
                .variants
                .into_iter()
                .map(|v| {
                    error_on_helper_attributes(&v.attrs, ERR_MSG_HELPER_ATTR_ENUM_VARIANTS)?;
                    let mut field_handling = into_field_handling(
                        crate_name.to_owned(),
                        v.fields,
                        settings.input_crate_replacement.as_ref(),
                        attr_option_wrap,
                    )?;
                    k8s_adjust_fields(
                        settings.input_crate_replacement.as_ref(),
                        &mut field_handling,
                        attr_k8s_openapi.as_ref(),
                        attr_kube.as_ref(),
                        k8s_resource_type.as_ref(),
                        crate_name,
                    )?;
                    Ok::<_, Error>((
                        v.ident,
                        forwarded_attributes(&v.attrs, &attr_copy_identifier)?,
                        field_handling,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let all_fields = variants
                .iter()
                .flat_map(|(_, _, fields)| &fields.fields)
                .collect::<Vec<_>>();
            let WhereClauses {
                optionable: where_clause_optionable,
                optionable_convert: where_clause_optionable_convert,
            } = where_clauses(
                where_clause,
                &generics.params,
                crate_name,
                settings.input_crate_replacement.as_ref(),
                &derive,
                attr_no_convert.is_some(),
                all_fields,
            )?;

            let optioned_variants = variants
                .iter()
                .map(|(variant, forward_attrs, f)| {
                    let fields = optioned_fields(
                        f,
                        skip_optionable_if_serde_serialize.as_ref(),
                        &attr_copy_identifier,
                    )?;
                    Ok::<_, Error>(quote!( #forward_attrs #variant #fields ))
                })
                .collect::<Result<Vec<_>, _>>()?;

            let impl_optionable_convert = attr_no_convert.is_none().then(|| {
                let (into_variants, try_from_variants, merge_variants): (Vec<_>, Vec<_>, Vec<_>) = variants
                    .iter()
                    .map(|(variant, _, struct_parsed)| {
                        let fields_into = into_optioned(struct_parsed, |selector| {
                            format_ident!("{self_prefix}{selector}").to_token_stream()
                        });
                        let fields_try_from = try_from_optioned
                            (struct_parsed, |selector| {
                                format_ident!("{other_prefix}{selector}").to_token_stream()
                            });
                        let fields_merge = merge_fields(struct_parsed,
                                                        |selector| format_ident!("{self_prefix}{selector}").to_token_stream(),
                                                        |selector| format_ident!("{other_prefix}{selector}").to_token_stream(),
                                                        false);
                        let self_destructure = destructure(struct_parsed, &self_prefix)?;
                        let other_destructure = destructure(struct_parsed, &other_prefix)?;
                        Ok::<_, Error>((
                            quote!( Self::#variant #self_destructure => #ty_ident_opt::#variant #fields_into ),
                            quote!( #ty_ident_opt::#variant #other_destructure => Self::#variant #fields_try_from ),
                            quote!( #ty_ident_opt::#variant #other_destructure => {
                                if let Self::#variant #self_destructure = self {
                                    #fields_merge
                                } else {
                                    *self = Self::try_from_optioned(#ty_ident_opt::#variant #other_destructure)?;
                                }
                            })
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter().multiunzip();
                Ok::<_, Error>(quote! {
                    #[automatically_derived]
                    impl #impl_generics #crate_name::OptionableConvert for #ty_ident #ty_generics #where_clause_optionable_convert {
                        fn into_optioned(self) -> #ty_ident_opt #ty_generics {
                            match self {
                                #(#into_variants),*
                            }
                        }

                        fn try_from_optioned(other: #ty_ident_opt #ty_generics)->Result<Self,#crate_name::Error>{
                            Ok(match other{
                                #(#try_from_variants),*
                            })
                        }

                        fn merge(&mut self, other: #ty_ident_opt #ty_generics) -> Result<(), #crate_name::Error>{
                            match other{
                                #(#merge_variants),*
                            }
                            Ok(())
                        }
                    }
                })
            }).transpose()?;
            Derived {
                enum_struct: quote! {enum},
                fields: quote! {{#(#optioned_variants),*}},
                where_clause_optionable,
                where_clause_optionable_convert,
                impl_optionable_convert,
            }
        }
        Data::Union(_) => return error("#[derive(Optionable)] not supported for unit structs"),
    };
    let impl_optioned_convert = attr_no_convert.is_none().then(|| {
        quote! {
            #[automatically_derived]
            impl #impl_generics #crate_name::OptionedConvert<#ty_ident #ty_generics> for #ty_ident_opt #ty_generics #where_clause_optionable_convert {
                fn from_optionable(value: #ty_ident #ty_generics) -> Self {
                    #crate_name::OptionableConvert::into_optioned(value)
                }
                fn try_into_optionable(self) -> Result<#ty_ident #ty_generics, #crate_name::Error> {
                    #crate_name::OptionableConvert::try_from_optioned(self)
                }
                fn merge_into(self, other: &mut #ty_ident #ty_generics) -> Result<(), #crate_name::Error> {
                    #crate_name::OptionableConvert::merge(other, self)
                }
            }
        }
    });

    let derive = derive
        .into_iter()
        .map(|derive| Path::from_string(&derive))
        .collect::<Result<Vec<_>, _>>()?;
    let derive = (!derive.is_empty()).then(|| quote! {#[derive(#(#derive),*)]});

    let k8s_openapi_impl_resource = attr_k8s_openapi
        .as_ref()
        .is_some_and(|attr| attr.resource.is_some())
        .then(|| {
            k8s_openapi_impl_resource(
                settings.input_crate_replacement.as_ref(),
                &ty_ident,
                &ty_ident_opt,
                &impl_generics,
                &ty_generics,
                &where_clause_optionable,
            )
        });
    let impl_k8s_metadata = attr_k8s_openapi
        .as_ref()
        .is_some_and(|attr| attr.metadata.is_some())
        .then(|| {
            k8s_openapi_impl_metadata(
                settings.input_crate_replacement.as_ref(),
                &ty_ident,
                &ty_ident_opt,
                &impl_generics,
                &ty_generics,
                &where_clause_optionable,
            )
        });
    let kube_derive_resource = attr_kube
        .is_some_and(|attr| attr.resource.is_some())
        .then(|| {
            quote! {
                #[derive(kube::Resource)]
                #[resource(inherit = #ty_ident )]
            }
        });
    let k8s_roundtrip_test = (attr_k8s_openapi.is_some_and(|attr|attr.resource.is_some())
        && ty_generics.to_token_stream().is_empty()
        // causes stackoverflows during the roundtrip tests, the stackoverflow already happens when trying to generate a `fake CRD`, so doesn't originate from this crate
        // (we rely on a forked version of k8s-openapi where we just added derives for `fake::Dummy`, but the CRD includes a `JSONSchemaProps` subfield which includes a lot
        // of other subfields involving `JSONSchemaProps`. Hence, `fake` easily generates a way to complicated and depply nested random structure).
        && ty_ident_opt!="CustomResourceDefinitionAc"
        // the cases below cause false positive errors due to flattening of effectively nil substructs in the  optioned type
        && ty_ident_opt!="StorageVersionAc"
        && ty_ident_opt!="DeviceClassAc"
        && ty_ident_opt!="ResourceClaimAc"
        && ty_ident_opt!="ResourceClaimTemplateAc")
        .then(|| {
            let fn_name = format_ident!(
                "roundtrip_{}",
                ty_ident_opt.to_token_stream().to_string().to_lowercase()
            );
            Ok::<_, Error>(quote! {
                #[cfg(test_k8s_openapi_roundtrip)]
                #[test]
                fn #fn_name() {
                   crate::testutil::roundtrip_test::<#ty_ident>();
                }
            })
        })
        .transpose()?;
    Ok(quote! {
                #derive
                #kube_derive_resource
                #ty_attr_forwarded
                #k8s_openapi_attrs
                #vis #enum_struct #ty_ident_opt #impl_generics #where_clause_optionable #fields

                #[automatically_derived]
                impl #impl_generics #crate_name::Optionable for #ty_ident #ty_generics #where_clause_optionable {
                    type Optioned = #ty_ident_opt #ty_generics;
                }

                #[automatically_derived]
                impl #impl_generics #crate_name::Optionable for #ty_ident_opt #ty_generics #where_clause_optionable {
                    type Optioned = #ty_ident_opt #ty_generics;
                }

                #impl_optionable_convert
                #impl_optioned_convert

                #k8s_openapi_impl_resource
                #impl_k8s_metadata

                #k8s_roundtrip_test
    })
}

/// Returns the `Optionable` and `OptionableConvert` implementation for self-resolving types
fn impl_optionable_self(crate_name: &Path, ty_ident: &Path, no_convert: bool) -> TokenStream {
    let convert_impl = (!no_convert).then(|| {
        quote! {
            #[automatically_derived]
            impl #crate_name::OptionableConvert for #ty_ident{
                fn into_optioned(self) -> #ty_ident {
                    self
                }

                fn try_from_optioned(value: Self::Optioned) -> Result<Self, #crate_name::Error> {
                    Ok(value)
                }

                fn merge(&mut self, other: Self::Optioned) -> Result<(), #crate_name::Error> {
                    *self = other;
                    Ok(())
                }
            }
        }
    });
    quote! {
        #[automatically_derived]
        impl #crate_name::Optionable for #ty_ident{
            type Optioned = Self;
        }

        #convert_impl
    }
}

/// Returns a tokenstream for the optioned fields and potential convert implementation of the optioned object (struct/enum variants).
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
fn optioned_fields(
    fields: &StructParsed,
    serde_attributes: Option<&TokenStream>,
    field_attr_forwards: &HashMap<Path, HashSet<Path>>,
) -> Result<TokenStream, Error> {
    let fields_token = fields.fields.iter().map(
        |FieldParsed {
             field: Field { attrs, vis, ident, ty, .. },
             handling,
         }| {
            let forwarded_attrs = forwarded_attributes(attrs, field_attr_forwards)?;
            let optioned_ty = optioned_ty(&fields.crate_name, ty);
            let colon = ident.as_ref().map(|_| quote! {:});
            Ok::<_, Error>(match handling {
                FieldHandling::Required | FieldHandling::OptionedOnly => quote! {#forwarded_attrs #vis #ident #colon #ty},
                FieldHandling::ManualOptioned(ty_opt) => quote! {#forwarded_attrs #vis #ident #colon Option<#ty_opt>},
                FieldHandling::IsOption => quote! {#forwarded_attrs #serde_attributes #vis #ident #colon #optioned_ty},
                FieldHandling::Other => quote! {#forwarded_attrs #serde_attributes #vis #ident #colon Option<#optioned_ty>},
            })
        },
    ).collect::<Result<Vec<_>, _>>()?;
    Ok(struct_wrapper(fields_token, &fields.struct_type))
}

/// Returns the field mapping implementation for `into_optioned`.
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
fn into_optioned(
    struct_parsed: &StructParsed,
    self_selector_fn: impl Fn(&TokenStream) -> TokenStream,
) -> TokenStream {
    let fields_token = struct_parsed.fields.iter().enumerate().map(|(i, FieldParsed { field: Field { ident, ty, .. }, handling })| {
        let colon = ident.as_ref().map(|_| quote! {:});
        let selector = ident.as_ref().map_or_else(|| {
            let i = Literal::usize_unsuffixed(i);
            quote! {#i}
        }, ToTokens::to_token_stream);
        let self_selector = self_selector_fn(&selector);
        let crate_name = &struct_parsed.crate_name;
        match (handling, is_self_resolving_optioned(ty)) {
            (FieldHandling::Required, _) | (FieldHandling::IsOption, true) => quote! {#ident #colon #self_selector},
            (FieldHandling::ManualOptioned(_), _) => quote! {#ident #colon Some(#crate_name::OptionedConvert::from_optionable(#self_selector))},
            (FieldHandling::IsOption, false) => quote! {#ident #colon #crate_name::OptionableConvert::into_optioned(#self_selector)},
            (FieldHandling::Other, true) => quote! {#ident #colon Some(#self_selector)},
            (FieldHandling::Other, false) => quote! {#ident #colon Some(#crate_name::OptionableConvert::into_optioned(#self_selector))},
            (FieldHandling::OptionedOnly, _) => quote! {#ident #colon Default::default()},
        }
    });
    struct_wrapper(fields_token, &struct_parsed.struct_type)
}

/// Returns the field-mappings implementation for `try_from_optioned`.
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
fn try_from_optioned(
    struct_parsed: &StructParsed,
    value_selector_fn: impl Fn(&TokenStream) -> TokenStream,
) -> TokenStream {
    let fields_token = struct_parsed.fields.iter().enumerate().filter_map(|(i, FieldParsed { field: Field { ident, ty, .. }, handling })| {
        let colon = ident.as_ref().map(|_| quote! {:});
        let selector = ident.as_ref().map_or_else(|| {
            let i = Literal::usize_unsuffixed(i);
            quote! {#i}
        }, ToTokens::to_token_stream);
        let value_selector = value_selector_fn(&selector);
        let crate_name = &struct_parsed.crate_name;
        match (handling, is_self_resolving_optioned(ty)) {
            (FieldHandling::Required, _) | (FieldHandling::IsOption, true) => Some(quote! {#ident #colon value.#selector}),
            (FieldHandling::ManualOptioned(_), _) => {
                let selector_quoted = LitStr::new(&selector.to_string(), ident.span());
                Some(quote! {
                    #ident #colon #crate_name::OptionedConvert::try_into_optionable(#value_selector.ok_or(#crate_name::Error{ missing_field: #selector_quoted })?
                    )?
                })
            }
            (FieldHandling::IsOption, false) => Some(quote! {
                #ident #colon #crate_name::OptionableConvert::try_from_optioned(
                    #value_selector
                )?
            }),
            (FieldHandling::Other, true) => {
                let selector_quoted = LitStr::new(&selector.to_string(), ident.span());
                Some(quote! {
                    #ident #colon #value_selector.ok_or(#crate_name::Error{ missing_field: #selector_quoted })?
                })
            }
            (FieldHandling::Other, false) => {
                let selector_quoted = LitStr::new(&selector.to_string(), ident.span());
                Some(quote! {
                    #ident #colon #crate_name::OptionableConvert::try_from_optioned(#value_selector.ok_or(#crate_name::Error{ missing_field: #selector_quoted })?
                    )?
                })
            }
            (FieldHandling::OptionedOnly, _) => None,
        }
    });

    struct_wrapper(fields_token, &struct_parsed.struct_type)
}

/// Returns the field-mappings implementation for `try_from_optioned`.
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
///
/// `merge_self_mut` should be true when the `self_selector` merge argument should be modified with a `& mut` on recursive calls.
fn merge_fields(
    struct_parsed: &StructParsed,
    self_selector_fn: impl Fn(&TokenStream) -> TokenStream,
    other_selector_fn: impl Fn(&TokenStream) -> TokenStream,
    merge_self_mut: bool,
) -> TokenStream {
    let fields_token = struct_parsed.fields.iter().enumerate().filter_map(
        |(
             i,
             FieldParsed {
                 field: Field { ident, ty, .. },
                 handling,
             },
         )| {
            let selector = ident.as_ref().map_or_else(
                || {
                    let i = Literal::usize_unsuffixed(i);
                    quote! {#i}
                },
                ToTokens::to_token_stream,
            );
            let self_merge_mut_modifier = merge_self_mut.then(|| quote! {&mut});
            let deref_modifier = (!merge_self_mut).then(|| quote! {*});
            let self_selector = self_selector_fn(&selector);
            let other_selector = other_selector_fn(&selector);
            let crate_name = &struct_parsed.crate_name;
            match (handling, is_self_resolving_optioned(ty)) {
                (FieldHandling::Required, _) | (FieldHandling::IsOption, true) => Some(quote! {#deref_modifier #self_selector = #other_selector;}),
                (FieldHandling::ManualOptioned(_), _) =>  Some(quote! {
                    if let Some(other_value)=#other_selector{
                        #crate_name::OptionedConvert::merge_into(other_value, #self_merge_mut_modifier #self_selector)?;
                    }
                }),
                (FieldHandling::IsOption, false) =>  Some(quote! {
                    #crate_name::OptionableConvert::merge(#self_merge_mut_modifier #self_selector, #other_selector)?;
                }),
                (FieldHandling::Other, true) =>  Some(quote! {
                    if let Some(other_value)=#other_selector{
                        #deref_modifier #self_selector = other_value;
                    }
                }),
                (FieldHandling::Other, false) =>  Some(quote! {
                    if let Some(other_value)=#other_selector{
                        #crate_name::OptionableConvert::merge(#self_merge_mut_modifier #self_selector, other_value)?;
                    }
                }),
                (FieldHandling::OptionedOnly, _) =>None,
            }
        },
    );

    quote! {
        #(#fields_token)*
    }
}
/// Tries to resolve the optioned type analogous to what we do in the main crate.
/// Due to limitations to macro resolving (no order guaranteed) we have to have an explicit
/// list of well-known types and their optioned types.
/// For now limited to self-resolving (mostly primitive) types
fn optioned_ty(crate_name: &Path, ty: &Type) -> TokenStream {
    if is_self_resolving_optioned(ty) {
        ty.to_token_stream()
    } else {
        quote! { <#ty as #crate_name::Optionable>::Optioned }
    }
}

const SELF_RESOLVING_TYPES: [&str; 18] = [
    // Rust primitives don't have inner structure, https://doc.rust-lang.org/rust-by-example/primitives.html
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "char", "bool", // Other types without inner structure
    "String", "OsString",
];

/// Checks when it is well known that the type resolves to itself as its `Optioned`.
/// Limited to self-resolving (mostly primitive) types.
fn is_self_resolving_optioned(ty: &Type) -> bool {
    if let Type::Path(TypePath { qself, path }) = &ty
        && qself.is_none()
        && SELF_RESOLVING_TYPES.contains(&&*path.to_token_stream().to_string())
    {
        true
    } else {
        false
    }
}

/// Extracts the `HELPER_ATTR_IDENT` attributes, unwraps them and returns them
/// as `TokenStream` that can be used as respective attribute.
fn forwarded_attributes(
    attrs: &[Attribute],
    attr_to_copy: &HashMap<Path, HashSet<Path>>,
) -> Result<Option<TokenStream>, Error> {
    let forward_attrs = attrs
        .iter()
        .map(|attr| {
            if attr.path().is_ident(HELPER_ATTR_IDENT) {
                return match &attr.meta {
                    Meta::List(MetaList { tokens, .. }) => Ok(Some(quote!(#[#tokens]))),
                    _ => error("Only lists like `#[optionable_attr(Serialize,Deserialize)]` are supported for `optionable_attr`"),
                };
            }
            let keys_to_copy = attr_to_copy.get(attr.path());
            if let Some(keys_to_copy) = keys_to_copy {
                // no key restrictions
                if keys_to_copy.is_empty() {
                    if attr.path().is_ident("derive") && let Meta::List(meta_list) = &attr.meta {
                        let derives = Punctuated::<Path, Token![,]>::parse_terminated
                            .parse2(meta_list.tokens.clone())?.into_iter().filter(|el| el.to_token_stream().to_string() != "Optionable" && el.to_token_stream().to_string() != "optionable::Optionable");
                        return Ok(Some(quote! {#[derive(#(#derives,)*)]}));
                    }
                    Ok(Some(attr.to_token_stream()))
                } else {
                    match &attr.meta {
                        Meta::Path(_) => Ok(None),
                        Meta::NameValue(meta_name_value) => Ok(keys_to_copy.contains(&meta_name_value.path).then(|| attr.to_token_stream())),
                        Meta::List(meta_list) => {
                            // we support one level of nesting for a Meta::List(Meta::NameValue(..)) setup like it's used by #[serde(rename=...)]
                            let inner_metas: Vec<TokenStream> = Punctuated::<Meta, Token![,]>::parse_terminated
                                .parse2(meta_list.tokens.clone())?.into_iter().filter_map(|meta| {
                                if let Meta::NameValue(meta_name_value) = meta {
                                    keys_to_copy.contains(&meta_name_value.path).then(|| Ok::<_, Error>(meta_name_value.to_token_stream()))
                                } else {
                                    None
                                }
                            }).collect::<Result<_, _>>()?;
                            if inner_metas.is_empty() {
                                Ok(None)
                            } else {
                                let attr = Attribute {
                                    meta: MetaList {
                                        path: meta_list.path.clone(),
                                        delimiter: meta_list.delimiter.clone(),
                                        tokens: inner_metas.into_iter().collect(),
                                    }.into(),
                                    ..*attr
                                };
                                Ok(Some(attr.to_token_stream()))
                            }
                        }
                    }
                }
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<TokenStream>();
    Ok((!forward_attrs.is_empty()).then_some(forward_attrs))
}

#[cfg(test)]
mod tests {
    use crate::{derive_optionable, CodegenSettings};
    use darling::FromMeta;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse_quote, Path};

    struct TestCase {
        input: TokenStream,
        output: TokenStream,
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_optionable() {
        let tcs = vec![
            // named struct fields
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    struct DeriveExample {
                        name: String,
                        pub surname: String,
                    }
                },
                output: quote! {
                    struct DeriveExampleOpt {
                        name: Option<String>,
                        pub surname: Option<String>
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleOpt {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned (self) -> DeriveExampleOpt {
                            DeriveExampleOpt  {
                                name: Some(self.name),
                                surname:Some(self.surname)
                            }
                        }

                        fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::Error> {
                            Ok(Self{
                                name: value.name.ok_or(::optionable::Error { missing_field: "name" })?,
                                surname: value.surname.ok_or(::optionable::Error { missing_field: "surname" })?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name = other_value;
                            }
                            if let Some(other_value) = other.surname {
                                self.surname = other_value;
                            }
                            Ok(())
                        }
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionedConvert<DeriveExample> for DeriveExampleOpt {
                        fn from_optionable(value: DeriveExample) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            // named struct fields, no convert impl
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    #[optionable(no_convert)]
                    struct DeriveExample {
                        name: String,
                        pub surname: String,
                    }
                },
                output: quote! {
                    struct DeriveExampleOpt {
                        name: Option<String>,
                        pub surname: Option<String>
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleOpt {
                        type Optioned = DeriveExampleOpt;
                    }
                },
            },
            // named struct fields with required fields
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    struct DeriveExample {
                        name: String,
                        #[optionable(required)]
                        pub surname: String,
                        #[optionable(optioned_type=MyInt)]
                        pub id: i32,
                    }
                },
                output: quote! {
                    struct DeriveExampleOpt {
                        name: Option<String>,
                        pub surname: String,
                        pub id: Option<MyInt>
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleOpt {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned (self) -> DeriveExampleOpt {
                            DeriveExampleOpt  {
                                name: Some(self.name),
                                surname: self.surname,
                                id: Some (::optionable::OptionedConvert::from_optionable(self.id))
                            }
                        }

                        fn try_from_optioned(value:DeriveExampleOpt ) -> Result <Self, ::optionable::Error> {
                            Ok (Self {
                                name: value.name.ok_or(::optionable::Error { missing_field: "name" })?,
                                surname: value.surname,
                                id: ::optionable::OptionedConvert::try_into_optionable(value.id.ok_or(::optionable::Error{ missing_field: "id" })?)?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name = other_value;
                            }
                            self.surname = other.surname;
                            if let Some (other_value) = other.id {
                                ::optionable::OptionedConvert::merge_into(other_value, &mut self.id)?;
                            }
                            Ok (())
                        }
                    }


                    #[automatically_derived]
                    impl ::optionable::OptionedConvert<DeriveExample> for DeriveExampleOpt {
                        fn from_optionable(value: DeriveExample) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            // named struct fields with forwarded derives and Serialize annotations
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    #[optionable(derive(Deserialize,Serialize,Default),suffix="Ac")]
                    #[optionable(attr_copy(attr=serde,key=rename))]
                    #[optionable_attr(serde(rename_all_fields = "camelCase", deny_unknown_fields))]
                    #[optionable_attr(serde(default))]
                    struct DeriveExample {
                        #[optionable_attr(serde(rename = "firstName"))]
                        name: String,
                        #[serde(rename="middle__name")]
                        middle_name: Option<String>,
                        surname: String,
                    }
                },
                output: quote! {
                    #[derive(Default, Deserialize, Serialize)]
                    #[serde(rename_all_fields = "camelCase", deny_unknown_fields)]
                    #[serde(default)]
                    struct DeriveExampleAc {
                        #[serde(rename = "firstName")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        name: Option<String>,
                        #[serde(rename = "middle__name")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        middle_name: <Option<String> as ::optionable::Optionable>::Optioned,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        surname: Option<String>
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleAc;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleAc {
                        type Optioned = DeriveExampleAc;
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned (self) -> DeriveExampleAc {
                            DeriveExampleAc  {
                                name: Some(self.name),
                                middle_name: ::optionable::OptionableConvert::into_optioned(self.middle_name),
                                surname: Some(self.surname)
                            }
                        }

                        fn try_from_optioned(value: DeriveExampleAc ) -> Result <Self, ::optionable::Error> {
                            Ok(Self{
                                name: value.name.ok_or(::optionable::Error { missing_field: "name"})?,
                                middle_name: ::optionable::OptionableConvert::try_from_optioned(value.middle_name)?,
                                surname: value.surname.ok_or(::optionable::Error { missing_field: "surname"})?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleAc ) -> Result<(), ::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name =  other_value;
                            }
                            ::optionable::OptionableConvert::merge(&mut self.middle_name, other.middle_name)?;
                            if let Some(other_value) = other.surname {
                                self.surname =  other_value;
                            }
                            Ok(())
                        }
                    }


                    #[automatically_derived]
                    impl ::optionable::OptionedConvert<DeriveExample> for DeriveExampleAc {
                        fn from_optionable(value: DeriveExample) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            // named struct fields with forwarded derives and Serialize annotations
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    #[optionable(derive(Deserialize,Serialize,Default),suffix="Ac")]
                    #[optionable(attr_copy(attr=serde,key=rename))]
                    #[optionable(option_wrap)]
                    #[optionable_attr(serde(rename_all_fields = "camelCase", deny_unknown_fields))]
                    #[optionable_attr(serde(default))]
                    struct DeriveExample {
                        #[optionable_attr(serde(rename = "firstName"))]
                        name: String,
                        #[serde(rename="middle__name")]
                        middle_name: Option<String>,
                        surname: String,
                    }
                },
                output: quote! {
                    #[derive(Default, Deserialize, Serialize)]
                    #[serde(rename_all_fields = "camelCase", deny_unknown_fields)]
                    #[serde(default)]
                    struct DeriveExampleAc {
                        #[serde(rename = "firstName")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        name: Option<String>,
                        #[serde(rename = "middle__name")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        middle_name: Option< <Option<String> as ::optionable::Optionable>::Optioned>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        surname: Option<String>
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleAc;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleAc {
                        type Optioned = DeriveExampleAc;
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned (self) -> DeriveExampleAc {
                            DeriveExampleAc  {
                                name: Some(self.name),
                                middle_name: Some(::optionable::OptionableConvert::into_optioned(self.middle_name)),
                                surname: Some(self.surname)
                            }
                        }

                        fn try_from_optioned(value: DeriveExampleAc ) -> Result <Self, ::optionable::Error> {
                            Ok(Self{
                                name: value.name.ok_or(::optionable::Error { missing_field: "name"})?,
                                middle_name: ::optionable::OptionableConvert::try_from_optioned(value.middle_name.ok_or(::optionable::Error{ missing_field : "middle_name" })?)?,
                                surname: value.surname.ok_or(::optionable::Error { missing_field: "surname"})?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleAc ) -> Result<(), ::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name =  other_value;
                            }
                            if let Some(other_value) = other.middle_name {
                                ::optionable::OptionableConvert::merge(&mut self.middle_name, other_value)?;
                            }
                            if let Some(other_value) = other.surname {
                                self.surname =  other_value;
                            }
                            Ok(())
                        }
                    }


                    #[automatically_derived]
                    impl ::optionable::OptionedConvert<DeriveExample> for DeriveExampleAc {
                        fn from_optionable(value: DeriveExample) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            // named struct fields with forwarded derives and Serialize annotations (full path variant)
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    #[optionable(derive(serde::Deserialize,serde::Serialize),suffix="Ac")]
                    struct DeriveExample {
                        name: String,
                        surname: String,
                    }
                },
                output: quote! {
                    #[derive(serde::Deserialize, serde::Serialize)]
                    struct DeriveExampleAc {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        surname: Option<String>
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleAc;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleAc {
                        type Optioned = DeriveExampleAc;
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned (self) -> DeriveExampleAc {
                            DeriveExampleAc {
                                name: Some(self.name),
                                surname: Some(self.surname)
                            }
                        }

                        fn try_from_optioned(value: DeriveExampleAc ) -> Result <Self, ::optionable::Error> {
                             Ok(Self{
                                name: value.name.ok_or(::optionable::Error{ missing_field: "name"})?,
                                surname: value.surname.ok_or(::optionable::Error{ missing_field: "surname"})?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleAc ) -> Result<(), ::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name = other_value;
                            }
                            if let Some(other_value) = other.surname {
                                self.surname = other_value;
                            }
                            Ok(())
                        }
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionedConvert<DeriveExample> for DeriveExampleAc {
                        fn from_optionable(value: DeriveExample) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            // unnamed struct fields
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    struct DeriveExample(pub String, i32);
                },
                output: quote! {
                    struct DeriveExampleOpt(
                        pub Option<String>,
                        Option<i32>
                    );

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleOpt {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned (self) -> DeriveExampleOpt {
                            DeriveExampleOpt (
                                Some(self.0),
                                Some(self.1)
                            )
                        }

                         fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::Error> {
                            Ok(Self(
                                value.0.ok_or(::optionable::Error { missing_field:"0" })?,
                                value.1.ok_or(::optionable::Error { missing_field: "1" })?
                            ))
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::Error> {
                            if let Some(other_value) = other.0 {
                                self.0 = other_value;
                            }
                            if let Some (other_value) = other.1 {
                                self.1 = other_value;
                            }
                            Ok (())
                        }
                    }


                    #[automatically_derived]
                    impl ::optionable::OptionedConvert<DeriveExample> for DeriveExampleOpt {
                        fn from_optionable(value: DeriveExample) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            // unnamed struct fields with required
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    struct DeriveExample(pub String, #[optionable(required)] i32);
                },
                output: quote! {
                    struct DeriveExampleOpt(
                        pub Option<String>,
                        i32
                    );

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleOpt {
                        type Optioned = DeriveExampleOpt;
                    }

                    # [automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned (self) -> DeriveExampleOpt {
                            DeriveExampleOpt (
                                Some(self.0),
                                self.1
                            )
                        }

                         fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::Error> {
                            Ok(Self(
                                value.0.ok_or(::optionable::Error { missing_field: "0" })?,
                                value.1))
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::Error> {
                            if let Some(other_value) = other.0 {
                                self.0 = other_value;
                            }
                            self.1 = other.1;
                            Ok (())
                        }
                    }


                    #[automatically_derived]
                    impl ::optionable::OptionedConvert<DeriveExample> for DeriveExampleOpt {
                        fn from_optionable(value: DeriveExample) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            // named struct fields with generics
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    #[optionable(derive(Serialize, Deserialize))]
                    struct DeriveExample<'a, T, T2: Serialize, T3> {
                        output: T,
                        input: Cow<'a, T2>,
                        #[optionable(required)]
                        extra: T3,
                    }
                },
                output: quote! {
                    #[derive (Deserialize, Serialize)]
                    struct DeriveExampleOpt<'a, T, T2: Serialize, T3>
                        where T: ::optionable::Optionable,
                              <T as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize,
                              Cow<'a ,T2>: ::optionable::Optionable,
                              <Cow<'a,T2> as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize
                    {
                        #[serde(skip_serializing_if="Option::is_none")]
                        output: Option< <T as ::optionable::Optionable>::Optioned>,
                        #[serde(skip_serializing_if="Option::is_none")]
                        input: Option< <Cow<'a, T2> as ::optionable::Optionable>::Optioned>,
                        extra: T3
                    }

                    #[automatically_derived]
                    impl<'a, T, T2: Serialize, T3> ::optionable::Optionable for DeriveExample<'a, T, T2, T3>
                        where T: ::optionable::Optionable,
                              <T as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize,
                              Cow<'a, T2>: ::optionable::Optionable,
                              <Cow<'a, T2> as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize
                    {
                        type Optioned = DeriveExampleOpt<'a, T,T2,T3>;
                    }

                    #[automatically_derived]
                    impl<'a, T, T2: Serialize, T3> ::optionable::Optionable for DeriveExampleOpt<'a ,T, T2, T3>
                        where T: ::optionable::Optionable,
                              <T as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize,
                              Cow<'a, T2>: ::optionable::Optionable,
                              <Cow<'a, T2> as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize
                    {
                        type Optioned = DeriveExampleOpt<'a, T, T2, T3>;
                    }

                    #[automatically_derived]
                    impl <'a, T, T2:Serialize, T3> ::optionable::OptionableConvert for DeriveExample<'a, T, T2, T3>
                        where T: ::optionable::OptionableConvert,
                              <T as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize,
                              Cow<'a, T2>: ::optionable::OptionableConvert,
                              <Cow<'a, T2> as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize
                    {
                        fn into_optioned (self) -> DeriveExampleOpt<'a, T, T2, T3> {
                            DeriveExampleOpt::<'a, T, T2, T3> {
                                output: Some(::optionable::OptionableConvert::into_optioned(self.output)),
                                input: Some(::optionable::OptionableConvert::into_optioned(self.input)),
                                extra: self.extra
                            }
                        }

                         fn try_from_optioned(value: DeriveExampleOpt<'a, T, T2, T3> ) -> Result <Self, ::optionable::Error> {
                             Ok(Self{
                                output: ::optionable::OptionableConvert::try_from_optioned(value.output.ok_or(::optionable::Error { missing_field: "output" })?)?,
                                input: ::optionable::OptionableConvert::try_from_optioned(value.input.ok_or(::optionable::Error { missing_field: "input" })?)?,
                                extra: value.extra
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt<'a, T, T2, T3> ) -> Result<(), ::optionable::Error> {
                            if let Some(other_value) = other.output {
                                ::optionable::OptionableConvert::merge(&mut self.output, other_value)?;
                            }
                            if let Some(other_value) = other.input {
                                ::optionable::OptionableConvert::merge(&mut self.input, other_value)?;
                            }
                            self.extra=other.extra;
                            Ok(())
                        }
                    }


                    #[automatically_derived]
                    impl <'a, T, T2:Serialize, T3> ::optionable::OptionedConvert<DeriveExample<'a, T, T2, T3> > for DeriveExampleOpt<'a, T, T2, T3>
                        where T: ::optionable::OptionableConvert,
                              <T as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize,
                              Cow<'a, T2>: ::optionable::OptionableConvert,
                              <Cow<'a, T2> as ::optionable::Optionable>::Optioned: Sized + serde::de::DeserializeOwned + Serialize
                    {
                        fn from_optionable(value: DeriveExample<'a, T, T2, T3>) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample<'a, T, T2, T3>, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample<'a, T, T2, T3>) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    enum DeriveExample {
                        Unit,
                        Plain(String),
                        Address{street: String, number: u32},
                        Address2(String,u32),
                    }
                },
                output: quote! {
                    enum DeriveExampleOpt {
                        Unit,
                        Plain( Option<String> ),
                        Address{ street: Option<String>, number:Option<u32> },
                        Address2( Option<String>, Option<u32> )
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExampleOpt {
                        type Optioned = DeriveExampleOpt;
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned (self) -> DeriveExampleOpt {
                            match self{
                                Self::Unit => DeriveExampleOpt::Unit,
                                Self::Plain(self_0) => DeriveExampleOpt::Plain(
                                    Some(self_0)
                                ),
                                Self::Address{street: self_street, number: self_number} => DeriveExampleOpt::Address{
                                    street: Some(self_street),
                                    number: Some(self_number)
                                },
                                Self::Address2(self_0, self_1) => DeriveExampleOpt::Address2(
                                    Some(self_0),
                                    Some(self_1)
                                )
                            }
                        }

                         fn try_from_optioned(other: DeriveExampleOpt) -> Result <Self, ::optionable::Error> {
                            Ok (match other {
                                DeriveExampleOpt::Unit => Self::Unit,
                                DeriveExampleOpt::Plain(other_0) => Self::Plain(
                                    other_0.ok_or(::optionable::Error { missing_field: "0" })?
                                ),
                                DeriveExampleOpt::Address{street: other_street, number: other_number} => Self::Address{
                                    street: other_street.ok_or(::optionable::Error { missing_field: "street" })?,
                                    number: other_number.ok_or(::optionable::Error { missing_field: "number" })?
                                },
                                DeriveExampleOpt::Address2(other_0, other_1) => Self::Address2(
                                    other_0.ok_or(::optionable::Error { missing_field: "0"})?,
                                    other_1.ok_or(::optionable::Error { missing_field: "1"})?)
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt) -> Result<(), ::optionable::Error> {
                            match other {
                                DeriveExampleOpt::Unit => {
                                    if let Self::Unit = self {} else {
                                        *self = Self::try_from_optioned(DeriveExampleOpt::Unit)?;
                                    }
                                },
                                DeriveExampleOpt::Plain(other_0) => {
                                    if let Self::Plain(self_0) = self{
                                        if let Some(other_value) = other_0 {
                                            *self_0 = other_value;
                                        }
                                    } else {
                                        *self = Self::try_from_optioned(DeriveExampleOpt::Plain(other_0))?;
                                    }
                                },
                                DeriveExampleOpt::Address{street: other_street, number: other_number} => {
                                    if let Self::Address{street: self_street, number: self_number}  = self{
                                        if let Some(other_value) = other_street {
                                            *self_street = other_value;
                                        }
                                        if let Some(other_value) = other_number {
                                            *self_number = other_value;
                                        }
                                    } else {
                                        *self = Self::try_from_optioned(DeriveExampleOpt::Address{street: other_street, number: other_number})?;
                                    }
                                },
                                DeriveExampleOpt::Address2(other_0, other_1) => {
                                    if let Self::Address2(self_0, self_1) = self{
                                        if let Some(other_value) = other_0 {
                                            *self_0 = other_value;
                                        }
                                        if let Some(other_value) = other_1 {
                                            *self_1 = other_value;
                                        }
                                    } else {
                                        *self = Self::try_from_optioned(DeriveExampleOpt::Address2(other_0, other_1))?;
                                    }
                                }
                            }
                            Ok(())
                        }
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionedConvert<DeriveExample> for DeriveExampleOpt {
                        fn from_optionable(value: DeriveExample) -> Self {
                            ::optionable::OptionableConvert::into_optioned(value)
                        }

                        fn try_into_optionable(self) -> Result<DeriveExample, ::optionable::Error> {
                            ::optionable::OptionableConvert::try_from_optioned(self)
                        }

                        fn merge_into(self, other: &mut DeriveExample) -> Result<(), ::optionable::Error> {
                            ::optionable::OptionableConvert::merge(other, self)
                        }
                    }
                },
            },
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    enum DeriveExample {
                        Unit,
                        Unit2,
                        Unit3
                    }
                },
                output: quote! {
                    #[automatically_derived]
                    impl ::optionable::Optionable for DeriveExample {
                        type Optioned = Self;
                    }

                    #[automatically_derived]
                    impl ::optionable::OptionableConvert for DeriveExample {
                        fn into_optioned(self) -> DeriveExample{
                            self
                        }

                        fn try_from_optioned(value: Self::Optioned) -> Result <Self , ::optionable::Error> {
                            Ok (value)
                        }

                        fn merge (&mut self , other: Self::Optioned) -> Result <() , ::optionable::Error> {
                            *self = other;
                            Ok(())
                        }
                    }
                },
            },
        ];
        for tc in tcs {
            let input = syn::parse2(tc.input).unwrap();
            let output = derive_optionable(input, None).unwrap();
            println!("{output}");
            assert_eq!(tc.output.to_string(), output.to_string());
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    /// Tests of the crate replacement settings available via codegen settings only
    fn test_crate_replacement() {
        let tcs = vec![TestCase {
            input: quote! {
                #[derive(Optionable)]
                struct DeriveExample {
                    name: crate::Name,
                    pub surname: Box<crate::SurName>,
                }
            },
            output: quote! {
                struct DeriveExampleOpt {
                    name: Option< <::testcrate::Name as crate::Optionable>::Optioned>,
                    pub surname: Option< <Box<::testcrate::SurName> as crate::Optionable>::Optioned>
                }

                #[automatically_derived]
                impl crate::Optionable for crate_prefix::DeriveExample {
                    type Optioned = DeriveExampleOpt;
                }

                #[automatically_derived]
                impl crate::Optionable for DeriveExampleOpt {
                    type Optioned = DeriveExampleOpt;
                }

                #[automatically_derived]
                impl crate::OptionableConvert for crate_prefix::DeriveExample {
                    fn into_optioned (self) -> DeriveExampleOpt {
                        DeriveExampleOpt  {
                            name: Some(crate::OptionableConvert::into_optioned(self.name)),
                            surname: Some(crate::OptionableConvert::into_optioned(self.surname))
                        }
                    }

                    fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, crate::Error> {
                        Ok(Self{
                            name: crate::OptionableConvert::try_from_optioned(value.name.ok_or(crate::Error { missing_field: "name" })?)?,
                            surname: crate::OptionableConvert::try_from_optioned(value.surname.ok_or(crate::Error { missing_field: "surname" })?)?
                        })
                    }

                    fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), crate::Error> {
                        if let Some(other_value) = other.name {
                             crate::OptionableConvert::merge(&mut self.name, other_value)?;
                        }
                        if let Some(other_value) = other.surname {
                             crate::OptionableConvert::merge(&mut self.surname, other_value)?;
                        }
                        Ok(())
                    }
                }

                #[automatically_derived]
                impl crate::OptionedConvert<crate_prefix::DeriveExample> for DeriveExampleOpt {
                    fn from_optionable(value: crate_prefix::DeriveExample) -> Self {
                        crate::OptionableConvert::into_optioned(value)
                    }

                    fn try_into_optionable(self) -> Result<crate_prefix::DeriveExample, crate::Error> {
                        crate::OptionableConvert::try_from_optioned(self)
                    }

                    fn merge_into(self, other: &mut crate_prefix::DeriveExample) -> Result<(), crate::Error> {
                       crate::OptionableConvert::merge(other, self)
                    }
                }
            },
        }];
        for tc in tcs {
            let input = syn::parse2(tc.input).unwrap();
            let output = derive_optionable(
                input,
                Some(&CodegenSettings {
                    ty_prefix: Some(Path::from_string("crate_prefix").unwrap()),
                    optionable_crate_name: Path::from_string("crate").unwrap(),
                    input_crate_replacement: Some(parse_quote!(testcrate)),
                }),
            )
            .unwrap();
            assert_eq!(tc.output.to_string(), output.to_string());
        }
    }
}
