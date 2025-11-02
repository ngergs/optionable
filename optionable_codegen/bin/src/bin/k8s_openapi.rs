use clap::Parser;
use darling::FromMeta;
use optionable_codegen::CodegenSettings;
use optionable_codegen_cli::{file_codegen, CodegenConfig, CodegenVisitor};
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use std::default::Default;
use std::fs::create_dir_all;
use std::mem::take;
use std::path::PathBuf;
use syn::Item::{Enum, Impl, Struct};
use syn::{
    parse_quote, Attribute, Expr, Fields, ImplItem, Item, ItemImpl, Path, ReturnType, Type,
    TypeGroup, TypeParamBound, TypeParen, TypeReference, WhereClause, WherePredicate,
};

const K8S_OPENAPI: &str = "k8s_openapi";

/// Generates `Optionable` and `OptionableConvert` implementation for structs/enums in
/// the referenced `input_file` and all included internal modules recursively.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file.
    input_file: PathBuf,
    /// Output directory
    output_dir: PathBuf,
}

#[derive(Default)]
/// Visitor for the `k8s-openapi` optionable codegen.
struct Visitor {
    /// The type suffix for the optioned type. Here this is fixed to "Ac".
    optioned_suffix: &'static str,
    /// Additional attributes that should be added to input structs.
    type_attrs_input_struct: Vec<Attribute>,
    /// Additional attributes that should be added to input enums.
    type_attrs_item_enum: Vec<Attribute>,
    /// Impl paths that should be copied over (with adjusting `crate` to `k8s-openapi`).
    impl_copy: Vec<Path>,
    /// Additional item that should be appended to the output (used by `impl_copy` implementation).
    internal_output_additional_item: Option<Item>,
}

impl Clone for Visitor {
    fn clone(&self) -> Self {
        Self {
            optioned_suffix: self.optioned_suffix,
            type_attrs_input_struct: self.type_attrs_input_struct.clone(),
            type_attrs_item_enum: self.type_attrs_item_enum.clone(),
            impl_copy: self.impl_copy.clone(),
            // we want to reset all other fields when entering a new module
            ..Default::default()
        }
    }
}

