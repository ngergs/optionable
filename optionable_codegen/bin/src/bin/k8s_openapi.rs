use clap::Parser;
use darling::FromMeta;
use optionable_codegen::CodegenSettings;
use optionable_codegen_cli::{file_codegen, CodegenConfig, CodegenVisitor};
use proc_macro2::Span;
use std::borrow::Cow;
use std::fs::create_dir_all;
use std::path::PathBuf;

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

impl CodegenVisitor for Visitor {}

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
            settings: Cow::Owned(codegen_settings),
            usage_aliases: vec![],
            is_mod_private: false,
        },
    )
}
