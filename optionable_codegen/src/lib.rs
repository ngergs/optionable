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
use crate::FieldHandling::{IsOption, Other, Required};
use darling::util::PathList;
use darling::{FromAttributes, FromDeriveInput};
use itertools::MultiUnzip;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::cmp::PartialEq;
use std::default::Default;
use std::{fmt, iter};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Where;
use syn::{
    parse_quote, Attribute, Data, DeriveInput, Error, Field, Fields, GenericParam, LitStr, Meta, Path,
    Token, Type, TypePath, WhereClause, WherePredicate,
};

const HELPER_IDENT: &str = "optionable";
const HELPER_ATTR_IDENT: &str = "optionable_attr";
const ERR_MSG_HELPER_ATTR_ENUM_VARIANTS: &str =
    "#[optionable] helper attributes not supported on enum variant level.";

#[derive(FromDeriveInput)]
#[darling(attributes(optionable))]
/// Helper attributes on the type definition level (attached to the `struct` or `enum` itself).
struct TypeHelperAttributes {
    /// Derive-macros that should be added to the optioned type
    derive: Option<PathList>,
    #[darling(default=default_suffix)]
    /// Explicit suffix to use for the optioned type.
    suffix: LitStr,
    /// Skip generating `OptionableConvert` impl
    no_convert: Option<()>,
}

#[derive(FromAttributes)]
#[darling(attributes(optionable))]
/// Helper attributes on the type definition level (attached to the `struct` or `enum` itself).
struct FieldHelperAttributes {
    /// Given field won't be optioned, it will also be required for the derived optioned type.
    required: Option<()>,
}

