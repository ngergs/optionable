use clap::Parser;
#[cfg(feature = "codegen")]
use std::fs;
#[cfg(feature = "codegen")]
use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;
#[cfg(feature = "codegen")]
use syn::DeriveInput;
#[cfg(feature = "codegen")]
use syn::Error;
#[cfg(feature = "codegen")]
use syn::Item::{Enum, Struct};
use syn::{parse_quote, Attribute};

/// Generates `Optionable` and `OptionableConvert` implementation for structs/enums in
/// all `*.rs` files in the input folder.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input_dir: PathBuf,
    output_dir: PathBuf,
    /// Number of times to greet
    #[arg(long, default_value_t = false)]
    no_convert: bool,
}

#[cfg(feature = "codegen")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    create_dir_all(&args.output_dir)?;
    let mut type_header = Vec::new();
    if args.no_convert {
        type_header.push(parse_quote! {
            #[optionable(no_convert)]
        });
    }
    file_codegen(&args.input_dir, &args.output_dir, &type_header)
}
#[cfg(feature = "codegen")]
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

#[cfg(feature = "codegen")]
/// Calls the `optionable`-derive macro with the provided argument.
fn codegen(input: impl Into<DeriveInput>) -> Result<Option<String>, Error> {
    let result = optionable_codegen::derive_optionable(input.into())?;
    let file = syn::parse2(result)?;
    Ok::<_, Error>(Some(prettyplease::unparse(&file)))
}

#[cfg(not(feature = "codegen"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    panic!("This binary requires the `codegen` feature to be enabled");
}
