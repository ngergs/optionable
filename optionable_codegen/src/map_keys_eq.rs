use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{AttrStyle, Attribute, Data, DeriveInput};

use crate::{CodegenSettings, helper::error};

const HELPER_ATTR_MAP_KEY_IDENT: &str = "map_key";

/// Derives the `OptionedMapKeysEq` implementation. All fields annotated with `#[map_key]` have to implement `PartialEq`.
/// The `OptionedMapKeysEq` impl will return true if all those fields are equal according to `PartialEq`.
///
/// # Errors
/// - If the annotated type is not a struct.
/// - If no fields have the `#[map_key]` helper attribute.
pub fn derive_optionable_map_keys_eq(
    input: DeriveInput,
    settings: Option<&CodegenSettings>,
) -> syn::Result<TokenStream> {
    let settings = settings.map(Cow::Borrowed).unwrap_or_default();
    let crate_name = &settings.optionable_crate_name;
    let ty_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Data::Struct(data) = input.data else {
        return error("The `MapKeysEq` derive macro only supports struct types.");
    };
    let key_field_selectors = match data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .into_iter()
            .filter_map(|f| field_is_map_key(&f.attrs).then_some(f.ident.to_token_stream()))
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .filter_map(|(i, f)| field_is_map_key(&f.attrs).then_some(i.to_token_stream()))
            .collect::<Vec<_>>(),
        syn::Fields::Unit => {
            return error(
                "The `MapKeysEq` derive macro only supports struct types (not unit types)",
            );
        }
    };
    if key_field_selectors.is_empty() {
        return error("At least one fields needs to have the `#[map_key]` helper attribute.");
    }
    let comparisons = key_field_selectors
        .iter()
        .map(|sel| quote! {self.#sel == other.#sel});
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #crate_name::merge::OptionableMapKeysEq for #ty_ident #ty_generics #where_clause {
            fn keys_eq(&self, other: &<Self as #crate_name::Optionable>::Optioned) -> bool{
                #(#comparisons)&&*
            }
        }
    })
}

/// Returns whether a field is selected as a map key by having the `#[map_key]` helper attribute.
fn field_is_map_key(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if matches!(attr.style, AttrStyle::Inner(_)) {
            return false;
        }
        attr.path().is_ident(HELPER_ATTR_MAP_KEY_IDENT)
    })
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::derive_optionable_map_keys_eq;

    #[test]
    fn optioned_map_keys_eq() {
        let input = quote! {
            #[derive(OptionableMapKeysEq)]
            struct EnvVar{
                #[map_key]
                name: String,
                #[map_key]
                id: i32,
                value: String,
            }
        };
        let parsed = syn::parse2(input).unwrap();
        let output = derive_optionable_map_keys_eq(parsed, None).unwrap();
        assert_eq!(
            output.to_string(),
            quote! {
                #[automatically_derived]
                impl ::optionable::merge::OptionableMapKeysEq for EnvVar {
                    fn keys_eq(&self, other: &<Self as ::optionable::Optionable>::Optioned) -> bool{
                        self.name == other.name && self.id == other.id
                    }
                }
            }
            .to_string()
        );
    }
}
