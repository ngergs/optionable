#[cfg(feature = "codegen")]
use std::env::args;
#[cfg(feature = "codegen")]
use std::fs;
#[cfg(feature = "codegen")]
use std::fs::create_dir_all;
use std::path::Path;
#[cfg(feature = "codegen")]
use syn::DeriveInput;
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
    let output_path_input = args
        .next()
        .ok_or("missing output directory argument")?
        .to_string();
    let output_path: &Path = Path::new(&output_path_input);
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
        create_dir_all(&output_path)?;
        fs::write(output_path.join(file.file_name()), result)?;
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
