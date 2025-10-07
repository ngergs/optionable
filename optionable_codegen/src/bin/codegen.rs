extern crate core;

#[cfg(feature = "codegen")]
mod codegen {
    use clap::Parser;
    use darling::FromMeta;
    use optionable_codegen::{attribute_derives, attribute_no_convert, CodegenSettings};
    use proc_macro2::Span;
    use std::borrow::Cow;
    use std::fs::create_dir_all;
    use std::mem::take;
    use std::path::{Path, PathBuf};
    use std::{fs, io};
    use syn::Item::{Enum, Mod, Struct};
    use syn::{Attribute, DeriveInput, Error, Item};

    /// Generates `Optionable` and `OptionableConvert` implementation for structs/enums in
    /// the referenced `input_file` and all included internal modules recursively.
    #[derive(Parser, Debug)]
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
        /// Replaces the keyword `crate` in the input with the provided string value and also
        /// uses in the generated code `crate` instead of `::optionable` to refer to the optionable crate.
        #[arg(long)]
        replace_crate_name: Option<String>,
    }

    pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
        let args = Args::parse();
        let type_attrs = input_type_attrs(&args)?;
        let codegen_settings = input_codegen_settings(&args)?;
        create_dir_all(&args.output_dir)?;
        file_codegen(
            &args.input_file,
            &args.output_dir,
            &type_attrs,
            &codegen_settings,
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
            settings.input_crate_replacement =
                Some(syn::Ident::new(replace_crate_name, Span::call_site()));
        }
        Ok(settings)
    }

    /// Read the respective `input_file` and calls codegen for it. The result is written
    /// into the `output_path` under the same `file_name` as the input.
    fn file_codegen(
        input_file: &Path,
        output_path: &Path,
        type_attrs: &Vec<Attribute>,
        codegen_settings: &CodegenSettings,
    ) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(output_path)?;
        let content_str = fs::read_to_string(input_file)?;
        let content = syn::parse_file(&content_str)?;
        let input_path = input_file
            .parent()
            .ok_or("current file {input_file} has no parent")?;
        let result = content
            .items
            .into_iter()
            .map(|item| item_codegen(item, input_path, output_path, type_attrs, codegen_settings))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let result = syn::File {
            shebang: None,
            attrs: vec![],
            items: result,
        };
        let result = prettyplease::unparse(&result);
        fs::write(
            output_path.join(input_file.file_name().ok_or::<io::Error>(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file name ends with `..` which is not supported: {input_file}",
            ))?),
            result,
        )?;
        Ok(())
    }

    /// Calls codegen for the respective item.
    fn item_codegen(
        item: Item,
        input_path: &Path,
        output_path: &Path,
        type_attrs: &Vec<Attribute>,
        codegen_settings: &CodegenSettings,
    ) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        match item {
            Struct(mut item) => {
                item.attrs.append(&mut type_attrs.clone());
                Ok::<_, Box<dyn std::error::Error>>(derive_codegen(item, codegen_settings)?)
            }
            Enum(mut item) => {
                item.attrs.append(&mut type_attrs.clone());
                Ok::<_, Box<dyn std::error::Error>>(derive_codegen(item, codegen_settings)?)
            }
            Mod(mut mod_entry) => {
                if let Some(content) = mod_entry.content.as_mut() {
                    let items = take(&mut content.1);
                    content.1 = items
                        .into_iter()
                        .map(|item| {
                            item_codegen(
                                item,
                                input_path,
                                output_path,
                                type_attrs,
                                codegen_settings,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .flatten()
                        .collect();
                    Ok(vec![Mod(mod_entry)])
                } else {
                    // include of a module from another file
                    let same_folder_mod_path = input_path.join(format!("{}.rs", mod_entry.ident));
                    if same_folder_mod_path.exists() {
                        file_codegen(
                            &same_folder_mod_path,
                            output_path,
                            type_attrs,
                            codegen_settings,
                        )?;
                    } else {
                        let sub_folder_mod_path =
                            input_path.join(mod_entry.ident.to_string()).join("mod.rs");
                        file_codegen(
                            &sub_folder_mod_path,
                            &output_path.join(mod_entry.ident.to_string()),
                            type_attrs,
                            codegen_settings,
                        )?;
                    }
                    Ok(vec![Mod(mod_entry)])
                }
            }
            _ => Ok(vec![]),
        }
    }

    /// Calls the `optionable`-derive macro with the provided `DeriveInput` argument.
    fn derive_codegen(
        input: impl Into<DeriveInput>,
        codegen_settings: &CodegenSettings,
    ) -> Result<Vec<Item>, Error> {
        let result = optionable_codegen::derive_optionable(
            input.into(),
            Some(Cow::Borrowed(codegen_settings)),
        )?;
        syn::parse2(result).map(|f: syn::File| f.items)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "codegen"))]
    panic!("This binary requires the `codegen` feature to be enabled");
    #[cfg(feature = "codegen")]
    codegen::main()
}
