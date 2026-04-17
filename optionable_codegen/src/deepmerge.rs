use darling::{FromAttributes, FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Field, FieldsNamed, FieldsUnnamed, visit::Visit};

use crate::helper::error;

#[derive(FromDeriveInput)]
#[darling(attributes(deepmerge))]
/// The type helper attributes to derive `deepmerge`.
struct TypeHelperAttributes {
    /// Crate name under which the `k8s_openapi` crate with the `DeepMerge` trait can be found.
    #[darling(default=|| "k8s_openapi".to_owned())]
    crate_k8s_openapi: String,
    /// Crate name under which the `optionable` crate with our helper methods can be found.
    #[darling(default=|| "optionable".to_owned())]
    crate_optionable: String,
    /// Used k8s openapi version from optionable in the form of the available package path, e.g. `k8s_openapi027`
    optionable_k8s_openapi_package: String,
}

#[derive(FromAttributes)]
#[darling(attributes(deepmerge))]
/// The field helper attributes to derive `deepmerge`.
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

/// Derives the `DeepMerge` and (if corresponding `#[deepmerge(map_key)]` field attributes are present) the `MapKeysEq` traits.
///
/// # Errors
/// - If the field helper attributes are malformed.
pub fn derive_deepmerge(input: &DeriveInput) -> syn::Result<TokenStream> {
    let mut vis = DeepmergeVisitor {
        attr: TypeHelperAttributes::from_derive_input(input)?,
        result: Ok(Vec::new()),
    };
    vis.visit_data(&input.data);
    let result = vis.result?;
    Ok(quote!(#(#result)*))
}

struct DeepmergeVisitor {
    attr: TypeHelperAttributes,
    result: Result<Vec<TokenStream>, syn::Error>,
}

impl<'ast> Visit<'ast> for DeepmergeVisitor {
    fn visit_fields_named(&mut self, fields: &'ast FieldsNamed) {
        fields.named.iter().for_each(|field| {
            let ident = field.ident.as_ref().unwrap(); // we are at named fields
            self.visit_field(&ident.to_token_stream(), field);
        });
    }

    fn visit_fields_unnamed(&mut self, fields: &'ast FieldsUnnamed) {
        fields.unnamed.iter().enumerate().for_each(|(i, field)| {
            self.visit_field(&i.to_token_stream(), field);
        });
    }
}

impl DeepmergeVisitor {
    fn visit_field(&mut self, ident: &TokenStream, field: &Field) {
        let Ok(result) = &mut self.result else {
            return;
        };
        let attrs = match FieldHelperAttributes::from_attributes(&field.attrs) {
            Ok(attrs) => attrs,
            Err(err) => {
                self.result = Err(err.into());
                return;
            }
        };
        let comparison = match attrs.method {
            MergeBehavior::DeepMerge => {
                let crate_k8s_openapi = &self.attr.crate_k8s_openapi;
                quote! {#crate_k8s_openapi::DeepMerge::merge_from(self.#ident, other.#ident);}
            }
            MergeBehavior::Atomic => {
                quote! {self.#ident = other.#ident;}
            }
            MergeBehavior::AppendNotPresent => {
                let crate_optionable = &self.attr.crate_optionable;
                quote! {#crate_optionable::merge::merge_append_not_present(self.#ident, other.#ident);}
            }
            MergeBehavior::IterMap => {
                let crate_optionable = &self.attr.crate_optionable;
                let k8s_openapi_package = &self.attr.optionable_k8s_openapi_package;
                quote! {#crate_optionable::#k8s_openapi_package::merge::merge_map(self.#ident, other.#ident);}
            }
        };
        result.push(comparison);
    }
}
