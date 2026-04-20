use darling::{FromAttributes, FromDeriveInput, FromMeta};
use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{
    DataEnum, DataStruct, DeriveInput, Field, FieldsNamed, FieldsUnnamed, Ident, Variant,
    parse_quote, visit::Visit,
};

#[derive(FromDeriveInput)]
#[darling(attributes(deepmerge))]
/// The type helper attributes to derive `k8s_openapi::DeepMerge`.
struct TypeHelperAttributes {
    /// Crate name under which the `k8s_openapi` crate with the `DeepMerge` trait can be found.
    #[darling(default=|| parse_quote!{k8s_openapi})]
    crate_k8s_openapi: Ident,
    /// Crate name under which the `optionable` crate with our helper methods can be found.
    #[darling(default=|| parse_quote!{optionable})]
    crate_optionable: Ident,
}

#[derive(FromAttributes)]
#[darling(attributes(deepmerge))]
/// The field helper attributes to derive `k8s_openapi::DeepMerge`.
struct FieldHelperAttributes {
    /// Merge method to use to handle the given fieldi
    #[darling(default)]
    method: MergeBehavior,
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

/// Derives the `k8s_openapi::DeepMerge` trait.
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

/// Visits onyl Data entries itself and delegates to the `EnumVisitor` or `StructVisitor` to generate the `merge_from_fn_body`.
struct DataVisitor {
    /// The parsed type attributes from the original type.
    attr: TypeHelperAttributes,
    /// `Ident` of the type for which `DeepMerge` should be generated.
    ty_ident: Ident,
    /// Result, will be filled after `visit_data` has been called once.
    merge_from_fn_body: Result<TokenStream, syn::Error>,
}

/// Visits the wrapper of the enum, only generates the `match self {...}` envelope and delegates to `EnumVariantVisitor` for the individual variants.
struct EnumVisitor<'a> {
    /// Shared reference to general config from `DataVisitor`.
    data_vis: &'a DataVisitor,
    /// Result, will be filled after `visit_data_enum` has been called once.
    variant_comparisons: Result<TokenStream, syn::Error>,
}

/// Visits the individual variants and generates the match condition and the individual field comparisons.
/// Delegates for the field comparisons to `StructVisitor.`
struct EnumVariantVisitor<'a, F1, F2, F3>
where
    F1: Fn(&TokenStream) -> TokenStream,
    F2: Fn(&TokenStream) -> TokenStream,
    F3: Fn(&TokenStream) -> TokenStream,
{
    /// Wwill be called to generate field comparisons
    struct_vis: StructVisitor<'a, F1, F2, F3>,
    /// Destructuring for the self keyword
    self_destructure: TokenStream,
    /// Destructuring for the other variable
    other_destructure: TokenStream,
}

/// Generates field comparisons.
struct StructVisitor<'a, F1, F2, F3>
where
    F1: Fn(&TokenStream) -> TokenStream,
    F2: Fn(&TokenStream) -> TokenStream,
    F3: Fn(&TokenStream) -> TokenStream,
{
    /// Shared reference to general config from `DataVisitor`.
    data_vis: &'a DataVisitor,
    /// Receives the field identifier and produces the `self` field selector, e.g. `self.my_field` or `self_my_field`.
    self_field_selector_fn: F1,
    /// Receives the field identifier and produces the `self` field selector, e.g. `self.my_field` or `self_my_field`.
    self_field_assign_fn: F2,
    /// Receives the field identifier and produces the `other` field selector, e.g. `other.my_field` or `other_my_field`.
    other_field_selector_fn: F3,
    /// Result, will be filled after `visit_fields` has been called once.
    field_comparisons: Result<TokenStream, syn::Error>,
}

