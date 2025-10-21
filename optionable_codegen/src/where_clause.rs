use crate::parsed_input::{FieldHandling, FieldParsed};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::borrow::Cow;
use std::collections::BTreeMap;
use syn::punctuated::Punctuated;
use syn::token::Where;
use syn::visit::Visit;
use syn::{
    parse_quote, visit, GenericParam, Generics, Path, PathSegment, Type, TypePath,
    WhereClause, WherePredicate,
};

/// Extracts the where clauses from the input for the `Optionable` trait, patching the bound to the optioned types.
pub(crate) fn where_clauses<'a>(
    crate_name: &Path,
    generics: &'a Generics,
    fields: impl IntoIterator<Item = &'a FieldParsed>,
) -> WhereClause {
    where_clauses_generalized(
        crate_name,
        generics,
        fields,
        &quote!(#crate_name::Optionable),
        // todo: excludes the usage of types that allow unsized types, like a generic parameter `T::Optioned=Cow<...>`
        &quote!(Sized),
    )
}

/// Extracts the where clauses from the input for the `OptionableConvert` trait, patching the bound to the optioned types.
pub(crate) fn where_clauses_convert<'a>(
    crate_name: &Path,
    generics: &'a Generics,
    fields: impl IntoIterator<Item = &'a FieldParsed>,
) -> WhereClause {
    where_clauses_generalized(
        crate_name,
        generics,
        fields,
        &quote!(#crate_name::OptionableConvert),
        // todo: excludes the usage of types that allow unsized types, like a generic parameter `T::Optioned=Cow<...>`
        &quote!(Sized),
    )
}

/// Internal generalized logic for the where clause
fn where_clauses_generalized<'a>(
    crate_name: &Path,
    generics: &'a Generics,
    fields: impl IntoIterator<Item = &'a FieldParsed>,
    predicate: &TokenStream,
    predicate_optioned: &TokenStream,
) -> WhereClause {
    let generic_params = generic_params_need_optionable(&generics.params, fields);
    let mut where_clause = generics
        .where_clause
        .clone()
        .unwrap_or_else(|| WhereClause {
            where_token: Where::default(),
            predicates: Punctuated::default(),
        });
    patch_where_clause_bounds(
        crate_name,
        &mut where_clause,
        &generic_params,
        predicate,
        predicate_optioned,
    );
    where_clause
}

/// Gets the list of generic parameters `T` which needs to be restricted to implement `Optionable`.
/// For this purpose the list of `fields` is gone through and all non-required fields are checked
/// for using any generic parameters.
fn generic_params_need_optionable<'a>(
    generic_params: impl IntoIterator<Item = &'a GenericParam>,
    fields: impl IntoIterator<Item = &'a FieldParsed>,
) -> Vec<Ident> {
    struct TypeNeedsOptionableVisitor(BTreeMap<Ident, bool>);
    impl<'ast> Visit<'ast> for TypeNeedsOptionableVisitor {
        fn visit_path_segment(&mut self, segment: &'ast PathSegment) {
            if segment.arguments.is_none()
                && self.0.contains_key(&segment.ident)
                && !(*self
                    .0
                    .get(&segment.ident)
                    .map(Cow::Borrowed)
                    .unwrap_or_default())
            {
                self.0.insert(segment.ident.clone(), true);
            }
            // Call the default impl to recurse nested segments.
            visit::visit_path_segment(self, segment);
        }
    }

    let mut type_needs_optionable = TypeNeedsOptionableVisitor(
        generic_params
            .into_iter()
            .filter_map(|param| {
                if let GenericParam::Type(ty_param) = param {
                    Some((ty_param.ident.clone(), false))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<_, _>>(),
    );
    fields
        .into_iter()
        .filter(|f| !matches!(f.handling, FieldHandling::Required))
        .for_each(|f| type_needs_optionable.visit_type(&f.field.ty));
    type_needs_optionable
        .0
        .into_iter()
        .filter_map(|(k, v)| if v { Some(k) } else { None })
        .collect()
}

/// Adjusts the where clause to add the provided predicate  type bounds.
/// Basically the original where clause with a type bound to the predicate added
/// for every generic type parameter `params`.
fn patch_where_clause_bounds<'a>(
    crate_name: &Path,
    where_clause: &mut WhereClause,
    params: impl IntoIterator<Item = &'a Ident>,
    predicate: &TokenStream,
    predicate_optioned: &TokenStream,
) {
    for ty_ident in params {
        let ty_path = Type::Path(TypePath {
            qself: None,
            path: ty_ident.clone().into(),
        });
        add_where_clause_predicate(where_clause, &ty_path, predicate);
        add_where_clause_predicate(
            where_clause,
            &Type::Path(parse_quote!(<#ty_ident as #crate_name::Optionable>::Optioned)),
            predicate_optioned,
        );
    }
}

/// Goes through the list of predicates and appends the new restriction to an already existing
/// entry if found or creates a new one
fn add_where_clause_predicate(where_clause: &mut WhereClause, ty: &Type, entry: &TokenStream) {
    let bounds = where_clause.predicates.iter_mut().find_map(|pred| {
        if let WherePredicate::Type(pred_ty) = pred
            && *ty == pred_ty.bounded_ty
        {
            Some(&mut pred_ty.bounds)
        } else {
            None
        }
    });
    if let Some(bounds) = bounds {
        bounds.push(parse_quote!(#entry));
    } else {
        where_clause.predicates.push(parse_quote!(#ty: #entry));
    }
}