impl Visitor {
    /// Adds the `#[optionable(required)]` attribute to the field if and only if
    /// it has type `ObjectMeta` and has the name `metadata`.
    /// Returns whether this mutation has been performed.
    fn set_metadata_required(fields: &mut Fields) {
        if let Fields::Named(fields) = fields {
            for field in &mut fields.named {
                if let Some(ident) = &field.ident
                    && ident == "metadata"
                    && Self::is_metadata(&field.ty)
                {
                    field.attrs.push(parse_quote!(#[optionable(required)]));
                }
            }
        }
    }

    /// Returns true if ty is a `path` of `::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta`
    /// or one of its shortened versions.
    fn is_metadata(ty: &Type) -> bool {
        if let Type::Path(type_path) = ty
            && (type_path.path
                == Path::from_string("crate::apimachinery::pkg::apis::meta::v1::ObjectMeta")
                    .unwrap()
                || type_path.path
                    == Path::from_string("crate::apimachinery::pkg::apis::meta::v1::ListMeta")
                        .unwrap())
        {
            true
        } else {
            false
        }
    }
}

impl CodegenVisitor for Visitor {
    fn visit_input(&mut self, item: &mut Item) {
        let suffix = self.optioned_suffix;
        match item {
            Struct(item) => {
                item.attrs.append(&mut self.type_attrs_input_struct.clone());
                item.attrs.push(parse_quote!(#[optionable(suffix=#suffix)]));
                Self::set_metadata_required(&mut item.fields);
            }
            Enum(item) => {
                item.attrs.append(&mut self.type_attrs_item_enum.clone());
                item.attrs.push(parse_quote!(#[optionable(suffix=#suffix)]));
                item.variants
                    .iter_mut()
                    .for_each(|variant| Self::set_metadata_required(&mut variant.fields));
            }
            Impl(item) => {
                if let Some(trait_) = &item.trait_
                    && self.impl_copy.contains(&trait_.1)
                {
                    let mut item = item.clone();
                    if let Type::Path(type_path) = item.self_ty.as_mut()
                        && let Some(path_tail) = type_path.path.segments.last_mut()
                    {
                        // add name prefix to type
                        let mut ident = path_tail.ident.to_string();
                        ident.push_str("Ac");
                        path_tail.ident = Ident::new(&ident, Span::call_site());

                        // replace `crate` with `k8s_openapi`
                        replace_crate_k8s_openapi(&mut item);
                        self.internal_output_additional_item = Some(item.into());
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_output(&mut self, items: &mut Vec<Item>, _: &CodegenSettings) {
        if self.internal_output_additional_item.is_some() {
            let item_additional = take(&mut self.internal_output_additional_item);
            if let Some(item_additional) = item_additional {
                if let Impl(item) = &item_additional
                    && item.self_ty == parse_quote!(ListAc<T>)
                {
                    // todo: generalize, atm ListAc makes a lot of problems due to it being generic over T
                    // and listing items for apply configurations isn't a use case
                    return;
                }
                items.push(item_additional);
            }
        }
    }
}

/// Dedicated binary target for the purpose of codegen for `k8s-openapi`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let codegen_settings = CodegenSettings {
        optionable_crate_name: Path::from_string("crate")?,
        ty_prefix: Some(Path::from_string(&format!("::{K8S_OPENAPI}"))?),
        input_crate_replacement: Some(Ident::new(K8S_OPENAPI, Span::call_site())),
    };
    create_dir_all(&args.output_dir)?;
    let optioned_suffix = "Ac";
    file_codegen(
        &args.input_file,
        &args.output_dir,
        CodegenConfig {
            visitor: Visitor {
                optioned_suffix,
                type_attrs_input_struct: vec![
                    parse_quote!(#[optionable(derive(Clone,std::fmt::Debug,Default,serde::Serialize, serde::Deserialize))]),
                ],
                type_attrs_item_enum: vec![
                    parse_quote!(#[optionable(derive(Clone,std::fmt::Debug,serde::Serialize, serde::Deserialize))]),
                ],
                impl_copy: vec![
                    parse_quote!(crate::Resource),
                    parse_quote!(Resource),
                    parse_quote!(crate::Metadata),
                    parse_quote!(Metadata),
                ],
                ..Default::default()
            },
            optioned_suffix,
            settings: codegen_settings,
            usage_aliases: vec![],
            is_mod_private: false,
        },
    )
}

/// Replace occurrences of `crate` with `k8s-openapi`
fn replace_crate_k8s_openapi(item: &mut ItemImpl) {
    if let Some(trait_) = &mut item.trait_
        && trait_.1.segments[0].to_token_stream().to_string() == "crate"
    {
        trait_.1.segments[0] = parse_quote!(k8s_openapi);
        trait_.1.leading_colon = None;
    }
    replace_crate_k8s_openapi_where_clause(&mut item.generics.where_clause);
    item.items.iter_mut().for_each(|item| match item {
        ImplItem::Type(item) => {
            if let Type::Path(path) = &mut item.ty {
                replace_crate_k8s_openapi_path(&mut path.path);
            }
        }
        ImplItem::Fn(item) => {
            if let ReturnType::Type(_, ty) = &mut item.sig.output {
                replace_crate_k8s_openapi_ty(ty);
            }
        }
        ImplItem::Const(item) => {
            replace_crate_k8s_openapi_where_clause(&mut item.generics.where_clause);
            replace_crate_k8s_openapi_ty(&mut item.ty);
            if let Expr::Path(expr) = &mut item.expr {
                replace_crate_k8s_openapi_path(&mut expr.path);
                if let Some(qself) = &mut expr.qself {
                    replace_crate_k8s_openapi_ty(&mut qself.ty);
                }
            }
        }
        _ => {}
    });
}

fn replace_crate_k8s_openapi_where_clause(where_clause: &mut Option<WhereClause>) {
    if let Some(where_clause) = where_clause {
        where_clause
            .predicates
            .iter_mut()
            .for_each(|where_predicate| {
                if let WherePredicate::Type(ty) = where_predicate {
                    ty.bounds.iter_mut().for_each(|bound| {
                        if let TypeParamBound::Trait(trait_bound) = bound {
                            replace_crate_k8s_openapi_path(&mut trait_bound.path);
                        }
                    });
                }
            });
    }
}

/// Replace occurrences of `crate` with `k8s-openapi`
fn replace_crate_k8s_openapi_ty(ty: &mut Type) {
    match ty {
        Type::Path(ty) => {
            replace_crate_k8s_openapi_path(&mut ty.path);
            if let Some(qself) = &mut ty.qself {
                replace_crate_k8s_openapi_ty(&mut qself.ty);
            }
        }
        Type::Reference(TypeReference { elem, .. })
        | Type::Paren(TypeParen { elem, .. })
        | Type::Group(TypeGroup { elem, .. }) => replace_crate_k8s_openapi_ty(elem),
        _ => {}
    }
}

/// Replace occurrences of `crate` with `k8s-openapi`
fn replace_crate_k8s_openapi_path(path: &mut Path) {
    if path.segments[0].to_token_stream().to_string() == "crate" {
        path.segments[0] = parse_quote!(k8s_openapi);
        path.leading_colon = None;
    }
}
