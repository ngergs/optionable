#[cfg(feature = "codegen")]
use quote::ToTokens;
#[cfg(feature = "codegen")]
use std::env::args;
#[cfg(feature = "codegen")]
use std::fs;
#[cfg(feature = "codegen")]
use std::fs::create_dir_all;
#[cfg(feature = "codegen")]
use syn::Error;
#[cfg(feature = "codegen")]
use syn::Item::{Enum, Struct};

#[cfg(feature = "codegen")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args();
    if args.len() != 3 {
        return Err("wrong number of arguments. Expected: <target-dir> <output-dir>".into());
    }
    let files = fs::read_dir(args.nth(1).ok_or("missing input file argument")?)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|file| file.file_name().to_string_lossy().ends_with(".rs"));
    let output_dir = args.next().ok_or("missing output directory argument")?;
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
        create_dir_all(&output_dir)?;
        fs::write(
            format!("{output_dir}/{}", file.file_name().to_string_lossy()),
            result,
        )?;
    }
    Ok(())
}

#[cfg(feature = "codegen")]
/// Calls the `optionable`-derive macro with the provided argument.
fn codegen(input: impl ToTokens) -> Result<Option<String>, Error> {
    let result = optionable_derive2::derive_optionable(input.into_token_stream())?;
    let file = syn::parse2(result)?;
    Ok::<_, Error>(Some(prettyplease::unparse(&file)))
}

#[cfg(not(feature = "codegen"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    panic!("This binary requires the `codegen` feature to be enabled");
}