fn default_suffix() -> LitStr {
    LitStr::new("Opt", Span::call_site())
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
pub fn derive_optionable(input: DeriveInput) -> syn::Result<TokenStream> {
    let attrs = TypeHelperAttributes::from_derive_input(&input)?;
    let forward_attrs = forwarded_attributes(&input.attrs);
    let vis = input.vis;
    let type_ident_opt = Ident::new(
        &(input.ident.to_string() + &attrs.suffix.value()),
        input.ident.span(),
    );
    let type_ident = &input.ident;

    let generics_colon = (!input.generics.params.is_empty()).then(|| quote! {::});
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let mut where_clause = input
        .generics
        .where_clause
        .clone()
        .unwrap_or_else(|| WhereClause {
            where_token: Where::default(),
            predicates: Punctuated::default(),
        });
    let where_clause_convert = attrs.no_convert.is_none().then(|| {
        let mut where_clause = where_clause.clone();

        patch_where_clause_bounds(&mut where_clause, &input.generics.params, |_| {
            quote!(::optionable::OptionableConvert)
        });
        where_clause
    });
    patch_where_clause_bounds(&mut where_clause, &input.generics.params, |_| {
        quote!(::optionable::Optionable)
    });

    // the impl statements are actually independent of deriving
    // the relevant associated type #type_ident_opt referenced by them
    let impls_optionable = quote! {
        #[automatically_derived]
        impl #impl_generics ::optionable::Optionable for #type_ident #ty_generics #where_clause {
            type Optioned = #type_ident_opt #ty_generics;
        }

        #[automatically_derived]
        impl #impl_generics ::optionable::Optionable for #type_ident_opt #ty_generics #where_clause {
            type Optioned = #type_ident_opt #ty_generics;
        }
    };

    // now we have to derive the actual implementation of #type_ident_opt
    // and add the #impl from above
    let derives = attrs.derive.unwrap_or_default();
    let skip_optionable_if_serde_serialize = derives
        .iter()
        .any(is_serialize)
        .then(|| quote!(#[serde(skip_serializing_if = "Option::is_none")]));
    let derives = (!derives
        .iter()
        .map(ToTokens::to_token_stream)
        .collect::<Vec<_>>()
        .is_empty())
    .then(|| quote! {#[derive(#(#derives),*)]});

    match input.data {
        Data::Struct(s) => {
            let fields = into_field_handling(s.fields)?;
            let unnamed_struct_semicolon =
                (fields.struct_type == StructType::Unnamed).then(|| quote!(;));
            let optioned_fields =
                optioned_fields(&fields, skip_optionable_if_serde_serialize.as_ref());

            let impl_optionable_convert = attrs.no_convert.is_none().then(|| {
                let into_optioned_fields = into_optioned(&fields, |selector| quote! { self.#selector });
                let try_from_optioned_fields =
                    try_from_optioned(&fields, |selector| quote! { value.#selector });
                let merge_fields = merge_fields(
                    &fields,
                    |selector| quote! { self.#selector },
                    |selector| quote! { other.#selector },
                    true);
                quote! {
                    #[automatically_derived]
                    impl #impl_generics ::optionable::OptionableConvert for #type_ident #ty_generics #where_clause_convert {
                        fn into_optioned(self) -> #type_ident_opt #ty_generics {
                            #type_ident_opt #generics_colon #ty_generics #into_optioned_fields
                        }

                        fn try_from_optioned(value: #type_ident_opt #ty_generics) -> Result<Self,::optionable::optionable::Error>{
                            Ok(Self #try_from_optioned_fields)
                        }

                        fn merge(&mut self, other: #type_ident_opt #ty_generics) -> Result<(), ::optionable::optionable::Error>{
                            #merge_fields
                            Ok(())
                        }
                    }
                }
            });

            Ok(quote! {
                #[automatically_derived]
                #derives
                #forward_attrs
                #vis struct #type_ident_opt #impl_generics #where_clause #optioned_fields #unnamed_struct_semicolon

                #impls_optionable

                #impl_optionable_convert
            })
        }
        Data::Enum(e) => {
            let self_prefix = quote! {self_};
            let other_prefix = quote! {other_};

            let variants = e
                .variants
                .into_iter()
                .map(|v| {
                    error_on_helper_attributes(&v.attrs, ERR_MSG_HELPER_ATTR_ENUM_VARIANTS)?;
                    Ok::<_, Error>((
                        v.ident,
                        forwarded_attributes(&v.attrs),
                        into_field_handling(v.fields)?,
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?;

            let optioned_variants = variants.iter().map(|(variant, forward_attrs, f)| {
                let fields = optioned_fields(f, skip_optionable_if_serde_serialize.as_ref());
                quote!( #forward_attrs #variant #fields )
            });

            let impl_optionable_convert = attrs.no_convert.is_none().then(|| {
                let (into_variants, try_from_variants, merge_variants): (Vec<_>, Vec<_>, Vec<_>) = variants
                    .iter()
                    .map(|(variant,_, f)| {
                        let fields_into = into_optioned(f, |selector| {
                            format_ident!("{self_prefix}{selector}").to_token_stream()
                        });
                        let fields_try_from = try_from_optioned(f, |selector| {
                            format_ident!("{other_prefix}{selector}").to_token_stream()
                        });
                        let fields_merge = merge_fields(f,
                                                        |selector| format_ident!("{self_prefix}{selector}").to_token_stream(),
                                                        |selector| format_ident!("{other_prefix}{selector}").to_token_stream(),
                                                        false);
                        let self_destructure = destructure(f, &self_prefix)?;
                        let other_destructure = destructure(f, &other_prefix)?;
                        Ok::<_, Error>((
                            quote!( Self::#variant #self_destructure => #type_ident_opt::#variant #fields_into ),
                            quote!( #type_ident_opt::#variant #other_destructure => Self::#variant #fields_try_from ),
                            quote!( #type_ident_opt::#variant #other_destructure => {
                                if let Self::#variant #self_destructure = self {
                                    #fields_merge
                                } else {
                                    *self = Self::try_from_optioned(#type_ident_opt::#variant #other_destructure)?;
                                }
                            })
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter().multiunzip();
                Ok::<_, Error>(quote! {
                    #[automatically_derived]
                    impl #impl_generics ::optionable::OptionableConvert for #type_ident #ty_generics #where_clause_convert {
                        fn into_optioned(self) -> #type_ident_opt #ty_generics {
                            match self {
                                #(#into_variants),*
                            }
                        }

                        fn try_from_optioned(other: #type_ident_opt #ty_generics)->Result<Self,::optionable::optionable::Error>{
                            Ok(match other{
                                #(#try_from_variants),*
                            })
                        }

                        fn merge(&mut self, other: #type_ident_opt #ty_generics) -> Result<(), ::optionable::optionable::Error>{
                            match other{
                                #(#merge_variants),*
                            }
                            Ok(())
                        }
                    }
                })
            }).transpose()?;

            Ok(quote!(
                #[automatically_derived]
                #derives
                #forward_attrs
                #vis enum #type_ident_opt #impl_generics #where_clause {
                    #(#optioned_variants),*
                }

                #impls_optionable

                #impl_optionable_convert
            ))
        }
        Data::Union(_) => error("#[derive(Optionable)] not supported for unit structs"),
    }
}

/// Constructs a destructure selector for the given fields, e.g. a `std::vec!(field_a, field_b)`
/// for a named enum or `std::vec!(0,1)` for an unnamed enum.
fn destructure(fields: &FieldsParsed, prefix: &TokenStream) -> Result<TokenStream, Error> {
    Ok(match fields.struct_type {
        StructType::Named => {
            let fields = fields
                .fields
                .iter()
                .map(|f| {
                    let ident = f.field.ident.as_ref().ok_or::<Error>(
                        error::<_, Error>(format!(
                            "expected field name but none present for {f:?}"
                        ))
                        .unwrap_err(),
                    )?;
                    let prefixed_ident = format_ident!("{1}{0}", ident.clone(), prefix.to_string());
                    Ok::<_, Error>(quote! {#ident: #prefixed_ident})
                })
                .collect::<Result<Vec<_>, _>>()?;
            quote! {{#(#fields),*}}
        }
        StructType::Unnamed => {
            let indices: Vec<_> = (0..fields.fields.len())
                .map(|i| {
                    let prefixed = format_ident!("{1}{0}", i, prefix.to_string());
                    quote! {#prefixed}
                })
                .collect();
            quote! {(#(#indices),*)}
        }
        StructType::Unit => quote! {},
    })
}

/// Returns a tokenstream for the optioned fields and potential convert implementation of the optioned object (struct/enum variants).
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
fn optioned_fields(fields: &FieldsParsed, serde_attributes: Option<&TokenStream>) -> TokenStream {
    let fields_token = fields.fields.iter().map(
        |FieldParsed {
             field: Field { attrs,vis, ident, ty, .. },
             handling,
         }| {
            let forward_attrs = forwarded_attributes(attrs);
            let optioned_ty = optioned_ty(ty);
            let colon = ident.as_ref().map(|_| quote! {:});
            match handling {
                Required => quote! {#forward_attrs #vis #ident #colon #ty},
                IsOption => quote! {#forward_attrs #serde_attributes #vis #ident #colon #optioned_ty},
                Other => quote! {#forward_attrs #serde_attributes #vis #ident #colon Option<#optioned_ty>},
            }
        },
    );
    struct_wrapper(fields_token, &fields.struct_type)
}

/// Returns the field mapping implementation for `into_optioned`.
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
fn into_optioned(
    fields: &FieldsParsed,
    self_selector_fn: impl Fn(&TokenStream) -> TokenStream,
) -> TokenStream {
    let fields_token = fields.fields.iter().enumerate().map(|(i, FieldParsed { field: Field { ident, ty, .. }, handling })| {
        let colon = ident.as_ref().map(|_| quote! {:});
        let selector = ident.as_ref().map_or_else(|| {
            let i = Literal::usize_unsuffixed(i);
            quote! {#i}
        }, ToTokens::to_token_stream);
        let self_selector = self_selector_fn(&selector);
        match (handling, is_self_resolving_optioned(ty)) {
            (Required, _) | (IsOption, true) => quote! {#ident #colon #self_selector},
            (IsOption, false) => quote! {#ident #colon <#ty as ::optionable::OptionableConvert>::into_optioned(#self_selector)},
            (Other, true) => quote! {#ident #colon Some(#self_selector)},
            (Other, false) => quote! {#ident #colon Some(<#ty as ::optionable::OptionableConvert>::into_optioned(#self_selector))}
        }
    });
    struct_wrapper(fields_token, &fields.struct_type)
}

/// Returns the field-mappings implementation for `try_from_optioned`.
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
fn try_from_optioned(
    fields: &FieldsParsed,
    value_selector_fn: impl Fn(&TokenStream) -> TokenStream,
) -> TokenStream {
    let fields_token = fields.fields.iter().enumerate().map(|(i, FieldParsed { field: Field { ident, ty, .. }, handling })| {
        let colon = ident.as_ref().map(|_| quote! {:});
        let selector = ident.as_ref().map_or_else(|| {
            let i = Literal::usize_unsuffixed(i);
            quote! {#i}
        }, ToTokens::to_token_stream);
        let value_selector = value_selector_fn(&selector);
        match (handling, is_self_resolving_optioned(ty)) {
            (Required, _) | (IsOption, true) => quote! {#ident #colon value.#selector},
            (IsOption, false) => quote! {
                #ident #colon <#ty as ::optionable::OptionableConvert>::try_from_optioned(
                    #value_selector
                )?
            },
            (Other, true) => {
                let selector_quoted = LitStr::new(&selector.to_string(), ident.span());
                quote! {
                    #ident #colon #value_selector.ok_or(optionable::optionable::Error{ missing_fields: std::vec![#selector_quoted] })?
                }
            }
            (Other, false) => {
                let selector_quoted = LitStr::new(&selector.to_string(), ident.span());
                quote! {
                    #ident #colon <#ty as ::optionable::OptionableConvert>::try_from_optioned(
                        #value_selector.ok_or(optionable::optionable::Error{ missing_fields: std::vec![#selector_quoted] })?
                    )?
                }
            }
        }
    });

    struct_wrapper(fields_token, &fields.struct_type)
}

/// Returns the field-mappings implementation for `try_from_optioned`.
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
///
/// `merge_self_mut` should be true when the `self_selector` merge argument should be modified with a `& mut` on recursive calls.
fn merge_fields(
    fields: &FieldsParsed,
    self_selector_fn: impl Fn(&TokenStream) -> TokenStream,
    other_selector_fn: impl Fn(&TokenStream) -> TokenStream,
    merge_self_mut: bool,
) -> TokenStream {
    let fields_token = fields.fields.iter().enumerate().map(
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
            match (handling, is_self_resolving_optioned(ty)) {
                (Required, _) | (IsOption, true) => quote! {#deref_modifier #self_selector = #other_selector;},
                (IsOption, false) => quote! {
                    <#ty as ::optionable::OptionableConvert>::merge(#self_merge_mut_modifier #self_selector, #other_selector)?;
                },
                (Other, true) => quote! {
                    if let Some(other_value)=#other_selector{
                        #deref_modifier #self_selector = other_value;
                    }
                },
                (Other, false) => quote! {
                    if let Some(other_value)=#other_selector{
                        <#ty as ::optionable::OptionableConvert>::merge(#self_merge_mut_modifier #self_selector, other_value)?;
                    }
                }
            }
        },
    );

    quote! {
        #(#fields_token)*
    }
}

/// Resolves to a comma-serapated list of entries with a (...) wrapper for unnamed structs
/// and {...} for named structs
fn struct_wrapper(
    tokens: impl Iterator<Item = TokenStream>,
    struct_name_type: &StructType,
) -> TokenStream {
    match struct_name_type {
        StructType::Named => quote!({#(#tokens),*}),
        StructType::Unnamed => quote!((#(#tokens),*)),
        StructType::Unit => quote!(),
    }
}

/// Adjusts the where clause to add the provided predicate  type bounds.
/// Basically the original where clause with a type bound to the preodicate added
/// for every generic type parameter (todo: that is not excluded via the `required` helper attribute.)
fn patch_where_clause_bounds(
    where_clause: &mut WhereClause,
    params: &Punctuated<GenericParam, Token![,]>,
    predicate: impl Fn(&Type) -> TokenStream,
) {
    params.iter().for_each(|param| {
        if let GenericParam::Type(type_param) = param {
            let path = &Type::Path(TypePath {
                qself: None,
                path: type_param.ident.clone().into(),
            });
            add_where_clause_predicate(where_clause, path, &predicate);
        }
    });
}

/// Goes through the list of predicates and appends the new restriction to an already existing
/// entry if found or creates a new one
fn add_where_clause_predicate(
    where_clause: &mut WhereClause,
    ty: &Type,
    entry: impl Fn(&Type) -> TokenStream,
) {
    let bounds = where_clause.predicates.iter_mut().find_map(|pred| {
        if let WherePredicate::Type(pred_ty) = pred
            && *ty == pred_ty.bounded_ty
        {
            Some(&mut pred_ty.bounds)
        } else {
            None
        }
    });
    let entry = entry(ty);
    if let Some(bounds) = bounds {
        bounds.push(parse_quote!(#entry));
    } else {
        where_clause.predicates.push(parse_quote!(#ty: #entry));
    }
}

/// Goes through the attributes, filters for our [`HELPER_IDENT`] helper-attribute identifier
/// and reports an error if anything is found.
fn error_on_helper_attributes(attrs: &[Attribute], err_msg: &'static str) -> syn::Result<()> {
    if attrs
        .iter()
        .filter(|attr| {
            println!("{}", attr.path().to_token_stream());
            attr.path().is_ident(HELPER_IDENT)
        })
        .collect::<Vec<_>>()
        .is_empty()
    {
        Ok(())
    } else {
        error(err_msg)
    }
}

/// Extracts information about the fields we care about, like
/// whether #[optional(required)] is set or whether the type is `Option<...>`.
fn into_field_handling(fields: Fields) -> Result<FieldsParsed, Error> {
    let struct_named = match &fields {
        Fields::Named(_) => StructType::Named,
        Fields::Unnamed(_) => StructType::Unnamed,
        Fields::Unit => StructType::Unit,
    };

    let fields_iter: Box<dyn Iterator<Item = Field>> = match fields {
        Fields::Named(f) => Box::new(f.named.into_iter()),
        Fields::Unnamed(f) => Box::new(f.unnamed.into_iter()),
        Fields::Unit => Box::new(iter::empty()),
    };
    let fields_with_handling = fields_iter
        .map(|field| {
            let attrs = FieldHelperAttributes::from_attributes(&field.attrs)?;
            let handling = if attrs.required.is_some() {
                Required
            } else if is_option(&field.ty) {
                IsOption
            } else {
                Other
            };
            Ok::<_, Error>(FieldParsed { field, handling })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(FieldsParsed {
        struct_type: struct_named,
        fields: fields_with_handling,
    })
}

/// How we handle different cases in order of importance/detection.
/// E.g. if a field is `Required` we don't care whether it's of `Option` type or not.
#[derive(Debug)]
enum FieldHandling {
    Required,
    IsOption,
    Other,
}

/// Just a combination of a field and which handling case it is
#[derive(Debug)]
struct FieldParsed {
    field: Field,
    handling: FieldHandling,
}

/// Type of the Struct we handle
#[derive(Debug, PartialEq)]
enum StructType {
    Named,
    Unnamed,
    Unit,
}

/// Fields with extracted data how we want to handle them
#[derive(Debug)]
struct FieldsParsed {
    struct_type: StructType,
    fields: Vec<FieldParsed>,
}

/// Checks whether this type identifier is a `std::option::Option` or a shortened variant of it.
fn is_option(ty: &Type) -> bool {
    if let Type::Path(TypePath {
        qself: _qself,
        path,
    }) = &ty
        && {
            let segments = &path.segments;
            (segments.len() == 1 && segments[0].ident == "Option")
                || (segments.len() == 2
                    && segments[0].ident == "option"
                    && segments[1].ident == "Option")
                || (segments.len() == 3
                    && segments[0].ident == "std"
                    && segments[1].ident == "option"
                    && segments[2].ident == "Option")
        }
    {
        true
    } else {
        false
    }
}

/// Checks whether this path is `serde::Serialize` or a shortened version of it.
fn is_serialize(path: &Path) -> bool {
    path.is_ident("Serialize") || {
        let segments = &path.segments;
        segments.len() == 2 && segments[0].ident == "serde" && segments[1].ident == "Serialize"
    }
}

/// Tries to resolve the optioned type analogous to what we do in the main crate.
/// Due to limitations to macro resolving (no order guaranteed) we have to have an explicit
/// list of well-known types and their optioned types.
/// For now limited to self-resolving (mostly primitive) types
fn optioned_ty(ty: &Type) -> TokenStream {
    if is_self_resolving_optioned(ty) {
        ty.to_token_stream()
    } else {
        quote! { <#ty as ::optionable::Optionable>::Optioned }
    }
}

const SELF_RESOLVING_TYPES: [&str; 17] = [
    // Rust primitives don't have inner structure, https://doc.rust-lang.org/rust-by-example/primitives.html
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32",
    "f64", "char", "bool", // Other types without inner structure
    "String",
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
fn forwarded_attributes(attrs: &[Attribute]) -> Option<TokenStream> {
    let forward_attrs = attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident(HELPER_ATTR_IDENT) {
                return None;
            }
            match &attr.meta {
                Meta::List(l) => Some(l.tokens.clone()),
                _ => None,
            }
        })
        .collect::<Vec<_>>();
    (!forward_attrs.is_empty()).then(|| quote!(#[#(#forward_attrs),*]))
}

/// error just prepares an error message that references the source span
pub(crate) fn error<S: AsRef<str> + fmt::Display, T>(msg: S) -> syn::Result<T> {
    Err(Error::new(Span::call_site(), msg))
}

#[cfg(test)]
mod tests {
    use crate::derive_optionable;
    use proc_macro2::TokenStream;
    use quote::quote;

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
                    #[automatically_derived]
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

                        fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok(Self{
                                name: value.name.ok_or(optionable::optionable::Error { missing_fields: std::vec!["name"] })?,
                                surname: value.surname.ok_or(optionable::optionable::Error { missing_fields: std::vec!["surname"] })?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name = other_value;
                            }
                            if let Some(other_value) = other.surname {
                                self.surname = other_value;
                            }
                            Ok(())
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
                    #[automatically_derived]
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
                    }
                },
                output: quote! {
                    #[automatically_derived]
                    struct DeriveExampleOpt {
                        name: Option<String>,
                        pub surname: String
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
                                surname: self.surname
                            }
                        }

                        fn try_from_optioned(value:DeriveExampleOpt ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok (Self {
                                name: value.name.ok_or(optionable::optionable::Error { missing_fields: std::vec! ["name"] })?,
                                surname: value.surname
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name = other_value;
                            }
                            self.surname = other.surname;
                            Ok (())
                        }
                    }
                },
            },
            // named struct fields with forwarded derives and Serialize annotations
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    #[optionable(derive(Deserialize,Serialize),suffix="Ac")]
                    #[optionable_attr(serde(rename_all = "camelCase"))]
                    struct DeriveExample {
                        #[optionable_attr(serde(rename = "firstName"))]
                        name: String,
                        middle_name: Option<String>,
                        surname: String,
                    }
                },
                output: quote! {
                    #[automatically_derived]
                    #[derive(Deserialize, Serialize)]
                    #[serde(rename_all = "camelCase")]
                    struct DeriveExampleAc {
                        #[serde(rename = "firstName")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        name: Option<String>,
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
                                middle_name: <Option<String> as ::optionable::OptionableConvert>::into_optioned(self.middle_name),
                                surname: Some(self.surname)
                            }
                        }

                        fn try_from_optioned(value: DeriveExampleAc ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok(Self{
                                name: value.name.ok_or(optionable::optionable::Error { missing_fields: std::vec!["name"]})?,
                                middle_name: <Option<String> as ::optionable::OptionableConvert>::try_from_optioned(value.middle_name)?,
                                surname: value.surname.ok_or(optionable::optionable::Error { missing_fields: std::vec!["surname"]})?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleAc ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name =  other_value;
                            }
                            <Option<String> as ::optionable::OptionableConvert>::merge(&mut self.middle_name, other.middle_name)?;
                            if let Some(other_value) = other.surname {
                                self.surname =  other_value;
                            }
                            Ok(())
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
                    #[automatically_derived]
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

                        fn try_from_optioned(value: DeriveExampleAc ) -> Result <Self, ::optionable::optionable::Error> {
                             Ok(Self{
                                name: value.name.ok_or(optionable::optionable::Error{ missing_fields: std::vec!["name"]})?,
                                surname: value.surname.ok_or(optionable::optionable::Error{ missing_fields: std::vec!["surname"]})?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleAc ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.name {
                                self.name = other_value;
                            }
                            if let Some(other_value) = other.surname {
                                self.surname = other_value;
                            }
                            Ok(())
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
                    #[automatically_derived]
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

                         fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok(Self(
                                value.0.ok_or(optionable::optionable::Error { missing_fields: std::vec!["0"] })?,
                                value.1.ok_or(optionable::optionable::Error { missing_fields: std::vec!["1"] })?
                            ))
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.0 {
                                self.0 = other_value;
                            }
                            if let Some (other_value) = other.1 {
                                self.1 = other_value;
                            }
                            Ok (())
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
                    #[automatically_derived]
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

                         fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok(Self(
                                value.0.ok_or(optionable::optionable::Error { missing_fields: std::vec!["0"] })?,
                                value.1))
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.0 {
                                self.0 = other_value;
                            }
                            self.1 = other.1;
                            Ok (())
                        }
                    }
                },
            },
            // named struct fields with generics
            TestCase {
                input: quote! {
                    #[derive(Optionable)]
                    struct DeriveExample<T, T2: Serialize> where T: DeserializeOwned {
                        output: T,
                        input: T2,
                    }
                },
                output: quote! {
                    #[automatically_derived]
                    struct DeriveExampleOpt<T, T2: Serialize>
                        where T: DeserializeOwned + ::optionable::Optionable,
                              T2: ::optionable::Optionable {
                        output: Option< <T as ::optionable::Optionable>::Optioned>,
                        input: Option< <T2 as ::optionable::Optionable>::Optioned>
                    }

                    #[automatically_derived]
                    impl<T, T2: Serialize> ::optionable::Optionable for DeriveExample<T, T2>
                        where T: DeserializeOwned + ::optionable::Optionable,
                              T2: ::optionable::Optionable  {
                        type Optioned = DeriveExampleOpt<T,T2>;
                    }

                    #[automatically_derived]
                    impl<T, T2: Serialize> ::optionable::Optionable for DeriveExampleOpt<T, T2>
                        where T: DeserializeOwned + ::optionable::Optionable,
                              T2: ::optionable::Optionable  {
                        type Optioned = DeriveExampleOpt<T,T2>;
                    }

                    #[automatically_derived]
                    impl <T, T2:Serialize> ::optionable::OptionableConvert for DeriveExample<T, T2 >
                        where T: DeserializeOwned + ::optionable::OptionableConvert,
                              T2: ::optionable::OptionableConvert {
                        fn into_optioned (self) -> DeriveExampleOpt<T, T2> {
                            DeriveExampleOpt::<T, T2> {
                                output: Some(<T as::optionable::OptionableConvert>::into_optioned(self.output)),
                                input: Some(<T2 as::optionable::OptionableConvert>::into_optioned(self.input))
                            }
                        }

                         fn try_from_optioned(value: DeriveExampleOpt<T, T2> ) -> Result <Self, ::optionable::optionable::Error> {
                             Ok(Self{
                                output: <T as ::optionable::OptionableConvert>::try_from_optioned(value.output.ok_or(optionable::optionable::Error { missing_fields: std::vec!["output"] })?)?,
                                input: <T2 as ::optionable::OptionableConvert>::try_from_optioned(value.input.ok_or(optionable::optionable::Error { missing_fields: std::vec!["input"] })?)?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt<T, T2> ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.output {
                                <T as ::optionable::OptionableConvert>::merge(&mut self.output, other_value)?;
                            }
                            if let Some(other_value) = other.input {
                                <T2 as ::optionable::OptionableConvert>::merge(&mut self.input, other_value)?;
                            }
                            Ok(())
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
                    # [automatically_derived]
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

                         fn try_from_optioned(other: DeriveExampleOpt) -> Result <Self, ::optionable::optionable::Error> {
                            Ok (match other {
                                DeriveExampleOpt::Unit => Self::Unit,
                                DeriveExampleOpt::Plain(other_0) => Self::Plain(
                                    other_0.ok_or(optionable::optionable::Error { missing_fields: std::vec!["0"] })?
                                ),
                                DeriveExampleOpt::Address{street: other_street, number: other_number} => Self::Address{
                                    street: other_street.ok_or(optionable::optionable::Error { missing_fields: std::vec!["street"] })?,
                                    number: other_number.ok_or(optionable::optionable::Error { missing_fields: std::vec!["number"] })?
                                },
                                DeriveExampleOpt::Address2(other_0, other_1) => Self::Address2(
                                    other_0.ok_or(optionable::optionable::Error { missing_fields: std::vec!["0"] })?,
                                    other_1.ok_or(optionable::optionable::Error { missing_fields: std::vec! ["1"] })?)
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt) -> Result<(), ::optionable::optionable::Error> {
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
                },
            },
        ];
        for tc in tcs {
            let input = syn::parse2(tc.input).unwrap();
            let output = derive_optionable(input).unwrap();
            println!("{output}");
            assert_eq!(tc.output.to_string(), output.to_string());
        }
    }
}
