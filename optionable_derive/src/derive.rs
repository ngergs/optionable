use crate::derive::FieldHandling::{IsOption, Other, Required};
use crate::error;
use darling::util::PathList;
use darling::{FromAttributes, FromDeriveInput};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::cmp::PartialEq;
use std::default::Default;
use std::iter;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Where;
use syn::{
    parse_quote, Attribute, Data, DeriveInput, Error, Field, Fields, GenericParam, LitStr, Path, Token,
    Type, TypePath, WhereClause, WherePredicate,
};

const HELPER_IDENT: &str = "optionable";
const ERR_MSG_HELPER_ATTR_ENUM_VARIANTS: &str =
    "#[optionable] helper attributes not supported on enum variant level.";

#[derive(FromDeriveInput)]
#[darling(attributes(optionable))]
/// Helper attributes on the type definition level (attached to the `struct` or `enum` itself).
struct TypeHelperAttributes {
    derive: Option<PathList>,
    #[darling(default=default_suffix)]
    suffix: LitStr,
}

#[derive(FromAttributes)]
#[darling(attributes(optionable))]
/// Helper attributes on the type definition level (attached to the `struct` or `enum` itself).
struct FieldHelperAttributes {
    required: Option<()>,
}

fn default_suffix() -> LitStr {
    LitStr::new("Opt", Span::call_site())
}