impl<'ast> Visit<'ast> for DataVisitor {
    fn visit_data_struct(&mut self, node: &'ast DataStruct) {
        let mut struct_vis = StructVisitor {
            data_vis: self,
            field_comparisons: Ok(TokenStream::new()),
            self_field_selector_fn: |field| quote!(&mut self.#field),
            self_field_assign_fn: |field| quote!(self.#field),
            other_field_selector_fn: |field| quote!(other.#field),
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
                field_comparisons: Ok(TokenStream::new()),
                self_field_selector_fn: |field| format_ident!("self_{field}").into_token_stream(),
                self_field_assign_fn: |field| format_ident!("self_{field}").into_token_stream(),
                other_field_selector_fn: |field| format_ident!("other_{field}").into_token_stream(),
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

impl<'ast, F1, F2, F3> Visit<'ast> for EnumVariantVisitor<'ast, F1, F2, F3>
where
    F1: Fn(&TokenStream) -> TokenStream,
    F2: Fn(&TokenStream) -> TokenStream,
    F3: Fn(&TokenStream) -> TokenStream,
{
    fn visit_fields_named(&mut self, fields: &'ast FieldsNamed) {
        self.struct_vis.visit_fields_named(fields);

        let mut self_destructure = TokenStream::new();
        let mut other_destructure = TokenStream::new();
        fields.named.iter().for_each(|field| {
            let field_ident = field.ident.to_token_stream();
            let self_ident = (self.struct_vis.self_field_selector_fn)(&field_ident);
            let other_ident = (self.struct_vis.other_field_selector_fn)(&field_ident);
            self_destructure.extend(quote!(#field_ident: #self_ident,));
            other_destructure.extend(quote!(#field_ident: #other_ident,));
        });
        self.self_destructure = quote!({#self_destructure});
        self.other_destructure = quote!({#other_destructure});
    }

    fn visit_fields_unnamed(&mut self, fields: &'ast FieldsUnnamed) {
        self.struct_vis.visit_fields_unnamed(fields);

        let mut self_destructure = TokenStream::new();
        let mut other_destructure = TokenStream::new();
        fields.unnamed.iter().enumerate().for_each(|(i, _)| {
            let i = Literal::usize_unsuffixed(i).into_token_stream();
            let self_ident = (self.struct_vis.self_field_selector_fn)(&i);
            let other_ident = (self.struct_vis.other_field_selector_fn)(&i);
            self_destructure.extend(quote!(#self_ident,));
            other_destructure.extend(quote!(#other_ident,));
        });
        self.self_destructure = quote!((#self_destructure));
        self.other_destructure = quote!((#other_destructure));
    }
}

impl<'ast, F1, F2, F3> Visit<'ast> for StructVisitor<'ast, F1, F2, F3>
where
    F1: Fn(&TokenStream) -> TokenStream,
    F2: Fn(&TokenStream) -> TokenStream,
    F3: Fn(&TokenStream) -> TokenStream,
{
    fn visit_fields_named(&mut self, fields: &'ast FieldsNamed) {
        self.field_comparisons = fields
            .named
            .iter()
            .map(|field| {
                let field_ident = field.ident.to_token_stream();
                field_comparison(
                    self.data_vis,
                    &(self.self_field_selector_fn)(&field_ident),
                    &(self.self_field_assign_fn)(&field_ident),
                    &(self.other_field_selector_fn)(&field_ident),
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
                let i = Literal::usize_unsuffixed(i).into_token_stream();
                field_comparison(
                    self.data_vis,
                    &(self.self_field_selector_fn)(&i),
                    &(self.self_field_assign_fn)(&i),
                    &(self.other_field_selector_fn)(&i),
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
    self_field_ident: &TokenStream,
    self_assign_ident: &TokenStream,
    other_field_ident: &TokenStream,
    field: &Field,
) -> syn::Result<TokenStream> {
    let attrs = FieldHelperAttributes::from_attributes(&field.attrs)?;
    Ok(match attrs.method {
        MergeBehavior::DeepMerge => {
            let crate_k8s_openapi = &data_vis.attr.crate_k8s_openapi;
            quote! {#crate_k8s_openapi::DeepMerge::merge_from(#self_field_ident, #other_field_ident);}
        }
        MergeBehavior::Atomic => {
            quote! {#self_assign_ident = #other_field_ident;}
        }
        MergeBehavior::AppendNotPresent => {
            let crate_optionable = &data_vis.attr.crate_optionable;
            quote! {#crate_optionable::merge::merge_append_not_present(#self_field_ident, #other_field_ident);}
        }
        MergeBehavior::IterMap => {
            let crate_optionable = &data_vis.attr.crate_optionable;
            quote! {#crate_optionable::k8s_openapi::merge::merge_map(#self_field_ident, #other_field_ident);}
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{derive_deepmerge, tests::normalize_token_str};
    use quote::quote;

    fn assert_deepmerge(input: proc_macro2::TokenStream, expected: proc_macro2::TokenStream) {
        let parsed = syn::parse2(input).unwrap();
        let output = derive_deepmerge(parsed).unwrap();
        assert_eq!(
            normalize_token_str(&expected.to_string()),
            normalize_token_str(&output.to_string())
        );
    }

    #[test]
    fn test_deepmerge_struct() {
        assert_deepmerge(
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
                        k8s_openapi::DeepMerge::merge_from(&mut self.name, other.name);
                        k8s_openapi::DeepMerge::merge_from(&mut self.surname, other.surname);
                    }
                }
            },
        );
    }
    #[test]
    fn test_deepmerge_enum() {
        assert_deepmerge(
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
                            DeepmergeEnum::Surname(self_0, ) => {
                                if let DeepmergeEnum::Surname(other_0, ) = other{
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
                            DeepmergeEnum::Tuple(self_0, self_1, ) => {
                                if let DeepmergeEnum::Tuple(other_0, other_1, ) = other{
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
