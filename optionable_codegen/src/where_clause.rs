use crate::parsed_input::{FieldHandling, FieldParsed};
use crate::TypeHelperAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::borrow::Cow;
use std::collections::BTreeMap;
use syn::punctuated::Punctuated;
use syn::token::Where;
use syn::visit::Visit;
use syn::{
    parse_quote, visit, GenericParam, Generics, Path, PathSegment, Type, TypeParamBound,
    TypePath, WhereClause, WherePredicate,
};

pub(crate) struct WhereClauses {
    pub struct_enum_def: WhereClause,
    pub impl_optionable: WhereClause,
    pub impl_optionable_convert: Option<WhereClause>,
}

pub(crate) fn where_clauses<'a>(
    crate_name: &Path,
    input_crate_replacement: Option<&Ident>,
    generics: &'a Generics,
    attrs: &TypeHelperAttributes,
    fields: impl IntoIterator<Item = &'a FieldParsed> + Clone,
) -> WhereClauses {
    let generic_params = generic_params_need_optionable(&generics.params, fields);
    let mut where_input = generics
        .where_clause
        .clone()
        .unwrap_or_else(|| WhereClause {
            where_token: Where::default(),
            predicates: Punctuated::default(),
        });
    if let Some(input_crate_replacement) = input_crate_replacement {
        where_clause_replace_input_crate_name(&mut where_input, input_crate_replacement);
    }

    let predicate_struct_enum_optioned = if let Some(derive) = &attrs.derive
        && !derive.is_empty()
    {
        &quote!(Sized + #(#derive)+*)
    } else {
        &quote!(Sized)
    };
    let where_clause_struct_enum_def = where_clause_generalized(
        crate_name,
        &generic_params,
        where_input.clone(),
        &quote!(#crate_name::Optionable),
        // todo: excludes the usage of types that allow unsized types, like a generic parameter `T::Optioned=Cow<...>`
        predicate_struct_enum_optioned,
    );
    let where_clause_impl = where_clause_generalized(
        crate_name,
        &generic_params,
        where_input.clone(),
        &quote!(#crate_name::Optionable),
        predicate_struct_enum_optioned,
    );
    let where_clause_impl_convert = attrs.no_convert.is_none().then(|| {
        where_clause_generalized(
            crate_name,
            &generic_params,
            where_input,
            &quote!(#crate_name::OptionableConvert),
            predicate_struct_enum_optioned,
        )
    });
    WhereClauses {
        struct_enum_def: where_clause_struct_enum_def,
        impl_optionable: where_clause_impl,
        impl_optionable_convert: where_clause_impl_convert,
    }
}

/// Internal generalized logic for the where clause
fn where_clause_generalized<'a>(
    crate_name: &Path,
    generic_params: &Vec<&Ident>,
    mut where_clause: WhereClause,
    predicate: &TokenStream,
    predicate_optioned: &TokenStream,
) -> WhereClause {
    where_clause_add_params(
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
) -> Vec<&'a Ident> {
    struct TypeNeedsOptionableVisitor<'a>(BTreeMap<&'a Ident, bool>);
    impl<'ast> Visit<'ast> for TypeNeedsOptionableVisitor<'ast> {
        fn visit_path_segment(&mut self, segment: &'ast PathSegment) {
            if segment.arguments.is_none()
                && self.0.contains_key(&segment.ident)
                && !(*self
                    .0
                    .get(&segment.ident)
                    .map(Cow::Borrowed)
                    .unwrap_or_default())
            {
                self.0.insert(&segment.ident, true);
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
                    Some((&ty_param.ident, false))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<_, _>>(),
    );
    if type_needs_optionable.0.is_empty() {
        vec![]
    } else {
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
}

/// Adjusts the where clause to add the provided predicate type bounds.
/// Basically the original where clause with a type bound to the predicate added
/// for every generic type parameter `params`.
fn where_clause_add_params<'a>(
    crate_name: &Path,
    where_clause: &mut WhereClause,
    params: &Vec<&Ident>,
    predicate: &TokenStream,
    predicate_optioned: &TokenStream,
) {
    for ty_ident in params {
        let ty_path = Type::Path(TypePath {
            qself: None,
            path: (*ty_ident).clone().into(),
        });
        where_clause_add_predicate(where_clause, &ty_path, predicate);
        where_clause_add_predicate(
            where_clause,
            &Type::Path(parse_quote!(<#ty_ident as #crate_name::Optionable>::Optioned)),
            predicate_optioned,
        );
    }
}

/// Goes through the list of predicates and appends the new restriction to an already existing
/// entry if found or creates a new one
fn where_clause_add_predicate(where_clause: &mut WhereClause, ty: &Type, entry: &TokenStream) {
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

/// Replaces leading `crate` in the where clause bounds with the provided replacement crate.
fn where_clause_replace_input_crate_name(
    where_clause: &mut WhereClause,
    replacement_crate_ident: &Ident,
) {
    where_clause.predicates.iter_mut().for_each(|pred| {
        if let WherePredicate::Type(pred_ty) = pred {
            pred_ty.bounds.iter_mut().for_each(|bound| {
                if let TypeParamBound::Trait(trait_bound) = bound
                    && trait_bound.path.segments[0].ident.to_string() == "crate"
                {
                    trait_bound.path.segments[0].ident = replacement_crate_ident.clone();
                }
            })
        }
    });
}