/// Derives the `Optionable`-trait from the main `optionable`-library.
#[allow(clippy::too_many_lines)]
pub(crate) fn derive_optionable(input: TokenStream) -> syn::Result<TokenStream> {
    let input = syn::parse2::<DeriveInput>(input)?;
    let attrs = TypeHelperAttributes::from_derive_input(&input)?;
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
    let mut where_clause_convert = where_clause.clone();
    patch_where_clause_bounds(&mut where_clause, &input.generics.params, |_| {
        quote!(::optionable::Optionable)
    });
    patch_where_clause_bounds(&mut where_clause_convert, &input.generics.params, |_| {
        quote!(::optionable::OptionableConvert)
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

            let into_optioned_fields = into_optioned(&fields, None);
            let try_from_optioned_fields = try_from_optioned(&fields, None);
            let merge_fields = merge_fields(&fields, None, None);

            Ok(quote! {
                #[automatically_derived]
                #derives
                #vis struct #type_ident_opt #impl_generics #where_clause #optioned_fields #unnamed_struct_semicolon

                #impls_optionable

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
            })
        }
        Data::Enum(e) => {
            let self_prefix = quote! {self_};
            let value_prefix = quote! {value_};
            let other_prefix = quote! {other_};

            let variants = e
                .variants
                .into_iter()
                .map(|v| {
                    error_on_helper_attributes(&v.attrs, ERR_MSG_HELPER_ATTR_ENUM_VARIANTS)?;
                    Ok::<_, Error>((v.ident, into_field_handling(v.fields)?))
                })
                .collect::<Result<Vec<_>, _>>()?;

            let optioned_variants = variants.iter().map(|(variant, f)| {
                let fields = optioned_fields(f, skip_optionable_if_serde_serialize.as_ref());
                quote!( #variant #fields )
            });
            let into_variants = variants
                .iter()
                .map(|(variant, f)| {
                    let fields = into_optioned(f, Some(&quote! {self_}));
                    let destruct = destructure(f, &self_prefix)?;
                    Ok::<_, Error>(
                        quote!( Self::#variant #destruct => #type_ident_opt::#variant #fields ),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            let try_from_variants = variants
                .iter()
                .map(|(variant, f)| {
                    let fields = try_from_optioned(f, Some(&quote! {value_}));
                    // can't have these for unit types
                    let destruct = destructure(f, &value_prefix)?;
                    Ok::<_, Error>(
                        quote!( #type_ident_opt::#variant #destruct => Self::#variant #fields ),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            let merge_variants = variants
                .iter()
                .map(|(variant, f)| {
                    let fields = merge_fields(f, Some(&self_prefix), Some(&other_prefix));
                    // can't have these for unit types
                    let self_mut_destructure = destructure(f, &self_prefix)?;
                    let other_destructure = destructure(f,  &other_prefix)?;
                    Ok::<_, Error>(quote!( #type_ident_opt::#variant #other_destructure => {
                        if let Self::#variant #self_mut_destructure = self {
                            #fields
                        } else {
                            *self = Self::try_from_optioned(#type_ident_opt::#variant #other_destructure)?;
                        }
                    }))
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(quote!(
                #[automatically_derived]
                #derives
                #vis enum #type_ident_opt #impl_generics #where_clause {
                    #(#optioned_variants),*
                }

                #impls_optionable

                #[automatically_derived]
                impl #impl_generics ::optionable::OptionableConvert for #type_ident #ty_generics #where_clause_convert {
                    fn into_optioned(self) -> #type_ident_opt #ty_generics {
                        match self {
                            #(#into_variants),*
                        }
                    }

                    fn try_from_optioned(value: #type_ident_opt #ty_generics)->Result<Self,::optionable::optionable::Error>{
                        Ok(match value{
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
            ))
        }
        Data::Union(_) => error("#[derive(Optionable)] not supported for unit structs"),
    }
}

/// Constructs a destructure selector for the given fields, e.g. a `vec!(field_a, field_b)`
/// for a named enum or `vec!(0,1)` for an unnamed enum.
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
    let fields_token = fields.fields.iter().map(|FieldParsed { field: Field { vis, ident, ty, .. }, handling }| {
        let colon = ident.as_ref().map(|_| quote! {:});
        match handling {
            Required => quote! {#vis #ident #colon #ty},
            IsOption => quote! {#serde_attributes #vis #ident #colon <#ty as ::optionable::Optionable>::Optioned},
            Other => quote! {#serde_attributes #vis #ident #colon Option<<#ty as ::optionable::Optionable>::Optioned>},
        }
    });
    struct_wrapper(fields_token, &fields.struct_type)
}

/// Returns the field mapping implementation for `into_optioned`.
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
fn into_optioned(fields: &FieldsParsed, self_token: Option<&TokenStream>) -> TokenStream {
    let fields_token = fields.fields.iter().enumerate().map(|(i, f)| {
        let colon = f.field.ident.as_ref().map(|_| quote! {:});
        let Field { ident, ty, .. } = &f.field;
        let selector = ident.as_ref().map_or_else(|| {
            let i = Literal::usize_unsuffixed(i);
            quote! {#i}
        }, ToTokens::to_token_stream);
        let self_selector=  if let Some(self_token)=self_token{
            format_ident!("{self_token}{selector}").to_token_stream()
        } else{
            quote! { self.#selector }
        };
        match f.handling {
            Required => quote! {#ident #colon #self_selector},
            IsOption => quote! {#ident #colon <#ty as ::optionable::OptionableConvert>::into_optioned(#self_selector)},
            Other => quote! {#ident #colon Some(<#ty as ::optionable::OptionableConvert>::into_optioned(#self_selector))}
        }
    });
    struct_wrapper(fields_token, &fields.struct_type)
}

/// Returns the field-mappings implementation for `try_from_optioned`.
/// The returned tokenstream will be of the form `{...}` for named fields and `(...)` for unnamed fields.
/// Does not include any leading `struct/enum` keywords or any trailing `;`.
fn try_from_optioned(fields: &FieldsParsed, value_token: Option<&TokenStream>) -> TokenStream {
    let fields_token = fields.fields.iter().enumerate().map(|(i, FieldParsed { field: Field { ident, ty, .. }, handling })| {
        let colon = ident.as_ref().map(|_| quote! {:});
        let selector = ident.as_ref().map_or_else(|| {
            let i = Literal::usize_unsuffixed(i);
            quote! {#i}
        }, ToTokens::to_token_stream);
        let value_selector=  if let Some(value_token)=value_token{
            format_ident!("{value_token}{selector}").to_token_stream()
        } else{
            quote! { value.#selector }
        };
        match handling {
            Required => quote! {#ident #colon value.#selector},
            IsOption => quote! {
                #ident #colon <#ty as ::optionable::OptionableConvert>::try_from_optioned(
                    #value_selector
                )?
            },
            Other => {
                let selector_quoted=LitStr::new(&selector.to_string(), ident.span());
                quote! {
                    #ident #colon <#ty as ::optionable::OptionableConvert>::try_from_optioned(
                        #value_selector.ok_or(optionable::optionable::Error{ missing_fields: vec![#selector_quoted] })?
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
fn merge_fields(
    fields: &FieldsParsed,
    self_token: Option<&TokenStream>,
    other_token: Option<&TokenStream>,
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
            let self_merge_mut_modifier=self_token.is_none().then(||quote! {&mut});
            let self_selector=  if let Some(self_token)=self_token{
                format_ident!("{self_token}{selector}").to_token_stream()
            } else{
                quote! { self.#selector }
            };
            let other_selector=  if let Some(other_token)=other_token{
                format_ident!("{other_token}{selector}").to_token_stream()
            } else{
                quote! { other.#selector }
            };
            match handling {
                Required => quote! {#self_selector = #other_selector;},
                IsOption => quote! {
                    <#ty as ::optionable::OptionableConvert>::merge(#self_merge_mut_modifier #self_selector, #other_selector)?;
                },
                Other => quote! {
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

#[cfg(test)]
mod tests {
    use crate::derive::derive_optionable;
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
                        name: Option<<String as ::optionable::Optionable>::Optioned>,
                        pub surname: Option<<String as ::optionable::Optionable>::Optioned>
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
                                name: Some(<String as::optionable::OptionableConvert>::into_optioned(self.name)),
                                surname:Some(<String as::optionable::OptionableConvert>::into_optioned(self.surname))
                            }
                        }

                        fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok(Self{
                                name: <String as ::optionable::OptionableConvert>::try_from_optioned(value.name.ok_or(optionable::optionable::Error { missing_fields: vec!["name"] })?)?,
                                surname: <String as ::optionable::OptionableConvert>::try_from_optioned(value.surname.ok_or(optionable::optionable::Error { missing_fields: vec!["surname"] })?)?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.name {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.name, other_value)?;
                            }
                            if let Some(other_value) = other.surname {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.surname, other_value)?;
                            }
                            Ok(())
                        }
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
                        name: Option<<String as ::optionable::Optionable>::Optioned>,
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
                                name: Some(<String as::optionable::OptionableConvert>::into_optioned(self.name)),
                                surname: self.surname
                            }
                        }

                        fn try_from_optioned(value:DeriveExampleOpt ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok (Self {
                                name: <String as ::optionable::OptionableConvert>::try_from_optioned(value.name.ok_or(optionable::optionable::Error { missing_fields: vec! ["name"] })?)?,
                                surname: value.surname
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.name {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.name, other_value)?;
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
                    struct DeriveExample {
                        name: String,
                        middle_name: Option<String>,
                        surname: String,
                    }
                },
                output: quote! {
                    #[automatically_derived]
                    #[derive(Deserialize, Serialize)]
                    struct DeriveExampleAc {
                        #[serde(skip_serializing_if = "Option::is_none")]
                        name: Option<<String as ::optionable::Optionable>::Optioned>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        middle_name: <Option<String> as ::optionable::Optionable>::Optioned,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        surname: Option<<String as ::optionable::Optionable>::Optioned>
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
                                name: Some(<String as::optionable::OptionableConvert>::into_optioned(self.name)),
                                middle_name: <Option<String> as ::optionable::OptionableConvert>::into_optioned(self.middle_name),
                                surname: Some(<String as::optionable::OptionableConvert>::into_optioned(self.surname))
                            }
                        }

                        fn try_from_optioned(value: DeriveExampleAc ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok(Self{
                                name: <String as ::optionable::OptionableConvert>::try_from_optioned(value.name.ok_or(optionable::optionable::Error { missing_fields: vec!["name"]})?)?,
                                middle_name: <Option<String> as ::optionable::OptionableConvert>::try_from_optioned(value.middle_name)?,
                                surname: <String as ::optionable::OptionableConvert>::try_from_optioned(value.surname.ok_or(optionable::optionable::Error { missing_fields: vec!["surname"]})?)?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleAc ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.name {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.name, other_value)?;
                            }
                            <Option<String> as ::optionable::OptionableConvert>::merge(&mut self.middle_name, other.middle_name)?;
                            if let Some(other_value) = other.surname {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.surname, other_value)?;
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
                        name: Option<<String as ::optionable::Optionable>::Optioned>,
                        #[serde(skip_serializing_if = "Option::is_none")]
                        surname: Option<<String as ::optionable::Optionable>::Optioned>
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
                                name: Some(<String as::optionable::OptionableConvert>::into_optioned(self.name)),
                                surname: Some(<String as::optionable::OptionableConvert>::into_optioned(self.surname))
                            }
                        }

                        fn try_from_optioned(value: DeriveExampleAc ) -> Result <Self, ::optionable::optionable::Error> {
                             Ok(Self{
                                name: <String as ::optionable::OptionableConvert>::try_from_optioned(value.name.ok_or(optionable::optionable::Error{ missing_fields: vec!["name"]})?)?,
                                surname: <String as ::optionable::OptionableConvert>::try_from_optioned(value.surname.ok_or(optionable::optionable::Error{ missing_fields: vec!["surname"]})?)?
                            })
                        }

                        fn merge(&mut self, other: DeriveExampleAc ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.name {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.name, other_value)?;
                            }
                            if let Some(other_value) = other.surname {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.surname, other_value)?;
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
                        pub Option<<String as ::optionable::Optionable>::Optioned>,
                        Option<<i32 as ::optionable::Optionable>::Optioned>
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
                                Some(<String as::optionable::OptionableConvert>::into_optioned(self.0)),
                                Some(<i32 as::optionable::OptionableConvert>::into_optioned(self.1))
                            )
                        }

                         fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok(Self(
                                <String as ::optionable::OptionableConvert>::try_from_optioned(value.0.ok_or(optionable::optionable::Error { missing_fields: vec!["0"] })?)?,
                                <i32 as ::optionable::OptionableConvert>::try_from_optioned(value.1.ok_or(optionable::optionable::Error { missing_fields: vec!["1"] })?)?
                            ))
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.0 {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.0, other_value)?;
                            }
                            if let Some (other_value) = other.1 {
                                <i32 as ::optionable::OptionableConvert>::merge(&mut self.1, other_value)?;
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
                        pub Option<<String as ::optionable::Optionable>::Optioned>,
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
                                Some(<String as::optionable::OptionableConvert>::into_optioned(self.0)),
                                self.1
                            )
                        }

                         fn try_from_optioned(value: DeriveExampleOpt ) -> Result <Self, ::optionable::optionable::Error> {
                            Ok(Self(
                                <String as ::optionable::OptionableConvert>::try_from_optioned(value.0.ok_or(optionable::optionable::Error { missing_fields: vec!["0"] })?)?,
                                value.1))
                        }

                        fn merge(&mut self, other: DeriveExampleOpt ) -> Result<(), ::optionable::optionable::Error> {
                            if let Some(other_value) = other.0 {
                                <String as ::optionable::OptionableConvert>::merge(&mut self.0, other_value)?;
                            }
                            self.1=other.1;
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
                        output: Option<<T as ::optionable::Optionable>::Optioned>,
                        input: Option<<T2 as ::optionable::Optionable>::Optioned>
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
                                output: <T as ::optionable::OptionableConvert>::try_from_optioned(value.output.ok_or(optionable::optionable::Error { missing_fields: vec!["output"] })?)?,
                                input: <T2 as ::optionable::OptionableConvert>::try_from_optioned(value.input.ok_or(optionable::optionable::Error { missing_fields: vec!["input"] })?)?
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
                        Plain( Option<<String as ::optionable::Optionable>::Optioned> ),
                        Address{ street: Option<< String as ::optionable::Optionable>::Optioned>, number:Option<<u32 as ::optionable::Optionable>::Optioned> },
                        Address2( Option<<String as ::optionable::Optionable>::Optioned>, Option<<u32 as ::optionable::Optionable>::Optioned> )
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
                                    Some(<String as ::optionable::OptionableConvert>::into_optioned(self_0))
                                ),
                                Self::Address{street: self_street, number: self_number} => DeriveExampleOpt::Address{
                                    street: Some(<String as ::optionable::OptionableConvert>::into_optioned(self_street)),
                                    number: Some(<u32 as ::optionable::OptionableConvert>::into_optioned(self_number))
                                },
                                Self::Address2(self_0, self_1) => DeriveExampleOpt::Address2(
                                    Some(<String as ::optionable::OptionableConvert>::into_optioned(self_0)),
                                    Some(<u32 as ::optionable::OptionableConvert>::into_optioned(self_1))
                                )
                            }
                        }

                         fn try_from_optioned(value: DeriveExampleOpt) -> Result <Self, ::optionable::optionable::Error> {
                            Ok (match value {
                                DeriveExampleOpt::Unit => Self::Unit,
                                DeriveExampleOpt::Plain(value_0) => Self::Plain(
                                    <String as ::optionable::OptionableConvert>::try_from_optioned(value_0.ok_or(optionable::optionable::Error { missing_fields: vec!["0"] })?)?
                                ),
                                DeriveExampleOpt::Address{street: value_street, number: value_number} => Self::Address{
                                    street: <String as ::optionable::OptionableConvert>::try_from_optioned(value_street.ok_or(optionable::optionable::Error { missing_fields: vec!["street"] })?)?,
                                    number: <u32 as ::optionable::OptionableConvert> ::try_from_optioned(value_number.ok_or(optionable::optionable::Error { missing_fields: vec!["number"] })?)?
                                },
                                DeriveExampleOpt::Address2(value_0, value_1) => Self::Address2(
                                    <String as ::optionable::OptionableConvert>::try_from_optioned(value_0.ok_or(optionable::optionable::Error { missing_fields: vec!["0"] })?)?,
                                    <u32 as ::optionable::OptionableConvert>::try_from_optioned(value_1.ok_or(optionable::optionable::Error { missing_fields: vec ! ["1"] })?)?)
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
                                            <String as ::optionable::OptionableConvert>::merge(self_0, other_value)?;
                                        }
                                    } else {
                                        *self = Self::try_from_optioned(DeriveExampleOpt::Plain(other_0))?;
                                    }
                                },
                                DeriveExampleOpt::Address{street: other_street, number: other_number} => {
                                    if let Self::Address{street: self_street, number: self_number}  = self{
                                        if let Some(other_value) = other_street {
                                            <String as ::optionable::OptionableConvert>::merge(self_street, other_value)?;
                                        }
                                        if let Some(other_value) = other_number {
                                            <u32 as ::optionable::OptionableConvert>::merge(self_number, other_value)?;
                                        }
                                    } else {
                                        *self = Self::try_from_optioned(DeriveExampleOpt::Address{street: other_street, number: other_number})?;
                                    }
                                },
                                DeriveExampleOpt::Address2(other_0, other_1) => {
                                    if let Self::Address2(self_0, self_1) = self{
                                        if let Some(other_value) = other_0 {
                                            <String as ::optionable::OptionableConvert>::merge(self_0, other_value)?;
                                        }
                                        if let Some(other_value) = other_1 {
                                            <u32 as ::optionable::OptionableConvert>::merge(self_1, other_value)?;
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
            let output = derive_optionable(tc.input).unwrap();
            println!("{output}");
            assert_eq!(tc.output.to_string(), output.to_string());
        }
    }
}
