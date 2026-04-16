use darling::{FromAttributes, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field};

use crate::helper::error;

#[derive(FromAttributes)]
#[darling(attributes(merge))]
/// The helper attributes to derive `deepmerge`.
struct FieldHelperAttributes {
    /// Merge method to use to handle the given field
    method: MergeBehavior,
    /// Set if the respective field is a map key
    map_key: Option<()>,
}

#[derive(FromMeta, Default)]
/// The supported behavior to merge specific fields
enum MergeBehavior {
    #[default]
    /// Default behavior, call `DeepMerge` for the given field
    DeepMerge,
    /// Completly overrides the field with the merge candidate (no recursive deep merging happens anymore from here)
    Atomic,
    // Appends entries not already present in the target, requires the corresponding field to be `impl Extent<T>+IntoIter<Item=T> where T: PartialEq`
    AppendNotPresent,
    // Merges entries that are already present in the target using `OptionableConvert`, appends those that are not.
    // Requires the correspond field to be `impl Extend<T>+IntoIter<Item=T> where T: MapKeysEq+OptionableConvert`
    IterMap,
}

pub fn derive_deepmerge(input: DeriveInput) -> syn::Result<TokenStream> {
    let result: Vec<TokenStream> = match input.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    // todo: handle different cases
                    fields_named
                        .named
                        .into_iter()
                        .map(|Field { ident, .. }| quote!(self.#ident = other.#ident))
                        .collect()
                }
                syn::Fields::Unnamed(fields_unnamed) => fields_unnamed
                    .unnamed
                    .into_iter()
                    .enumerate()
                    .map(|(i, _)| quote!(self.#i = other.#i))
                    .collect(),
                // nothing to do
                syn::Fields::Unit => return Ok(quote!()),
            }
        }
        Data::Enum(data_enum) => return error("todo"),
        Data::Union(_) => return error("#[derive(DeepMerge)] not supported for unit structs"),
    };
    Ok(quote!(#(#result);*))
}
