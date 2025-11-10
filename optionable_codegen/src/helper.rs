//! Collections of crate-internal functions that do not hold much interesting logic
//! but help with the codegen.

use crate::parsed_input::{StructParsed, StructType};
use crate::HELPER_IDENT;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::fmt;
use syn::token::PathSep;
use syn::{Attribute, Error, GenericArgument, PathArguments, Type, TypePath};

/// error just prepares an error message that references the source span
pub(crate) fn error<S: AsRef<str> + fmt::Display, T>(msg: S) -> syn::Result<T> {
    Err(Error::new(Span::call_site(), msg))
}

/// Constructs a destructure selector for the given fields, e.g. a `(field_a, field_b)`
/// for a named enum or `(0,1)` for an unnamed enum.
pub(crate) fn destructure(
    fields: &StructParsed,
    prefix: &TokenStream,
) -> Result<TokenStream, Error> {
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
                    let prefixed_ident = format_ident!("{1}{0}", ident, prefix.to_string());
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

/// Resolves to a comma-separated list of entries with a (...) wrapper for unnamed structs
/// and {...} for named structs
pub(crate) fn struct_wrapper(
    tokens: impl IntoIterator<Item = TokenStream>,
    struct_name_type: &StructType,
) -> TokenStream {
    let tokens = tokens.into_iter();
    match struct_name_type {
        StructType::Named => quote!({#(#tokens),*}),
        StructType::Unnamed => quote!((#(#tokens),*)),
        StructType::Unit => quote!(),
    }
}

/// Goes through the attributes, filters for our [`HELPER_IDENT`] helper-attribute identifier
/// and reports an error if anything is found.
pub(crate) fn error_on_helper_attributes(
    attrs: &[Attribute],
    err_msg: &'static str,
) -> syn::Result<()> {
    if attrs
        .iter()
        .filter(|attr| attr.path().is_ident(HELPER_IDENT))
        .collect::<Vec<_>>()
        .is_empty()
    {
        Ok(())
    } else {
        error(err_msg)
    }
}

/// Replaces `crate` in the path and all angle-bracketed path arguments with the provided value.
/// Does nothing if the type is not a path.
pub(crate) fn type_path_replace_crate(ty: &mut Type, replacement: &Ident) {
    if let Type::Path(ty_path) = ty {
        if let Some(first_path_segment) = ty_path.path.segments.first_mut()
            && first_path_segment.ident == "crate"
        {
            first_path_segment.ident = replacement.clone();
            ty_path.path.leading_colon = Some(PathSep::default());
        }
        ty_path.path.segments.iter_mut().for_each(|p| {
            if let PathArguments::AngleBracketed(args) = &mut p.arguments {
                args.args.iter_mut().for_each(|arg| {
                    if let GenericArgument::Type(ty) = arg {
                        type_path_replace_crate(ty, replacement);
                    }
                });
            }
        });
    }
}

/// Checks whether this type identifier is a `std::option::Option` or a shortened variant of it.
pub(crate) fn is_option(ty: &Type) -> bool {
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
                    && (segments[0].ident == "core" || segments[0].ident == "std")
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
pub(crate) fn is_serialize(path: &str) -> bool {
    path == "Serialize" || path == "serde :: Serialize"
}
