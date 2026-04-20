//! This crate is on its own not really usable. It is re-exported together with the required
//! traits in the [optionable](https://docs.rs/optionable/) crate. The relevant docs can be also found there.
//!
//! This is only a separate crate as derive macros have to be a separate crate.
use proc_macro::TokenStream;

/// Derive macro to derive the `Optionable` trait for structs/enums recursively by generating
/// a type with all fields recursively replaced with `Option` versions.
/// All non-required fields have to implement the `Optionable` trait. This trait is already implemented by this library
/// for many primitive types, wrapper and container types.
///
/// ### Type-level attributes (on the struct/enum level)
/// - **`optionable_attr`**: Helper for the `derive` type-level attribute, for details see the `derive` attribute.
/// - **`derive`**: Allows to specify derive attributes that should be attached to the generate optioned struct/enum.
///   If you need to forward additional helper attributes to the generated type use `attr_copy` or `optionable_attr`
///   with the attribute to forward as content (works for type and field attributes).
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(derive(Deserialize, Serialize))]
///   #[optionable_attr(serde(rename_all = "camelCase"))]
///   struct MyStruct{
///     #[optionable_attr(serde(rename = "firstName"))]
///     name: String,
///     surname: String,
///   }
///   ```
/// - **`attr_copy`**: Attributes that should be copied over the optioned type.
///   If no `key` filter is set all attributes with the given path are copied over.
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(derive(Deserialize, Serialize))]
///   #[optionable(attr_copy(path=serde,key=rename))]
///   struct MyStruct{
///     name: String,
///     surname: String,
///   }
/// - **`no_convert`**: Does not derive the `OptionableConvert` implementation.
///   Might be required if the target type involves smart pointers or unsized fields.
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(no_convert)]
///   struct MyStruct{}
///   ```
/// - **`option_wrap`**: Whether to wrap a field that is already an `Option<T>` inside an outer `Option<Option<T>>` for the optioned type.
///   Useful for struct patching but not helpful once serializing to e.g. JSON is involved.
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(option_wrap)]
///   struct MyStruct{}
///   ```
/// - **`suffix`**: The name of the generated optioned struct/enum will be `<original><suffix>` with suffix
///   defaulting to `"Opt"`. The suffix value can be adjusted via e.g. `#[optionable(suffix="Ac")]`.
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(suffix="Ac")]
///   struct MyStruct{}
///   ```
///
/// ### Field-level attributes (for structs and struct-typed enum variants)
/// - **`optionable_attr`**: Helper for the `derive` type-level attribute, for details see the `derive` attribute.
/// - **`required`**: The annotated field will be kept as is and won't be transformed into some optional variant
///   for the derived optioned Struct.
///   ```rust,ignore
///   #[derive(optionable)]
///   struct MyStruct{
///     street: String; // will be an `Option<String>` in the derived `MyStructOpt`.
///     #[optionable(required)]
///     number: u32; // will also be a u32 in the derived `MyStructOpt`.
///   }
///   ```
/// - **`optioned_type`**: For circumvention of the orphan rule. Uses the provided type for the optioned variant of the field.
///   If `no_convert` is not set the provided type has to implement `OptionedConvert<O>` where `O` is the type of this field in the original type.
///   ```rust,ignore
///   #[derive(optionable)]
///   struct MyStruct{
///     street: String; // will be an `Option<String>` in the derived `MyStructOpt`.
///     #[optionable(optioned_type=MyInt)]
///     number: i32; // will be a `MyInt` in the derived `MyStructOpt`.
///   }
///   ````
/// - **`merge`**: Customize the merge behaviour for the derived `OptionableConvert::merge` implementation. The following values are supported (e.g. as `#[merge(atomic)]`):
///     - `optionable_convert` Default behavior, just call `OptionableConvert::merge` for the given field entry.
///     - `atomic` Always override the field completly when present in the optioned type.
///     - `append_not_present` Kubernetes `set` merge logic. Appends all entries that are not already present, requires for the field type to `impl Extend<T> + IntoIterator<Item=T> where T: PartialEq`.
///     - `iter_map` Kubernetes `map` merge logic. Identifies if entries belong together via the `optionable::merge::OptionableMapKeysEq` trait. Merges entries that belong together (short-circuit, first match is merged) and appends all entries that are not already present, requires for the field type to `impl Extend<T> + IntoIterator<Item=T> where T: OptionableMapKeysEq + OptionableConvert`.
/// - **`merge_map_key`**: Implies `required` (see above). Furthermore, if at least one field has this attribute the trait `OptionableMapKeysEq` is implemented
///   which uses the corresponding fields to determine if the "mapping keys"  of two elements are equal. If the given struct is `T` this becomes utilized if another struct has a field
///   with the `merge(map)` attribute set and has a type that `impl IntoIterator<Item=T>`.
///   ```rust,ignore
///   #[derive(optionable)]
///   struct EnvVar{
///     #[merge_map_key]
///     street: String; // will be a `String` in the derived `EnvVarOpt` and used as a merge key for the merge logic of `Container` below.
///     value: String; // will be an `Option<String>` in the derived `EnvVarOpt`.
///   }
///
///   #[derive(optionable)]
///   struct Container{
///     #[merge(iter_map)]
///     env: Vec<EnvVar>; // will be a `Option<Vec<EnvVarOpt>>` in the derived `ContainerOpt`. The `OptionableConvert::merge` logic will merge entries that share the same keys (here the `name`) short-circuiting together and append those which did not find a merge target.
///   }
///   ````
///
/// ### Kubernetes type-level attributes (on the struct/enum level)
/// Specialized Kubernetes specific type attributes to help deriving optioned types (in go called `ApplyConfiguration`)
/// for Kubernetes resources/subfields.
///
/// - **`kube`**: Intended to be placed on types implementing subfields of a kube CRD spec.
///   Copies over any `#[(serde(rename="...")]`,`#[(serde(rename_all="...")]` and `#[(serde(rename_all_fields="...")]` attributes over to the optioned types.
///   ```rust,ignore
///   #[optionable(kube())]
///   #[derive(Optionable, Clone, Debug, Deserialize, Serialize, JsonSchema)]
///   pub struct MyCrdSpecTemplate {
///     pub replicas: u32,
///   }
///   ```
///
///   If you need to configure optionable explicitly setting this is e.g. for a Struct equivalent to:
///   ```rust,ignore
///   #[optionable(
///     attr_copy(path=serde,key=rename)
///     attr_copy(path=serde,key=rename_all)
///     attr_copy(path=serde,key=rename_all_fields)
///   )]
///   ```
///
/// - **`kube(resource)`**: Very specialized version of the `kube()` attribute intended to be placed on the root element of a `kube::CustomResource`.
///   Besides forwarding the serde rename attribute like `kube()` it also adds the following behaviour:
///     - Marks the `.metadata` field as required for the optioned type.
///     - Derives `kube::core::Resource` for the optioned type by referencing the original root `kube::CustomResource`.
///     - Adds specialized serde Serializing/Deserializing behaviour to the optioned type to handle the Group-Version-Kind envelope used by Kubernetes
///       (utilizing a [`PhantomData`-Marker](https://docs.rs/optionable/latest/optionable/kube/fn.serialize_api_envelope.html) on the optioned root type).
///    ```rust,ignore
///    #[derive(CustomResource)]
///    #[kube(
///        group = "example.localhost",
///        version = "v1",
///        kind = "CustomCrd",
///        namespaced,
///        derive = "Optionable",
///        attr = "optionable(kube(resource),derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize))",
///    )]
///    ```
#[proc_macro_derive(Optionable, attributes(optionable, optionable_attr))]
pub fn derive_optionable(input: TokenStream) -> TokenStream {
    try_derive_optionable(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into())
}

