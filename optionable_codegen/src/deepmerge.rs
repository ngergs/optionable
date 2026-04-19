use darling::{FromAttributes, FromDeriveInput, FromMeta};
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{
    DataEnum, DataStruct, DeriveInput, Field, FieldsNamed, FieldsUnnamed, Ident, Variant,
    parse_quote, visit::Visit,
};

use crate::helper::error;

#[derive(FromDeriveInput)]
#[darling(attributes(deepmerge))]
/// The type helper attributes to derive `deepmerge`.
struct TypeHelperAttributes {
    /// Crate name under which the `k8s_openapi` crate with the `DeepMerge` trait can be found.
    #[darling(default=|| parse_quote!{k8s_openapi})]
    crate_k8s_openapi: Ident,
    /// Crate name under which the `optionable` crate with our helper methods can be found.
    #[darling(default=|| parse_quote!{optionable})]
    crate_optionable: Ident,
    /// Used k8s openapi version from optionable in the form of the available package path, e.g. `k8s_openapi027`
    k8s_openapi_package: Option<String>,
}

#[derive(FromAttributes)]
#[darling(attributes(deepmerge))]
/// The field helper attributes to derive `deepmerge`.
struct FieldHelperAttributes {
    /// Merge method to use to handle the given fieldi
    #[darling(default)]
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
pub fn derive_deepmerge(input: DeriveInput) -> syn::Result<TokenStream> {
    let mut vis = DataVisitor {
        attr: TypeHelperAttributes::from_derive_input(&input)?,
        ty_ident: input.ident,
        merge_from_fn_body: Ok(TokenStream::new()),
    };
    let (impl_generics, ty_generics, where_generics) = input.generics.split_for_impl();
    vis.visit_data(&input.data);
    let comparisons = vis.merge_from_fn_body?;

    let ident = vis.ty_ident;
    let crate_k8s_openapi = &vis.attr.crate_k8s_openapi;
    Ok(
        quote! {impl #impl_generics #crate_k8s_openapi::DeepMerge for #ident #ty_generics #where_generics{
            fn merge_from(&mut self, other: Self){
                #comparisons
            }
        }},
    )
}

// todo split into multiple visitors for different use cases
struct DataVisitor {
    attr: TypeHelperAttributes,
    ty_ident: Ident,
    merge_from_fn_body: Result<TokenStream, syn::Error>,
}

struct StructVisitor<'a> {
    data_vis: &'a DataVisitor,
    self_ident: &'a TokenStream,
    field_comparisons: Result<TokenStream, syn::Error>,
}

struct EnumVariantVisitor<'a> {
    // will be called to generate field comparisons so must be mut
    struct_vis: StructVisitor<'a>,
    self_destructure: TokenStream,
    other_destructure: TokenStream,
}

struct EnumVisitor<'a> {
    data_vis: &'a DataVisitor,
    variant_comparisons: Result<TokenStream, syn::Error>,
}

impl<'ast> Visit<'ast> for DataVisitor {
    fn visit_data_struct(&mut self, node: &'ast DataStruct) {
        let mut struct_vis = StructVisitor {
            data_vis: self,
            self_ident: &quote!(self),
            field_comparisons: Ok(TokenStream::new()),
        };
        struct_vis.visit_data_struct(node);
        self.merge_from_fn_body = struct_vis.field_comparisons;
    }

    fn visit_data_enum(&mut self, node: &'ast DataEnum) {
        let mut enum_vis = EnumVisitor {
            data_vis: self,
            variant_comparisons: Ok(TokenStream::new()),
        };
        enum_vis.visit_data_enum(node);
        self.merge_from_fn_body = enum_vis.variant_comparisons.map(|comparisons| {
            quote! {
                match self{
                    #comparisons
                }
            }
        });
    }
}

