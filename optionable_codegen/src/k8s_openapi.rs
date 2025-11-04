use crate::parsed_input::FieldHandling::OptionedOnly;
use crate::parsed_input::{FieldParsed, StructParsed};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, ImplGenerics, Path, TypeGenerics, WhereClause};

/// Adjust the parsed struct for the `k8s_openapi::Metadata` requirements.
pub(crate) fn k8s_openapi_field_metadata_adjust(struct_parsed: &mut StructParsed) {
    struct_parsed
        .fields
        .iter_mut()
        .filter(|f| f.field.ident.as_ref().is_some_and(|f| *f == "metadata"))
        .for_each(|f| f.field.attrs.push(parse_quote!(#[optionable(required)])));
}

/// Adjust the parsed struct for the `k8s_openapi::Resource` requirements.
pub(crate) fn k8s_openapi_field_resource_adjust(
    struct_parsed: &mut StructParsed,
    crate_name: &Path,
    ty_ident_opt: &Ident,
    ty_generics: &TypeGenerics,
) {
    let mut serialize_fn_name = crate_name.to_token_stream().to_string();
    serialize_fn_name.push_str("::k8s_openapi::serialize_api_envelope");
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
pub(crate) fn l(
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
