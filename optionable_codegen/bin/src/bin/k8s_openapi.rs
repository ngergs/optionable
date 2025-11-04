use clap::Parser;
use darling::FromMeta;
use optionable_codegen::CodegenSettings;
use optionable_codegen_cli::{file_codegen, CodegenConfig, CodegenVisitor};
use proc_macro2::{Ident, Span};
use std::collections::HashSet;
use std::default::Default;
use std::fs::create_dir_all;
use std::path::PathBuf;
use syn::Item::{Enum, Impl, Struct};
use syn::{parse_quote, Attribute, Fields, Item, Path, Type};

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
    has_resource_impl: HashSet<Ident>,
    has_metadata_impl: HashSet<Ident>,
}

impl Clone for Visitor {
    fn clone(&self) -> Self {
        Self {
            optioned_suffix: self.optioned_suffix,
            type_attrs_input_struct: self.type_attrs_input_struct.clone(),
            type_attrs_item_enum: self.type_attrs_item_enum.clone(),
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
    fn visit_pre_input(&mut self, item: &Item) {
        if let Impl(item) = item
            && let Some(trait_) = &item.trait_
            && let Type::Path(path) = &*item.self_ty
            && path.path.segments.len() == 1
        {
            if trait_.1 == parse_quote!(crate::Resource) || trait_.1 == parse_quote!(Resource) {
                self.has_resource_impl
                    .insert(path.path.segments[0].ident.clone());
            }
            if trait_.1 == parse_quote!(crate::Metadata) || trait_.1 == parse_quote!(Metadata) {
                self.has_metadata_impl
                    .insert(path.path.segments[0].ident.clone());
            }
        }
    }

    fn visit_input(&mut self, item: &mut Item) {
        let suffix = self.optioned_suffix;
        match item {
            Struct(item) => {
                item.attrs.append(&mut self.type_attrs_input_struct.clone());
                item.attrs.push(parse_quote!(#[optionable(suffix=#suffix)]));
                if self.has_resource_impl.contains(&item.ident) {
                    item.attrs
                        .push(parse_quote!(#[optionable(k8s_openapi_resource)]));
                }
                if self.has_metadata_impl.contains(&item.ident) {
                    item.attrs
                        .push(parse_quote!(#[optionable(k8s_openapi_metadata)]));
                }
                Self::set_metadata_required(&mut item.fields);
            }
            Enum(item) => {
                item.attrs.append(&mut self.type_attrs_item_enum.clone());
                item.attrs.push(parse_quote!(#[optionable(suffix=#suffix)]));
                item.variants
                    .iter_mut()
                    .for_each(|variant| Self::set_metadata_required(&mut variant.fields));
            }
            _ => {}
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
                    parse_quote!(#[optionable(derive(Clone,std::fmt::Debug,Default,PartialEq,serde::Serialize, serde::Deserialize))]),
                    parse_quote!(#[optionable_attr(serde(rename_all="camelCase"))]),
                ],
                type_attrs_item_enum: vec![
                    parse_quote!(#[optionable(derive(Clone,std::fmt::Debug,PartialEq,serde::Serialize, serde::Deserialize))]),
                    parse_quote!(#[optionable_attr(serde(rename_all="camelCase",untagged))]),
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
