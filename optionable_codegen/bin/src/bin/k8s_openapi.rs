use clap::Parser;
use darling::FromMeta;
use optionable_codegen::{CodegenSettings, derive_deepmerge, derive_map_keys_eq};
use optionable_codegen_cli::{
    CodegenConfig, CodegenVisitor, ListType, MapType, OpenApiListExtensions,
    determine_list_map_keys, file_codegen,
};
use proc_macro2::{Ident, Span};
use quote::{ToTokens, quote};
use std::collections::HashSet;
use std::default::Default;
use std::fs::create_dir_all;
use std::path::PathBuf;
use syn::Item::{Enum, Impl, Struct, Use};
use syn::{DeriveInput, Fields, Item, Path, Type, parse_quote};

/// Generates `Optionable` and `OptionableConvert` implementation for structs/enums in
/// the referenced `input_file` and all included internal modules recursively.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file.
    #[arg(long, value_name = "input-file")]
    input_file: PathBuf,
    /// K8s openapi spec, should point to a checked out version of <https://github.com/kubernetes/kubernetes/blob/master/api/openapi-spec/v3>
    #[arg(long, value_name = "k8s-openapi-v3-dir")]
    k8s_openapi_v3_dir: PathBuf,
    /// Output directory
    #[arg(long, value_name = "output-dir")]
    output_dir: PathBuf,
    /// package name for the generated package, e.g. `k8s-openapi027`
    #[arg(long, value_name = "package-name")]
    package_name: String,
}

/// Visitor for the `k8s-openapi` optionable codegen.
struct Visitor<'a> {
    /// The type suffix for the optioned type. Here this is fixed to "Ac".
    optioned_suffix: &'static str,
    /// Field prefix of the content of currently processed file, e.g. `api.core.v1`
    field_prefix: Option<String>,
    // The mapping from the k8s field identifier with the `io.k8s.`-prefix removed (e.g. `api.core.v1.Container.env_from`)
    // to the rust field names (rustized using the k8s-openapi mapping) for th list-map-keys logic.
    list_extensions: &'a OpenApiListExtensions,
    /// Additional attributes that should be added to input structs/enums.
    has_resource_impl: HashSet<Ident>,
    has_metadata_impl: HashSet<Ident>,
    current_has_map_keys: bool,
}

impl Clone for Visitor<'_> {
    fn clone(&self) -> Self {
        Self {
            optioned_suffix: self.optioned_suffix,
            list_extensions: self.list_extensions,
            field_prefix: self.field_prefix.clone(),
            // we want to reset all other fields when entering a new module
            has_resource_impl: HashSet::new(),
            has_metadata_impl: HashSet::new(),
            current_has_map_keys: false,
        }
    }
}

