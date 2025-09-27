use clap::Parser;
#[cfg(feature = "codegen")]
use std::fs;
#[cfg(feature = "codegen")]
use std::fs::create_dir_all;
use std::path::PathBuf;
#[cfg(feature = "codegen")]
use syn::DeriveInput;
#[cfg(feature = "codegen")]
use syn::Error;
#[cfg(feature = "codegen")]
use syn::Item::{Enum, Struct};

/// Generates `Optionable` and `OptionableConvert` implementation for structs/enums in
/// all `*.rs` files in the input folder.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input_dir: PathBuf,
    output_dir: PathBuf,
}

#[cfg(feature = "codegen")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    create_dir_all(&args.output_dir)?;
    let files = fs::read_dir(args.input_dir)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|file| file.file_name().to_string_lossy().ends_with(".rs"));
    for file in files {
        let content_str = fs::read_to_string(file.path())?;
        let content = syn::parse_file(&content_str)?;
        let result = content
            .items
            .into_iter()
            .map(|item| match item {
                Struct(item) => Ok::<_, Error>(codegen(item)?),
                Enum(item) => Ok::<_, Error>(codegen(item)?),
                _ => Ok(None),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join("\n\n");
        fs::write(args.output_dir.join(file.file_name()), result)?;
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
