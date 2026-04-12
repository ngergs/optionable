use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::parsed_input::{FieldHandling, FieldParsed};

/// Returns the `OptionableMapKeysEq` impl if at least one of the struct fields has an `#[optionable_map_key]` attribute, else returns `None`.
pub(crate) fn map_keys_eq_impl(fields: &[FieldParsed]) -> Option<TokenStream> {
    let key_field_selectors = fields
        .iter()
        .filter(|f| matches!(f.handling, FieldHandling::MapKey))
        .map(|f| f.field.ident.to_token_stream())
        .collect::<Vec<_>>();
    if key_field_selectors.is_empty() {
        return None;
    }
    let comparisons = key_field_selectors
        .into_iter()
        .map(|sel| quote! {self.#sel == other.#sel});
    Some(quote!(#(#comparisons)&&*))
}