impl CodegenVisitor for Visitor<'_> {
    fn visit_output_path(&mut self, path: &[&str]) {
        if path.len() > 1 {
            // first path element is the k8s openapi target version, e.g. `v1_31`
            // afterwards follows the k8s type, e.g. `api.core.v1`
            self.field_prefix = Some(path[1..].join("."));
        }
    }

    fn filter(&mut self, item: &Item) -> bool {
        // `WatchEvent` is the only externally tagged enum in k8s-openapi and two variations share a tag.
        // This would require us to implement custom serialization/deserialization.
        // As there is no obvious use case for an optioned WatchEvent it is omitted for now.
        // k8s docs: https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.34/#watchevent-v1-meta
        // k8s-openapi code: https://github.com/Arnavion/k8s-openapi/blob/master/src/v1_34/apimachinery/pkg/apis/meta/v1/watch_event.rs
        match item {
            Enum(item) => item.ident != "WatchEvent",
            Use(item_use) => !item_use
                .tree
                .to_token_stream()
                .to_string()
                .ends_with("WatchEvent"),
            _ => true,
        }
    }

    fn visit_pre_input(&mut self, item: &Item) {
        if let Impl(item) = item
            && let Some(trait_) = &item.trait_
            && let Type::Path(path) = &*item.self_ty
            && path.path.segments.len() == 1
        {
            if trait_.1 == parse_quote!(crate::Resource) || trait_.1 == parse_quote!(Resource) {
                self.has_resource_impl
                    .insert(path.path.segments[0].ident.clone());
            }
            if trait_.1 == parse_quote!(crate::Metadata) || trait_.1 == parse_quote!(Metadata) {
                self.has_metadata_impl
                    .insert(path.path.segments[0].ident.clone());
            }
        }
    }

    fn visit_input(&mut self, item: &mut Item) {
        let suffix = self.optioned_suffix;
        self.current_has_map_keys = false;
        match item {
            Struct(item) => {
                if let Some(field_prefix) = &self.field_prefix {
                    if let Some(list_keys) = self
                        .list_extensions
                        .list_map_keys
                        .get(&format!("{}.{}", field_prefix, item.ident))
                    {
                        for field in &mut item.fields {
                            if let Some(field_ident) = &field.ident
                                && list_keys.contains(&field_ident.to_string())
                            {
                                self.current_has_map_keys = true;
                                field.attrs.push(parse_quote!(#[optionable(merge_map_key)]));
                                field.attrs.push(parse_quote!(#[optionable_attr(map_key)]));
                            }
                        }
                    }
                    for field in &mut item.fields {
                        if let Some(field_ident) = &field.ident {
                            if let Some(merge_type) = self
                                .list_extensions
                                .list_type
                                .get(&format!("{}.{}.{}", field_prefix, item.ident, field_ident))
                            {
                                let merge_type = match merge_type {
                                    ListType::Atomic => quote!(atomic),
                                    ListType::Set => quote!(append_not_present),
                                    ListType::Map(_) => quote!(iter_map),
                                };
                                field
                                    .attrs
                                    .push(parse_quote!(#[optionable(merge(#merge_type))]));
                                field
                                    .attrs
                                    .push(parse_quote!(#[optionable_attr(deepmerge(method(#merge_type)))]));
                            } else if let Some(merge_type) = self
                                .list_extensions
                                .map_type
                                .get(&format!("{}.{}.{}", field_prefix, item.ident, field_ident))
                                .or(self
                                    .list_extensions
                                    .map_type
                                    .get(&format!("{}.{}", field_prefix, item.ident)))
                            {
                                println!("{:?}", self.list_extensions.map_type);
                                let merge_type_optionable = match merge_type {
                                    MapType::Atomic => Some(quote!(atomic)),
                                    MapType::Granular => None,
                                };
                                if let Some(merge_type_optionable) = merge_type_optionable {
                                    field.attrs.push(
                                        parse_quote!(#[optionable(merge(#merge_type_optionable))]),
                                    );
                                }
                                let merge_type_deepmerge = match merge_type {
                                    MapType::Atomic => quote!(atomic),
                                    MapType::Granular => quote!(granular),
                                };
                                field.attrs.push(
                                    parse_quote!(#[deepmerge(method(#merge_type_deepmerge))]),
                                );
                            }
                        }
                    }
                }

                let k8s_attr = match (
                    self.has_resource_impl.contains(&item.ident),
                    self.has_metadata_impl.contains(&item.ident),
                ) {
                    (true, true) => parse_quote!(#[optionable(k8s_openapi(resource,metadata))]),
                    (true, false) => parse_quote!(#[optionable(k8s_openapi(resource))]),
                    (false, true) => parse_quote!(#[optionable(k8s_openapi(metadata))]),
                    (false, false) => parse_quote!(#[optionable(k8s_openapi())]),
                };
                item.attrs.push(k8s_attr);
                item.attrs.push(parse_quote!(#[optionable(suffix=#suffix)]));
            }
            Enum(item) => {
                item.attrs.push(parse_quote!(#[optionable(k8s_openapi())]));
                item.attrs.push(parse_quote!(#[optionable(suffix=#suffix)]));
            }
            _ => {}
        }
    }

    fn visit_output(&mut self, items: &mut Vec<Item>) -> Result<(), syn::Error> {
        let mut extra_items = vec![];
        for item in items.iter_mut() {
            match item {
                Impl(item)
                    if item.trait_.as_ref().is_some_and(|trait_| {
                        trait_.1 == parse_quote!(crate::OptionableConvert)
                            || trait_
                                .1
                                .to_token_stream()
                                .to_string()
                                .starts_with("crate :: OptionedConvert") // to remove generic arguments
                    }) =>
                {
                    item.attrs
                        .push(parse_quote!(#[cfg(feature="k8s_openapi_convert")]));
                }
                Enum(item) => {
                    item.attrs.push(parse_quote!(#[serde(untagged)]));
                }
                Struct(item) => {
                    // addiitional DeepMerge and MapKeysEq derives
                    let mut derive_input: DeriveInput = item.clone().into();
                    // todo generalize k8s_openapi version
                    derive_input.attrs.extend(Some(parse_quote!(#[deepmerge(crate_k8s_openapi=k8s_openapi027,crate_optionable=crate)])));
                    let extra_impls = derive_deepmerge(derive_input)?;
                    extra_items.extend(syn::parse2(extra_impls).map(|f: syn::File| f.items)?);
                    if self.current_has_map_keys {
                        let mut derive_input: DeriveInput = item.clone().into();
                        // todo generalize k8s_openapi version
                        derive_input
                            .attrs
                            .extend(Some(parse_quote!(#[map_keys_eq(crate_optionable=crate)])));
                        let map_keys_impl = derive_map_keys_eq(&derive_input)?;
                        extra_items.extend(syn::parse2(map_keys_impl).map(|f: syn::File| f.items)?);
                    }
                    // remove #[map_item] and #[deepmerge] attribute from fields as we have now derived MapKeysEq and the helper would be unowned in the output
                    item.fields.iter_mut().for_each(|f| {
                        f.attrs.retain(|attr| {
                            let path = attr.path().to_token_stream().to_string();
                            path != "map_key" && path != "deepmerge"
                        });
                    });

                    // upstream k8s did not follow the usual pattern here (and likely now it's too late to change it)
                    if item.ident == "DaemonEndpointAc"
                        && let Fields::Named(fields) = &mut item.fields
                        && let Some(field) = fields
                            .named
                            .iter_mut()
                            .find(|field| field.ident.as_ref().is_some_and(|id| id == "port"))
                    {
                        field.attrs.push(parse_quote!(#[serde(rename = "Port")]));
                    }
                    // extra handling to support deserializing Quantity (wrapper around an inner string) from an int following upstream
                    if item.ident == "QuantityAc" {
                        item.attrs.push(parse_quote!(#[serde(from = "crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc")]));

                        extra_items.push(parse_quote! {
                        impl From<crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc> for QuantityAc {
                            fn from(value: crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc) -> Self {
                                QuantityAc(match value {
                                    crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc::Int(i) => i.map(|i| i.to_string()),
                                    crate::k8s_openapi::apimachinery::pkg::util::intstr::IntOrStringAc::String(s) => s,
                                })
                            }
                        }
                    });
                    }
                }
                _ => {}
            }
        }
        items.append(&mut extra_items);
        Ok(())
    }
}

/// Dedicated binary target for the purpose of codegen for `k8s-openapi`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let list_map_keys = determine_list_map_keys(&args.k8s_openapi_v3_dir)?;
    let package_name_ident = Ident::new(&args.package_name, Span::call_site());
    let codegen_settings = CodegenSettings {
        optionable_crate_name: Path::from_string("crate")?,
        ty_prefix: Some(Path::from_string(&args.package_name)?),
        input_crate_replacement: Some(package_name_ident.clone()),
    };
    create_dir_all(&args.output_dir)?;
    let optioned_suffix = "Ac";
    file_codegen(
        &args.input_file,
        &args.output_dir,
        CodegenConfig {
            visitor: Visitor {
                optioned_suffix,
                field_prefix: None,
                list_extensions: &list_map_keys,
                has_resource_impl: HashSet::default(),
                has_metadata_impl: HashSet::default(),
                current_has_map_keys: false,
            },
            optioned_suffix,
            settings: codegen_settings,
            usage_aliases: vec![],
            is_mod_private: false,
        },
    )
}