impl<'ast> Visit<'ast> for EnumVisitor<'ast> {
    fn visit_variant(&mut self, variant: &'ast Variant) {
        let Ok(variant_comparisons) = &mut self.variant_comparisons else {
            return;
        };

        let mut enum_variant_visitor = EnumVariantVisitor {
            struct_vis: StructVisitor {
                data_vis: self.data_vis,
                self_ident: &quote!(self),
                field_comparisons: Ok(TokenStream::new()),
            },
            self_destructure: TokenStream::new(),
            other_destructure: TokenStream::new(),
        };
        enum_variant_visitor.visit_fields(&variant.fields);

        let ty_ident = &self.data_vis.ty_ident;
        let variant_ident = &variant.ident;
        let variant_self_destructure = &enum_variant_visitor.self_destructure;
        let variant_other_destructure = &enum_variant_visitor.other_destructure;
        let comparison = enum_variant_visitor
            .struct_vis
            .field_comparisons
            .map(|comparisons| {
                quote!(#ty_ident::#variant_ident #variant_self_destructure=> {
                    if let #ty_ident::#variant_ident #variant_other_destructure = other {
                        #comparisons
                    } else {
                        *self = other;
                    }
                })
            });
        match comparison {
            Ok(c) => variant_comparisons.extend(c),
            Err(err) => self.variant_comparisons = Err(err),
        }
    }
}

