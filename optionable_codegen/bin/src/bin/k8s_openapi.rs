use clap::Parser;
use darling::FromMeta;
use optionable_codegen::CodegenSettings;
use optionable_codegen_cli::{file_codegen, CodegenConfig, CodegenVisitor};
use proc_macro2::Span;
use std::fs::create_dir_all;
use std::path::PathBuf;
use syn::{parse_quote, Fields, ItemEnum, ItemStruct, Type};

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

#[derive(Clone)]
struct Visitor {}

impl Visitor {
    /// Adds the `#[optionable(required)]` attribute to the field if and only if
    /// it has type `ObjectMeta` and has the name `metadata`.
    fn set_metadat_required(fields: &mut Fields) {
        if let Fields::Named(fields) = fields {
            fields.named.iter_mut().for_each(|field| {
                if let Some(ident) = &field.ident
                    && ident == "metadata"
                    && Self::is_object_meta(&field.ty)
                {
                    field.attrs.push(parse_quote!(#[optionable(required)]));
                }
            });
        }
    }
    //::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta

    /// Returns true if ty is a `path` of `::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta`
    /// or one of its shortened versions.
    fn is_object_meta(ty: &Type) -> bool {
        if let Type::Path(type_path) = ty
            && (type_path.path
                == syn::Path::from_string("crate::apimachinery::pkg::apis::meta::v1::ObjectMeta")
                    .unwrap()
                || type_path.path
                    == syn::Path::from_string("apimachinery::pkg::apis::meta::v1::ObjectMeta")
                        .unwrap()
                || type_path.path
                    == syn::Path::from_string("pkg::apis::meta::v1::ObjectMeta").unwrap()
                || type_path.path == syn::Path::from_string("meta::v1::ObjectMeta").unwrap()
                || type_path.path == syn::Path::from_string("v1::ObjectMeta").unwrap()
                || type_path.path == syn::Path::from_string("ObjectMeta").unwrap())
        {
            true
        } else {
            false
        }
    }
}

impl CodegenVisitor for Visitor {
    fn visit_input_struct(&mut self, item: &mut ItemStruct) {
        Self::set_metadat_required(&mut item.fields);
    }

    fn visit_input_enum(&mut self, item: &mut ItemEnum) {
        item.variants
            .iter_mut()
            .for_each(|variant| Self::set_metadat_required(&mut variant.fields));
    }
}

/// Dedicated binary target for the purpose of codegen for `k8s-openapi`.
pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let codegen_settings = CodegenSettings {
        optionable_crate_name: syn::Path::from_string("crate")?,
        ty_prefix: Some(syn::Path::from_string(&format!("::{K8S_OPENAPI}"))?),
        input_crate_replacement: Some(syn::Ident::new(K8S_OPENAPI, Span::call_site())),
    };
    create_dir_all(&args.output_dir)?;
    file_codegen(
        &args.input_file,
        &args.output_dir,
        CodegenConfig {
            visitor: Visitor {},
            settings: codegen_settings,
            usage_aliases: vec![],
            is_mod_private: false,
        },
    )
}
