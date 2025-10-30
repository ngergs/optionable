extern crate core;

use darling::FromMeta;
use optionable_codegen::CodegenSettings;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use std::borrow::Cow;
use std::fs::create_dir_all;
use std::mem::take;
use std::path::Path;
use std::{fs, io};
use syn::Item::{Enum, Mod, Struct, Use};
use syn::{
    parse_quote, Attribute, DeriveInput, Error, Item, ItemEnum, ItemStruct, Token, UseTree,
    Visibility,
};

/// Read the respective `input_file` and calls codegen for it. The result is written
/// into the `output_path` under the same `file_name` as the input.
pub fn file_codegen(
    input_file: &Path,
    output_path: &Path,
    type_attrs: &Vec<Attribute>,
    codegen_settings: &CodegenSettings,
    usage_aliases: &[syn::Path],
    is_private: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(output_path)?;
    let content_str = fs::read_to_string(input_file)?;
    let content = syn::parse_file(&content_str)?;
    let input_path = input_file
        .parent()
        .ok_or("current file {input_file} has no parent")?;
    let mut usage_aliases: Vec<_> = usage_aliases.into();
    usage_aliases.append(&mut get_usage_aliases(&content.items)?);
    let result = content
            .items
            .into_iter()
            .filter_map(|item| {
                match &item {
                    Struct(ItemStruct { ident, .. }) | Enum(ItemEnum { ident, .. }) => {
                        if is_private
                            && !usage_aliases
                                .iter()
                                .any(|p| p.segments.last().is_some_and(|last| last.ident == *ident))
                        {
                            return None;
                        }
                        let mut codegen_settings = codegen_settings.clone();
                        if let Some(ty_prefix) = &mut codegen_settings.ty_prefix
                            && !usage_aliases.is_empty()
                        {
                            let ty_prefix_segments = ty_prefix.segments.clone();
                            let ty_prefix_tail = ty_prefix_segments.iter().rev();
                            'outer: for usage_alias in &usage_aliases {
                                let mut ty_prefix_tail = ty_prefix_tail.clone();
                                let usage_alias_tail = usage_alias.segments.iter().rev().skip(1);
                                for usage_alias_el in usage_alias_tail {
                                    if ty_prefix_tail
                                        .next()
                                        .is_some_and(|ty_tail_el| ty_tail_el != usage_alias_el)
                                    {
                                        continue 'outer;
                                    }
                                }
                                // we found a match, replace the ty_prefix with the reduced version
                                ty_prefix.segments = ty_prefix_tail
                                    .rev()
                                    .map(std::borrow::ToOwned::to_owned)
                                    .collect();
                            }
                        }
                        Some(Cow::Owned(codegen_settings))
                    }
                    Mod(_) | Use(_) => Some(Cow::Borrowed(codegen_settings)),
                    _ => None,
                }
                .map(|settings| (settings, item))
            })
            .map(|(codegen_settings, item)| {
                // copy over usage aliases
                if let Use(item_use) = &item
                    && item_use.vis == Visibility::Public(Token![pub](Span::call_site()))
                    && let UseTree::Path(path) = &item_use.tree
                    && &path.ident == "self"
                {
                    let mut item_use = item_use.clone();
                    let leaf = get_use_tree_leaf_mut(&mut item_use.tree)?;
                    if let UseTree::Name(name) = leaf {
                        let mut name_string=name.ident.to_string();
                        name_string.push_str("Opt");
                        name.ident=Ident::from_string(&name_string)?;
                    } else{
                        return Err(Error::new(Span::call_site(),format!("unsupported use tree leaf note, only supports plain `Name`, got {leaf:?}")).into())
                    }
                    item_use.attrs.push(parse_quote! {#[allow(unused_imports)]});
                    Ok(vec![Use(item_use)])
                } else {
                    item_codegen(
                        item,
                        input_path,
                        output_path,
                        type_attrs,
                        &codegen_settings,
                        &usage_aliases,
                    )
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
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
    usage_aliases: &[syn::Path],
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
                let mut usage_aliases: Vec<_> = usage_aliases.into();
                usage_aliases.append(&mut get_usage_aliases(&items)?);
                content.1 = items
                    .into_iter()
                    .map(|item| {
                        item_codegen(
                            item,
                            input_path,
                            output_path,
                            type_attrs,
                            codegen_settings,
                            &usage_aliases,
                        )
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
                let is_private =
                    mod_entry.vis != Visibility::Public(Token![pub](Span::call_site()));
                let same_folder_mod_path = input_path.join(format!("{}.rs", mod_entry.ident));
                if same_folder_mod_path.exists() {
                    file_codegen(
                        &same_folder_mod_path,
                        output_path,
                        type_attrs,
                        &codegen_settings,
                        usage_aliases,
                        is_private,
                    )?;
                } else {
                    let sub_folder_mod_path =
                        input_path.join(mod_entry.ident.to_string()).join("mod.rs");
                    file_codegen(
                        &sub_folder_mod_path,
                        &output_path.join(mod_entry.ident.to_string()),
                        type_attrs,
                        &codegen_settings,
                        usage_aliases,
                        is_private,
                    )?;
                }
                Ok(vec![Mod(mod_entry)])
            }
        }
        _ => Ok(vec![]),
    }
}

/// Filter the items and returns usages of the form `use self::<...>` if they are `UseTree`s themselves.
fn get_usage_aliases<'a>(
    items: impl IntoIterator<Item = &'a Item>,
) -> Result<Vec<syn::Path>, darling::Error> {
    items
        .into_iter()
        .filter_map(|item| {
            if let Use(item) = item
                && item.vis == Visibility::Public(Token![pub](Span::call_site()))
                && let UseTree::Path(path) = &item.tree
                && &path.ident == "self"
            {
                Some(syn::Path::from_string(
                    &path.tree.to_token_stream().to_string(),
                ))
            } else {
                None
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

/// Calls the `optionable`-derive macro with the provided `DeriveInput` argument.
fn derive_codegen(
    input: impl Into<DeriveInput>,
    codegen_settings: &CodegenSettings,
) -> Result<Vec<Item>, Error> {
    let result = optionable_codegen::derive_optionable(input.into(), Some(codegen_settings))?;
    syn::parse2(result).map(|f: syn::File| f.items)
}

/// Traverses the use tree to it's leaf and returns a mutable reference to the leaf node.
/// Only supports plain `UseTree::Path` at the moment.
fn get_use_tree_leaf_mut(tree: &mut UseTree) -> Result<&mut UseTree, Error> {
    match tree {
        tree @ UseTree::Name(_) => Ok(tree),
        UseTree::Path(path) => get_use_tree_leaf_mut(&mut path.tree),
        _ => Err(Error::new(
            Span::call_site(),
            format!(
                "unsupported `UseTree` type, only `Path` is supported at the moment, got {tree:?}"
            ),
        )),
    }
}