impl<'ast> Visit<'ast> for EnumVariantVisitor<'ast> {
    fn visit_fields_named(&mut self, fields: &'ast FieldsNamed) {
        self.struct_vis.visit_fields_named(fields);

        let mut self_destructure = TokenStream::new();
        let mut other_destructure = TokenStream::new();
        fields.named.iter().for_each(|field| {
            let ident = field.ident.as_ref().unwrap(); // we are at named fields
            let self_ident = format_ident!("self_{ident}");
            let other_ident = format_ident!("other_{ident}");
            self_destructure.extend(quote!(#ident: #self_ident,));
            other_destructure.extend(quote!(#ident: #other_ident,));
        });
        self.self_destructure = quote!({#self_destructure});
        self.other_destructure = quote!({#other_destructure});
    }

    fn visit_fields_unnamed(&mut self, fields: &'ast FieldsUnnamed) {
        self.struct_vis.visit_fields_unnamed(fields);

        let mut self_destructure = TokenStream::new();
        let mut other_destructure = TokenStream::new();
        fields.unnamed.iter().enumerate().for_each(|(i, _)| {
            let i = Literal::usize_unsuffixed(i);
            let self_ident = format_ident!("self_{i}");
            let other_ident = format_ident!("other_{i}");
            self_destructure.extend(quote!(#i: #self_ident,));
            other_destructure.extend(quote!(#i: #other_ident,));
        });
        self.self_destructure = quote!((#self_destructure));
        self.other_destructure = quote!((#other_destructure));
    }
}

impl<'ast> Visit<'ast> for StructVisitor<'ast> {
    fn visit_fields_named(&mut self, fields: &'ast FieldsNamed) {
        self.field_comparisons = fields
            .named
            .iter()
            .map(|field| {
                field_comparison(
                    self.data_vis,
                    self.self_ident,
                    &field.ident.as_ref().unwrap().into_token_stream(), //unwrap is ok we are at named fields here
                    field,
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|comparisons| comparisons.into_iter().collect());
    }

    fn visit_fields_unnamed(&mut self, fields: &'ast FieldsUnnamed) {
        self.field_comparisons = fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, field)| {
                field_comparison(
                    self.data_vis,
                    self.self_ident,
                    &Literal::usize_unsuffixed(i).into_token_stream(),
                    field,
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|comparisons| comparisons.into_iter().collect());
    }
}

/// Generates the field comparisons for the given field
fn field_comparison(
    data_vis: &DataVisitor,
    self_ident: &TokenStream,
    field_ident: &TokenStream,
    field: &Field,
) -> syn::Result<TokenStream> {
    let attrs = FieldHelperAttributes::from_attributes(&field.attrs)?;
    Ok(match attrs.method {
        MergeBehavior::DeepMerge => {
            let crate_k8s_openapi = &data_vis.attr.crate_k8s_openapi;
            quote! {#crate_k8s_openapi::DeepMerge::merge_from(#self_ident.#field_ident, other.#field_ident);}
        }
        MergeBehavior::Atomic => {
            quote! {#self_ident.#field_ident = other.#field_ident;}
        }
        MergeBehavior::AppendNotPresent => {
            let crate_optionable = &data_vis.attr.crate_optionable;
            quote! {#crate_optionable::merge::merge_append_not_present(#self_ident.#field_ident, other.#field_ident);}
        }
        MergeBehavior::IterMap => {
            let crate_optionable = &data_vis.attr.crate_optionable;
            let Some(k8s_openapi_package) = &data_vis.attr.k8s_openapi_package else {
                return error(
                    "The `k8sopenapi_package` helper attribute is required for usage of `#[deepmerge(itermap)]`, set e.g. `#[deepmerge(k8sopenapi_package(k8s_openapi027))]` if that's the version you use",
                );
            };
            quote! {#crate_optionable::#k8s_openapi_package::merge::merge_map(#self_ident.#field_ident, other.#field_ident);}
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{derive_deepmerge, tests::normalize_token_str};
    use quote::quote;

    fn assert_odeepmerge(input: proc_macro2::TokenStream, expected: proc_macro2::TokenStream) {
        let parsed = syn::parse2(input).unwrap();
        let output = derive_deepmerge(parsed).unwrap();
        assert_eq!(
            normalize_token_str(&expected.to_string()),
            normalize_token_str(&output.to_string())
        );
    }

    #[test]
    fn test_deepmerge_struct() {
        assert_odeepmerge(
            quote! {
                #[derive(DeepMerge)]
                struct DeepmergeStruct{
                    name: String,
                    pub surname: String,
                }
            },
            quote! {
                impl k8s_openapi::DeepMerge for DeepmergeStruct{
                    fn merge_from(&mut self, other: Self){
                        k8s_openapi::DeepMerge::merge_from(self.name, other.name);
                        k8s_openapi::DeepMerge::merge_from(self.surname, other.surname);
                    }
                }
            },
        );
    }
    #[test]
    fn test_deepmerge_enum() {
        assert_odeepmerge(
            quote! {
                #[derive(DeepMerge)]
                enum DeepmergeEnum{
                    Surname(String),
                    Name{name: String, surname: String},
                    Tuple(String, String),
                }
            },
            quote! {
                impl k8s_openapi::DeepMerge for DeepmergeEnum{
                    fn merge_from(&mut self, other: Self){
                        match self{
                            DeepmergeEnum::Surname(0: self_0, ) => {
                                if let DeepmergeEnum::Surname(0: other_0, ) = other{
                                    k8s_openapi::DeepMerge::merge_from (self_0, other_0);
                                } else {
                                    *self = other;
                                }
                            }
                            DeepmergeEnum::Name{name: self_name, surname: self_surname, }=> {
                                if let DeepmergeEnum::Name{name: other_name, surname: other_surname, } = other{
                                    k8s_openapi::DeepMerge::merge_from (self_name, other_name);
                                    k8s_openapi::DeepMerge::merge_from (self_surname, other_surname);
                                } else {
                                    *self = other;
                                }
                            }
                            DeepmergeEnum::Tuple(0: self_0, 1: self_1, ) => {
                                if let DeepmergeEnum::Tuple(0: self_0, 1: self_1, ) = other{
                                    k8s_openapi::DeepMerge::merge_from (self_0, other_0);
                                    k8s_openapi::DeepMerge::merge_from (self_1, other_1);
                                } else {
                                    *self = other;
                                }
                            }
                        }
                    }
                }
            },
        );
    }
}
