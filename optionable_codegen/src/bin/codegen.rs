#[cfg(feature = "codegen")]
mod codegen {
    use clap::Parser;
    use optionable_codegen::{attribute_derives, attribute_no_convert};
    use std::fs::create_dir_all;
    use std::path::PathBuf;
    use std::{fs, io};
    use syn::Item::{Enum, Struct};
    use syn::{Attribute, DeriveInput, Error};

    /// Generates `Optionable` and `OptionableConvert` implementation for structs/enums in
    /// all `*.rs` files in the input folder.
    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Args {
        /// Input directory, will be traversed recursively.
        input_dir: PathBuf,
        /// Output directory
        output_dir: PathBuf,
        /// Whether to opt-out of generating `OptionableConvert`-trait implementations.
        #[arg(long, default_value_t = false)]
        no_convert: bool,
        /// Identifiers for which derive statements should be added to the generated structs/enums.
        #[arg(long, short)]
        derive: Vec<String>,
    }

    pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
        let args = Args::parse();
        let type_attrs = input_type_attrs(&args)?;
        create_dir_all(&args.output_dir)?;
        file_codegen(&args.input_dir, &args.output_dir, &type_attrs)
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

    fn file_codegen(
        input_dir: &PathBuf,
        output_dir: &PathBuf,
        type_attrs: &Vec<Attribute>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(output_dir)?;
        let files = fs::read_dir(input_dir)?
            .map(|file| {
                let file = file?;
                let file_type = file.file_type()?;
                Ok::<_, io::Error>((file, file_type))
            })
            .collect::<Result<Vec<_>, _>>()?;
        files
            .iter()
            .filter(|(_, filetype)| filetype.is_dir())
            .map(|(file, _)| {
                file_codegen(
                    &input_dir.join(file.file_name()),
                    &output_dir.join(file.file_name()),
                    type_attrs,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let files = files.into_iter().filter(|(file, filetype)| {
            filetype.is_file() && file.file_name().to_string_lossy().ends_with(".rs")
        });
        for (file, _) in files {
            let content_str = fs::read_to_string(file.path())?;
            let content = syn::parse_file(&content_str)?;
            let result = content
                .items
                .into_iter()
                .map(|item| match item {
                    Struct(mut item) => {
                        item.attrs.append(&mut type_attrs.clone());
                        Ok::<_, Error>(codegen(item)?)
                    }
                    Enum(mut item) => {
                        item.attrs.append(&mut type_attrs.clone());
                        Ok::<_, Error>(codegen(item)?)
                    }
                    _ => Ok(None),
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join("\n\n");
            fs::write(output_dir.join(file.file_name()), result)?;
        }
        Ok(())
    }

    /// Calls the `optionable`-derive macro with the provided argument.
    fn codegen(input: impl Into<DeriveInput>) -> Result<Option<String>, Error> {
        let result = optionable_codegen::derive_optionable(input.into())?;
        let file = syn::parse2(result)?;
        Ok::<_, Error>(Some(prettyplease::unparse(&file)))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "codegen"))]
    panic!("This binary requires the `codegen` feature to be enabled");
    #[cfg(feature = "codegen")]
    codegen::main()
}
