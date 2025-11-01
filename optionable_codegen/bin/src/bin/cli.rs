use clap::Parser;
use darling::FromMeta;
use optionable_codegen::{attribute_derives, attribute_no_convert, CodegenSettings};
use optionable_codegen_cli::{file_codegen, CodegenConfig, CodegenVisitor};
use proc_macro2::Span;
use std::fs::create_dir_all;
use std::path::PathBuf;
use syn::Item::{Enum, Struct};
use syn::{Attribute, Error, Item, ItemEnum, ItemStruct};

/// Generates `Optionable` and `OptionableConvert` implementation for structs/enums in
/// the referenced `input_file` and all included internal modules recursively.
#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file.
    input_file: PathBuf,
    /// Output directory
    output_dir: PathBuf,
    /// Whether to opt-out of generating `OptionableConvert`-trait implementations.
    #[arg(long, default_value_t = false)]
    no_convert: bool,
    /// Identifiers for which derive statements should be added to the generated structs/enums.
    #[arg(long, short)]
    derive: Vec<String>,
    /// Flag for the purpose of generating code that should be added to the optionable crate itself.
    /// - Replaces the keyword `crate` in the input with the provided string value.
    /// - Prepends the provided string crate name and the respective package subpath to the `impl` definitions.
    /// - Uses in the generated code `crate` instead of `::optionable` to refer to the optionable crate.
    #[arg(long)]
    replace_crate_name: Option<String>,
}

#[derive(Clone)]
struct Visitor {
    type_attrs: Vec<Attribute>,
}

impl Visitor {
    /// Adds the `#[optionable(required)]` attribute to the field if and only if
    /// it has type `ObjectMeta` and has the name `metadata`.
    fn append_attrs(&mut self, attrs: &mut Vec<Attribute>) {
        attrs.append(&mut self.type_attrs.clone());
    }
}

impl CodegenVisitor for Visitor {
    fn visit_input(&mut self, item: &mut Item) {
        match item {
            Struct(ItemStruct { attrs, .. }) | Enum(ItemEnum { attrs, .. }) => {
                self.append_attrs(attrs);
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let type_attrs = input_type_attrs(&args)?;
    let codegen_settings = input_codegen_settings(&args)?;
    create_dir_all(&args.output_dir)?;
    file_codegen(
        &args.input_file,
        &args.output_dir,
        CodegenConfig {
            visitor: Visitor { type_attrs },
            optioned_suffix: "Opt",
            settings: codegen_settings,
            usage_aliases: vec![],
            is_mod_private: false,
        },
    )
}

/// Parses the input args and generated corresponding type attributes
fn input_type_attrs(args: &Args) -> Result<Vec<Attribute>, Error> {
    let mut type_attrs = Vec::new();
    if args.no_convert {
        type_attrs.push(attribute_no_convert());
    }
    if !args.derive.is_empty() {
        let derives = args
            .derive
            .iter()
            .map(|d| syn::parse_str(d))
            .collect::<Result<Vec<_>, _>>()?
            .into();

        type_attrs.push(attribute_derives(&derives));
    }
    Ok(type_attrs)
}

fn input_codegen_settings(args: &Args) -> Result<CodegenSettings, Error> {
    let mut settings = CodegenSettings::default();
    if let Some(replace_crate_name) = &args.replace_crate_name {
        settings.optionable_crate_name = syn::Path::from_string("crate")?;
        settings.ty_prefix = Some(syn::Path::from_string(&format!("::{replace_crate_name}"))?);
        settings.input_crate_replacement =
            Some(syn::Ident::new(replace_crate_name, Span::call_site()));
    }
    Ok(settings)
}
