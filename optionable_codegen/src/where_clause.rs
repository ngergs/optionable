use crate::parsed_input::{FieldHandling, FieldParsed};
use darling::FromMeta;
use proc_macro2::Ident;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Where};
use syn::visit::Visit;
use syn::{
    parse_quote, visit, Error, GenericParam, Path, PathSegment, Type, TypeParamBound,
    WhereClause, WherePredicate,
};

pub(crate) struct WhereClauses {
    pub struct_enum_def: WhereClause,
    pub impl_optionable: WhereClause,
    pub impl_optionable_convert: Option<WhereClause>,
}

pub(crate) fn where_clauses<'a>(
    where_clause: Option<WhereClause>,
    generics_params: &Punctuated<GenericParam, Comma>,
    crate_name: &Path,
    input_crate_replacement: Option<&Ident>,
    derive: &BTreeSet<String>,
    no_convert: bool,
    fields: impl IntoIterator<Item = &'a FieldParsed> + Clone,
) -> Result<WhereClauses, Error> {
    let mut where_input = where_clause.unwrap_or_else(|| WhereClause {
        where_token: Where::default(),
        predicates: Punctuated::default(),
    });
    if let Some(input_crate_replacement) = input_crate_replacement {
        where_clause_replace_input_crate_name(&mut where_input, input_crate_replacement);
    }

    let bound_optioned = derive
        .iter()
        .map(|el| {
            if el == "Deserialize" || el == "serde::Deserialize" {
                Cow::Owned("serde::de::DeserializeOwned".to_owned())
            } else {
                Cow::Borrowed(el)
            }
        })
        .map(|el| Path::from_string(&el))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|el| parse_quote!(#el))
        .fold(
            vec![parse_quote!(Sized)],
            |mut acc: Vec<TypeParamBound>, el| {
                acc.push(el);
                acc
            },
        );

    let generic_field_ty = generic_field_types(generics_params, fields);
    let optionable_bound = vec![parse_quote!(#crate_name::Optionable)];
    let optionable_convert_bound = vec![parse_quote!(#crate_name::OptionableConvert)];
    let where_clause_struct_enum_def = where_clause_generalized(
        crate_name,
        where_input.clone(),
        &generic_field_ty,
        &optionable_bound,
        &bound_optioned,
    );
    let where_clause_impl = where_clause_generalized(
        crate_name,
        where_input.clone(),
        &generic_field_ty,
        &optionable_bound,
        &bound_optioned,
    );
    let where_clause_impl_convert = (!no_convert).then(|| {
        where_clause_generalized(
            crate_name,
            where_input,
            &generic_field_ty,
            &optionable_convert_bound,
            &bound_optioned,
        )
    });
    Ok(WhereClauses {
        struct_enum_def: where_clause_struct_enum_def,
        impl_optionable: where_clause_impl,
        impl_optionable_convert: where_clause_impl_convert,
    })
}

/// Returns the list of field types that contain any generic parameter.
fn generic_field_types<'a, 'b>(
    generic_params: impl IntoIterator<Item = &'a GenericParam>,
    fields: impl IntoIterator<Item = &'b FieldParsed>,
) -> Vec<&'b Type> {
    struct TypeHasGenerics<'a> {
        generics: &'a BTreeMap<&'a Ident, bool>,
        value: bool,
    }
    impl<'ast> Visit<'ast> for TypeHasGenerics<'ast> {
        fn visit_path_segment(&mut self, segment: &'ast PathSegment) {
            if self.generics.contains_key(&segment.ident) {
                self.value = true;
            } else {
                // Call the default impl to recurse nested segments.
                visit::visit_path_segment(self, segment);
            }
        }
    }

    let generics = generic_params
        .into_iter()
        .filter_map(|param| {
            if let GenericParam::Type(ty_param) = param {
                Some((&ty_param.ident, false))
            } else {
                None
            }
        })
        .collect::<BTreeMap<_, _>>();
    let mut type_has_generics = {
        TypeHasGenerics {
            generics: &generics,
            value: false,
        }
    };
    if type_has_generics.generics.is_empty() {
        vec![]
    } else {
        fields
            .into_iter()
            .filter(|f| !matches!(f.handling, FieldHandling::Required))
            .filter(|f| {
                type_has_generics.value = false;
                type_has_generics.visit_type(&f.field.ty);
                type_has_generics.value
            })
            .map(|f| &f.field.ty)
            .collect()
    }
}

/// Adjusts the where clause to add the provided predicate type bounds.
/// Basically the original where clause with a type bound to the predicate added
/// for every generic type parameter `params`.
fn where_clause_generalized(
    crate_name: &Path,
    mut where_clause: WhereClause,
    params: &Vec<&Type>,
    bounds: &[TypeParamBound],
    bounds_optioned: &[TypeParamBound],
) -> WhereClause {
    for ty in params {
        where_clause_add_predicate(&mut where_clause, ty, bounds);
        where_clause_add_predicate(
            &mut where_clause,
            &Type::Path(parse_quote!(<#ty as #crate_name::Optionable>::Optioned)),
            bounds_optioned,
        );
    }
    where_clause
}

/// Goes through the list of predicates and appends the new restriction to an already existing
/// entry if found or creates a new one
fn where_clause_add_predicate(
    where_clause: &mut WhereClause,
    ty: &Type,
    bounds: &[TypeParamBound],
) {
    let existing_bounds = where_clause.predicates.iter_mut().find_map(|pred| {
        if let WherePredicate::Type(pred_ty) = pred
            && *ty == pred_ty.bounded_ty
        {
            Some(&mut pred_ty.bounds)
        } else {
            None
        }
    });

    if let Some(existing_bounds) = existing_bounds {
        for bound in bounds {
            if !existing_bounds.iter().any(|el| el == bound) {
                existing_bounds.push(bound.clone());
            }
        }
    } else {
        where_clause
            .predicates
            .push(parse_quote!(#ty: #(#bounds)+*));
    }
}

/// Replaces leading `crate` in the where clause bounds with the provided replacement crate.
fn where_clause_replace_input_crate_name(
    where_clause: &mut WhereClause,
    replacement_crate_ident: &Ident,
) {
    where_clause.predicates.iter_mut().for_each(|pred| {
        if let WherePredicate::Type(pred_ty) = pred {
            pred_ty.bounds.iter_mut().for_each(|bound| {
                if let TypeParamBound::Trait(trait_bound) = bound
                    && trait_bound.path.segments[0].ident == "crate"
                {
                    trait_bound.path.segments[0].ident = replacement_crate_ident.clone();
                }
            });
        }
    });
}
