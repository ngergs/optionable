extern crate core;

use optionable_codegen::CodegenSettings;
use std::fs::create_dir_all;
use std::mem::take;
use std::path::Path;
use std::{fs, io};
use syn::Item::{Enum, Mod, Struct};
use syn::{Attribute, DeriveInput, Error, Item};

/// Read the respective `input_file` and calls codegen for it. The result is written
/// into the `output_path` under the same `file_name` as the input.
///
/// # Errors
/// - IO errors when reading the input files or writing the output files.
/// - Rust syntax in the input file.
/// - Codegen errors due to e.g. erroneous helper attributes.
pub fn file_codegen(
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
                        item_codegen(item, input_path, output_path, type_attrs, codegen_settings)
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .flatten()
                    .collect();
                Ok(vec![Mod(mod_entry)])
            } else {
                // include of a module from another file
                let mut codegen_settings = codegen_settings.clone();
                codegen_settings.ty_prefix = if let Some(mut ty_prefix) = codegen_settings.ty_prefix
                {
                    ty_prefix.segments.push(mod_entry.ident.clone().into());
                    Some(ty_prefix)
                } else {
                    Some(mod_entry.ident.clone().into())
                };
                let same_folder_mod_path = input_path.join(format!("{}.rs", mod_entry.ident));
                if same_folder_mod_path.exists() {
                    file_codegen(
                        &same_folder_mod_path,
                        output_path,
                        type_attrs,
                        &codegen_settings,
                    )?;
                } else {
                    let sub_folder_mod_path =
                        input_path.join(mod_entry.ident.to_string()).join("mod.rs");
                    file_codegen(
                        &sub_folder_mod_path,
                        &output_path.join(mod_entry.ident.to_string()),
                        type_attrs,
                        &codegen_settings,
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
    let result = optionable_codegen::derive_optionable(input.into(), Some(codegen_settings))?;
    syn::parse2(result).map(|f: syn::File| f.items)
}
