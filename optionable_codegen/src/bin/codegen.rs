extern crate core;

#[cfg(feature = "codegen")]
mod codegen {
    use clap::Parser;
    use darling::FromMeta;
    use optionable_codegen::{attribute_derives, attribute_no_convert, CodegenSettings};
    use proc_macro2::{Ident, Span};
    use quote::ToTokens;
    use std::fs::create_dir_all;
    use std::mem::take;
    use std::path::{Path, PathBuf};
    use std::{fs, io};
    use syn::Item::{Enum, Mod, Struct};
    use syn::{Attribute, DeriveInput, Error, Item, Token, UseTree, Visibility};

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
        /// - Replaces the keyword `crate` in the input with the provided string value.
        /// - Prepends the provided string crate name and the respective package subpath to the `impl` definitions.
        /// - Uses in the generated code `crate` instead of `::optionable` to refer to the optionable crate.
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
            &vec![],
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
                Some(Ident::new(replace_crate_name, Span::call_site()));
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
        usage_aliases: &[(String, String)],
    ) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all(output_path)?;
        let content_str = fs::read_to_string(input_file)?;
        let content = syn::parse_file(&content_str)?;
        let input_path = input_file
            .parent()
            .ok_or("current file {input_file} has no parent")?;
        let mut usage_aliases: Vec<_> = usage_aliases.into();
        usage_aliases.append(&mut get_usage_aliases(&content.items));
        let result = content
            .items
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
            .map(|item| patch_impl_references(item, &usage_aliases))
            .collect::<Result<Vec<_>, _>>()?;
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
        usage_aliases: &[(String, String)],
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
                    usage_aliases.append(&mut get_usage_aliases(&items));
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
                        .map(|item| patch_impl_references(item, &usage_aliases))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(vec![Mod(mod_entry)])
                } else {
                    // include of a module from another file
                    let mut codegen_settings = codegen_settings.clone();
                    codegen_settings.ty_prefix =
                        if let Some(mut ty_prefix) = codegen_settings.ty_prefix {
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
                            usage_aliases,
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
                        )?;
                    }
                    Ok(vec![Mod(mod_entry)])
                }
            }
            _ => Ok(vec![]),
        }
    }

    /// Filter the items and returns usages of the form `use self::<...>` if they are `UseTree`s themselves.
    fn get_usage_aliases<'a>(items: impl IntoIterator<Item = &'a Item>) -> Vec<(String, String)> {
        items
            .into_iter()
            .filter_map(|item| {
                if let Item::Use(item) = item
                    && item.vis == Visibility::Public(Token![pub](Span::call_site()))
                    && let UseTree::Path(path) = &item.tree
                    && &path.ident == "self"
                    && let UseTree::Path(path) = *path.tree.clone()
                    && let Some(leaf) = leaf_element(&path.tree)
                {
                    // todo: less hacky?
                    let mut path_prefixed = ":: ".to_owned();
                    let path = path.to_token_stream().to_string();
                    path_prefixed.push_str(&path);
                    let mut leaf_prefixed = ":: ".to_owned();
                    leaf_prefixed.push_str(&leaf.to_token_stream().to_string());
                    Some((path_prefixed, leaf_prefixed))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Lead element of a `UseTree`. Does only support plain paths, returns None for unsupported types.
    fn leaf_element(tree: &UseTree) -> Option<Ident> {
        match tree {
            UseTree::Name(ident) => Some(ident.ident.clone()),
            UseTree::Path(path) => leaf_element(&path.tree),
            _ => None,
        }
    }

    /// Patches the object the `impl` references according to the provided aliases.
    fn patch_impl_references(
        item: Item,
        usage_aliases: &[(String, String)],
    ) -> Result<Item, Error> {
        if let Item::Impl(mut item) = item {
            let mut ty = item.self_ty.to_token_stream().to_string();
            for alias in usage_aliases {
                if ty.contains(&alias.0) {
                    ty = ty.replace(&alias.0, &alias.1);
                }
            }
            item.self_ty = syn::parse_str::<syn::Type>(&ty)?.into();
            Ok(Item::Impl(item))
        } else {
            Ok(item)
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "codegen"))]
    panic!("This binary requires the `codegen` feature to be enabled");
    #[cfg(feature = "codegen")]
    codegen::main()
}
