use darling::FromDeriveInput;
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed, Ident, parse_quote, visit::Visit,
};

use crate::helper::error;

#[derive(FromDeriveInput)]
#[darling(attributes(map_keys_eq))]
/// The type helper attributes to derive `optionable::MapKeysEq`, only supports structs.
struct TypeHelperAttributes {
    #[darling(default=|| parse_quote!{optionable})]
    crate_optionable: Ident,
}

/// Derives the `MapKeysEq` trait.
///
/// # Errors
/// - If the field helper attributes are malformed.
pub fn derive_map_keys_eq(input: &DeriveInput) -> syn::Result<TokenStream> {
    let attr = TypeHelperAttributes::from_derive_input(input)?;
    let mut vis = DataVisitor {
        fn_body: Ok(TokenStream::new()),
    };
    let (impl_generics, ty_generics, where_generics) = input.generics.split_for_impl();
    vis.visit_data(&input.data);
    let comparisons = vis.fn_body?;

    let ty_ident = &input.ident;
    let crate_optionable = &attr.crate_optionable;
    Ok(
        quote! {impl #impl_generics #crate_optionable::merge::MapKeysEq for #ty_ident #ty_generics #where_generics{
            fn keys_eq(&self, other: &Self) -> bool {
                #comparisons
            }
        }},
    )
}

/// Visits onyl Data entries itself and delegates to the `EnumVisitor` or `StructVisitor` to generate the `merge_from_fn_body`.
struct DataVisitor {
    /// Result, will be filled after `visit_data` has been called once.
    fn_body: Result<TokenStream, syn::Error>,
}

impl<'ast> Visit<'ast> for DataVisitor {
    fn visit_data_enum(&mut self, _: &'ast DataEnum) {
        self.fn_body =
            error("unsupported type, only struct types are supported for `MapKeysEq` derive");
    }

    fn visit_data_union(&mut self, _: &'ast DataUnion) {
        self.fn_body =
            error("unsupported type, only struct types are supported for `MapKeysEq` derive");
    }

    fn visit_fields_named(&mut self, fields: &'ast FieldsNamed) {
        let key_fields = fields
            .named
            .iter()
            .filter(|field| {
                field
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().to_token_stream().to_string() == "map_key")
                    .count()
                    != 0
            })
            .collect::<Vec<_>>();
        if key_fields.is_empty() {
            self.fn_body = error(
                "At least one field has to have the `#[map_key]` attribute to derive `optionable::MapKeysEq`",
            );
            return;
        }
        let comparisons = key_fields.into_iter().map(|field| {
            let ident = &field.ident;
            quote! {self.#ident == other.#ident}
        });
        self.fn_body = Ok(quote! {#(#comparisons)&&*});
    }

    fn visit_fields_unnamed(&mut self, fields: &'ast FieldsUnnamed) {
        let key_fields = fields
            .unnamed
            .iter()
            .enumerate()
            .filter_map(|(i, field)| {
                (field
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().to_token_stream().to_string() == "map_key")
                    .count()
                    != 0)
                    .then_some(i)
            })
            .collect::<Vec<_>>();
        if key_fields.is_empty() {
            self.fn_body = error(
                "At least one field has to have the `#[map_key]` attribute to derive `optionable::MapKeysEq`",
            );
            return;
        }
        let comparisons = key_fields.into_iter().map(|i| {
            let i = Literal::usize_unsuffixed(i);
            quote! {self.#i == other.#i}
        });
        self.fn_body = Ok(quote! {#(#comparisons)&&*});
    }
}

#[cfg(test)]
mod tests {
    use crate::{map_keys_eq::derive_map_keys_eq, tests::normalize_token_str};
    use quote::quote;

    fn assert_map_keys_eq(input: proc_macro2::TokenStream, expected: proc_macro2::TokenStream) {
        let parsed = syn::parse2(input).unwrap();
        let output = derive_map_keys_eq(parsed).unwrap();
        assert_eq!(
            normalize_token_str(&expected.to_string()),
            normalize_token_str(&output.to_string())
        );
    }

    #[test]
    fn test_map_keys_eq_struct() {
        assert_map_keys_eq(
            quote! {
                #[derive(MapKeysEq)]
                struct EnvVar{
                    #[map_key]
                    key: String,
                    #[map_key]
                    pub key2: String,
                    val: String,
                }
            },
            quote! {
                impl optionable::merge::MapKeysEq for EnvVar{
                    fn keys_eq(&self, other: &Self) -> bool{
                        self.key == other.key && self.key2 == other.key2
                    }
                }
            },
        );
    }

    #[test]
    fn test_map_keys_eq_tuple() {
        assert_map_keys_eq(
            quote! {
                #[derive(MapKeysEq)]
                struct EnvVar(#[map_key] String, String, #[map_key] String);
            },
            quote! {
                impl optionable::merge::MapKeysEq for EnvVar{
                    fn keys_eq(&self, other: &Self) -> bool{
                        self.0 == other.0 && self.2 == other.2
                    }
                }
            },
        );
    }
}
