extern crate core;

use darling::FromMeta;
use optionable_codegen::CodegenSettings;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use std::fs::create_dir_all;
use std::mem::take;
use std::path::Path;
use std::{fs, io};
use syn::Item::{Enum, Mod, Struct, Use};
use syn::{
    parse_quote, DeriveInput, Error, Item, ItemEnum, ItemStruct, Token, UseTree, Visibility,
};

/// Used for callback actions when encountering specific elements.
/// The `CodegenVisitor` will be cloned value upon entering a new module.
/// Implementors can use this to reset some internal state that should not carry over to the new module.
/// Every function prefixed with `visit_pre` will visit all items of the current file
/// under consideration before the other visits (or the codegen) are called.
/// For every codegen item under consideration the `visit_input` fields will be called, then codegen
/// and subsequently `visit_output`. Visitor implementation can store state for a given item internally
/// and use it for the output, no other codegen input item will be called in between.
pub trait CodegenVisitor: Clone {
    /// Reads the existing input items, executed for all items prior to any other mutations.
    /// Useful to e.g. collect a list of `use ...` entries.
    fn visit_pre_input(&mut self, _: &Item) {}
    /// Mutates the input.
    fn visit_input(&mut self, _: &mut Item) {}
    /// Mutates the generated code. Will be called once after code generation.
    /// Implementors can also reset internal states tracking input here.
    fn visit_output(&mut self, _: &mut Vec<Item>, _: &CodegenSettings) {}
}

/// Represents the current codegen configuration
#[derive(Clone)]
pub struct CodegenConfig<V: CodegenVisitor> {
    /// Visitor for detailed field/type handling. See `CodegenVisitor` trait.
    pub visitor: V,
    /// Suffix that will be used for the optioned types. Will be used for `pub use ...` re-exports.
    pub optioned_suffix: &'static str,
    /// Settings that can be configured via the library settings functionality
    pub settings: CodegenSettings,
    /// Re-exported aliases for types resulting from `pub use <....>` statements.
    pub usage_aliases: Vec<syn::Path>,
    /// Whether the current module is private. Only considers those types for codegen that have
    /// a `usage_aliases` entry.
    pub is_mod_private: bool,
}

/// Read the respective `input_file` and calls codegen for it. The result is written
/// into the `output_path` under the same `file_name` as the input.
///
/// # Errors
/// - IO errors on reading the input file.
/// - IO errors on writing the output file.
/// - Codegen errors, e.g. due to misused helper attributes.
///
pub fn file_codegen<Vis: CodegenVisitor>(
    input_file: &Path,
    output_path: &Path,
    mut conf: CodegenConfig<Vis>,
) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(output_path)?;
    let content_str = fs::read_to_string(input_file)?;
    let content = syn::parse_file(&content_str)?;
    content.items.iter().for_each(|item| {
        conf.visitor.visit_pre_input(item);
    });
    let input_path = input_file
        .parent()
        .ok_or("current file {input_file} has no parent")?;
    conf.usage_aliases
        .append(&mut get_usage_aliases(&content.items)?);
    let result = content
        .items
        .into_iter()
        .map(|item| {
            if let Use(item_use) = &item
                && item_use.vis == Visibility::Public(Token![pub](Span::call_site()))
                && let UseTree::Path(path) = &item_use.tree
                && &path.ident == "self"
            {
                let mut item_use = item_use.clone();
                let leaf = get_use_tree_leaf_mut(&mut item_use.tree)?;
                if let UseTree::Name(name) = leaf {
                    let mut name_string = name.ident.to_string();
                    name_string.push_str(conf.optioned_suffix);
                    name.ident = Ident::from_string(&name_string)?;
                } else {
                    return Err(Error::new(Span::call_site(), format!("unsupported use tree leaf note, only supports plain `Name`, got {leaf:?}")).into());
                }
                item_use.attrs.push(parse_quote! {#[allow(unused_imports)]});
                return Ok(Some(vec![Use(item_use)]));
            }

            let settings = if let Struct(ItemStruct { ident, .. }) | Enum(ItemEnum { ident, .. }) = &item {
                if conf.is_mod_private
                    && !conf.usage_aliases
                    .iter()
                    .any(|p| p.segments.last().is_some_and(|last| last.ident == *ident))
                {
                     return Ok(None)
                }
                let mut settings = conf.settings.clone();
                if let Some(ty_prefix) = &mut settings.ty_prefix
                    && !conf.usage_aliases.is_empty()
                {
                    let ty_prefix_segments = ty_prefix.segments.clone();
                    let ty_prefix_tail = ty_prefix_segments.iter().rev();
                    'outer: for usage_alias in &conf.usage_aliases {
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
                // first option is filtering, second is whether we want to alter the settings
                Some(settings)
            } else { None };
            if let Some(settings) = settings {
                conf.settings = settings;
            }
            item_codegen(
                item,
                input_path,
                output_path,
                &mut conf,
            ).map(Some)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
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
fn item_codegen<V: CodegenVisitor>(
    mut item: Item,
    input_path: &Path,
    output_path: &Path,
    conf: &mut CodegenConfig<V>,
) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    conf.visitor.visit_input(&mut item);
    let mut result = match item {
        Struct(item) => Ok::<_, Box<dyn std::error::Error>>(derive_codegen(item, &conf.settings)?),
        Enum(item) => Ok::<_, Box<dyn std::error::Error>>(derive_codegen(item, &conf.settings)?),
        Mod(mut mod_entry) => {
            if let Some(content) = mod_entry.content.as_mut() {
                let items = take(&mut content.1);
                let mut usage_aliases: Vec<_> = conf.usage_aliases.clone();
                usage_aliases.append(&mut get_usage_aliases(&items)?);
                let mut conf = CodegenConfig {
                    usage_aliases,
                    ..conf.clone()
                };
                content.1 = items
                    .into_iter()
                    .map(|item| item_codegen(item, input_path, output_path, &mut conf))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .flatten()
                    .collect();
                Ok(vec![Mod(mod_entry)])
            } else {
                // include of a module from another file
                let mut settings = conf.settings.clone();
                settings.ty_prefix = if let Some(mut ty_prefix) = settings.ty_prefix {
                    ty_prefix.segments.push(mod_entry.ident.clone().into());
                    Some(ty_prefix)
                } else {
                    Some(mod_entry.ident.clone().into())
                };
                let is_mod_private =
                    mod_entry.vis != Visibility::Public(Token![pub](Span::call_site()));
                let conf = CodegenConfig {
                    settings,
                    is_mod_private,
                    ..conf.clone()
                };
                let same_folder_mod_path = input_path.join(format!("{}.rs", mod_entry.ident));
                if same_folder_mod_path.exists() {
                    file_codegen(&same_folder_mod_path, output_path, conf)?;
                } else {
                    let sub_folder_mod_path =
                        input_path.join(mod_entry.ident.to_string()).join("mod.rs");
                    file_codegen(
                        &sub_folder_mod_path,
                        &output_path.join(mod_entry.ident.to_string()),
                        conf,
                    )?;
                }
                Ok(vec![Mod(mod_entry)])
            }
        }
        _ => Ok(vec![]),
    }?;
    conf.visitor.visit_output(&mut result, &conf.settings);
    Ok(result)
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
