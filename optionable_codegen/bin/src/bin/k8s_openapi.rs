use clap::Parser;
use darling::FromMeta;
use optionable_codegen::CodegenSettings;
use optionable_codegen_cli::{file_codegen, CodegenConfig, CodegenVisitor};
use proc_macro2::{Ident, Span};
use std::collections::HashSet;
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
struct Visitor {
    current_item_resource_ident: Option<Ident>,
    has_impl_resources: HashSet<Ident>,
}

impl Clone for Visitor {
    fn clone(&self) -> Self {
        // we want to reset all fields when entering a new module
        Self::default()
    }
}

impl Visitor {
    /// Adds the `#[optionable(required)]` attribute to the field if and only if
    /// it has type `ObjectMeta` and has the name `metadata`.
    /// Returns whether this mutation has been performed.
    fn set_metadata_required(fields: &mut Fields) -> bool {
        if let Fields::Named(fields) = fields {
            for field in &mut fields.named {
                if let Some(ident) = &field.ident
                    && ident == "metadata"
                    && Self::is_object_meta(&field.ty)
                {
                    field.attrs.push(parse_quote!(#[optionable(required)]));
                    return true;
                }
            }
        }
        false
    }

    /// Returns true if ty is a `path` of `::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta`
    /// or one of its shortened versions.
    fn is_object_meta(ty: &Type) -> bool {
        if let Type::Path(type_path) = ty
            && (type_path.path
                == Path::from_string("crate::apimachinery::pkg::apis::meta::v1::ObjectMeta")
                    .unwrap()
                || type_path.path
                    == Path::from_string("apimachinery::pkg::apis::meta::v1::ObjectMeta").unwrap()
                || type_path.path == Path::from_string("pkg::apis::meta::v1::ObjectMeta").unwrap()
                || type_path.path == Path::from_string("meta::v1::ObjectMeta").unwrap()
                || type_path.path == Path::from_string("v1::ObjectMeta").unwrap()
                || type_path.path == Path::from_string("ObjectMeta").unwrap())
        {
            true
        } else {
            false
        }
    }

    /// Adds the `#optionable(derive(k8s_openapi::Resource))` and `#[optionable_attr(#[resource(inherit="<ty_ident>")])]` attribute to the field if and only if
    /// it has type `ObjectMeta` and has the name `metadata`.
    fn add_derive_resource(&mut self, ident: &Ident, attrs: &mut Vec<Attribute>) {
        if self.has_impl_resources.contains(ident) {
            self.current_item_resource_ident = Some(ident.clone());
            attrs.push(parse_quote!(#[optionable(derive(kube::Resource))]));
            attrs.push(parse_quote!(#[optionable_attr(resource(inherit = #ident))]));
        } else {
            self.current_item_resource_ident = None;
        }
    }
}

impl CodegenVisitor for Visitor {
    /// Collect all `Resource` implementation statements for objects that actually
    /// can be used with the derive statement from `kube` which requires `metadata: ObjectMeta`
    ///  as a field.
    fn visit_pre_input(&mut self, item: &Item) {
        if let Impl(item) = item
            && let Some(trait_) = &item.trait_
            && (trait_.1 == Path::from_string("crate::Resource").unwrap()
                || trait_.1 == Path::from_string("Resource").unwrap())
            && let Type::Path(type_path) = &*item.self_ty
            && let Some(ident) = type_path.path.get_ident()
        {
            self.has_impl_resources.insert(ident.clone());
        }
    }
    fn visit_input(&mut self, item: &mut Item) {
        match item {
            Struct(item) => {
                item.attrs.push(parse_quote!(#[optionable(suffix="Ac")]));
                if Self::set_metadata_required(&mut item.fields) {
                    self.add_derive_resource(&item.ident, &mut item.attrs);
                }
            }
            Enum(item) => {
                item.attrs.push(parse_quote!(#[optionable(suffix="Ac")]));
                if item
                    .variants
                    .iter_mut()
                    .all(|variant| Self::set_metadata_required(&mut variant.fields))
                {
                    self.add_derive_resource(&item.ident, &mut item.attrs);
                }
            }
            _ => self.current_item_resource_ident = None,
        }
    }

    fn visit_output(&self, item: &mut Vec<Item>, settings: &CodegenSettings) {
        if let Some(ident) = &self.current_item_resource_ident
            && let Some(mut ty_prefix) = settings.ty_prefix.clone()
        {
            ty_prefix.segments.push(ident.clone().into());
            item.push(Item::Use(parse_quote!(
                #[allow(unused_imports)]
                use #ty_prefix;
            )));
        }
    }
}

/// Dedicated binary target for the purpose of codegen for `k8s-openapi`.
pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let codegen_settings = CodegenSettings {
        optionable_crate_name: Path::from_string("crate")?,
        ty_prefix: Some(Path::from_string(&format!("::{K8S_OPENAPI}"))?),
        input_crate_replacement: Some(Ident::new(K8S_OPENAPI, Span::call_site())),
    };
    create_dir_all(&args.output_dir)?;
    file_codegen(
        &args.input_file,
        &args.output_dir,
        CodegenConfig {
            visitor: Visitor::default(),
            optioned_suffix: "Ac",
            settings: codegen_settings,
            usage_aliases: vec![],
            is_mod_private: false,
        },
    )
}