/// Internal method for `derive_optionable` to simplify error handling.
fn try_derive_optionable(input: TokenStream) -> Result<TokenStream, syn::Error> {
    Ok(optionable_codegen::derive_optionable(syn::parse2(input.into())?, None)?.into())
}

/// Derive macro to derive the `DeepMerge` trait from `k8s_openapi` for structs/enums.
/// All fields have to implement the `DeepMerge` trait.
/// This trait is already implemented `k8s_openapi` for many primitive types, wrapper and container types.
///
/// ### Type-level attributes (on the struct/enum level)
/// - **`crate_k8s_openpi`**: Crate name for `k8s_openapi` where `DeepMerge` can be found. Defaults to `k8s_openapi`.
/// - **`crate_optionable`**: Crate name for `optionable` where helper functions used by the derived implentations are located.
///   Defaults to `optionable`.
///
/// ### Field-level attributes (for structs and struct-typed enum variants)
/// - **`method`**: Customize the merge behaviour. The following values are supported (e.g. as `#[deepmerge(atomic)]`):
///     - `deepmerge` Default behavior, just call `Deepmerge::merge_from` for the given field entry.
///     - `atomic` Always override the field completly when present in the optioned type.
///     - `append_not_present` Kubernetes `set` merge logic. Appends all entries that are not already present, requires for the field type to `impl Extend<T> + IntoIterator<Item=T> where T: PartialEq`.
///     - `iter_map` Kubernetes `map` merge logic. Identifies if entries belong together via the `optionable::merge::OptionableMapKeysEq` trait. Merges entries that belong together (short-circuit, first match is merged) and appends all entries that are not already present, requires for the field type to `impl Extend<T> + IntoIterator<Item=T> where T: OptionableMapKeysEq + OptionableConvert`.
///
#[proc_macro_derive(DeepMerge, attributes(deepmerge))]
pub fn derive_deepmerge(input: TokenStream) -> TokenStream {
    try_derive_deepmerge(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into())
}

/// Internal method for `derive_optionable` to simplify error handling.
fn try_derive_deepmerge(input: TokenStream) -> Result<TokenStream, syn::Error> {
    Ok(optionable_codegen::derive_deepmerge(syn::parse2(input.into())?)?.into())
}

/// Derive macro to derive the `MapKeysEq` trait from `optionabe` for structs.
///
/// ### Type-level attributes (on the struct level)
/// - **`crate_optionable`**: Crate name for `optionable`.  Defaults to `optionable`.
///
/// ### Field-level attributes
/// - **`map_key`**: Marks a field as being a `map_key` that has to be equal on two structs for `MapKeysEq::map_keys_eq` to be true. The corresponding field has to `impl PartialEq`.
#[proc_macro_derive(MapKeysEq, attributes(map_key))]
pub fn derive_map_keys_eq(input: TokenStream) -> TokenStream {
    try_derive_map_keys_eq(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into())
}

/// Internal method for `derive_optionable` to simplify error handling.
fn try_derive_map_keys_eq(input: TokenStream) -> Result<TokenStream, syn::Error> {
    Ok(optionable_codegen::derive_map_keys_eq(syn::parse2(input.into())?)?.into())
}
