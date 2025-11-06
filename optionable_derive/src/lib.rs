//! This crate is on its own not really usable. It is re-exported together with the required
//! traits in the [optionable](https://docs.rs/optionable/) crate. The relevant docs can be also found there.
//!
//! This is only a separate crate as derive macros have to be a separate crate.
use proc_macro::TokenStream;
use syn::{parse_quote, DeriveInput};

/// Derive macro to derive the `Optionable` trait for structs/enums recursively by generating
/// a type with all fields recursively replaced with `Option` versions.
/// All non-required fields have to implement the `Optionable` trait. This trait is already implemented by this library
/// for many primitive types, wrapper and container types.
///
/// ### Type-level attributes (on the struct/enum level)
/// - **`optionable_attr`**: Helper for the `derive` type-level attribute, for details see the `derive` attribute.
/// - **`derive`**: Allows to specify derive attributes that should be attached to the generate optioned struct/enum.
///   If you need to forward additional helper attributes to the generated type use `optionable_attr`
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
/// - **`no_convert`**: Does not derive the `OptionableConvert` implementation.
///   Might be required if the target type involves smart pointers or unsized fields.
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(no_convert)]
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
///
/// ### Kubernetes type-level attributes (on the struct/enum level)
/// Specialized Kubernetes specific type attributes to help deriving optioned types (in go called `ApplyConfiguration`)
/// for Kubernetes resources/subfields.
///
/// - Derives for the optioned type `Clone, Debug, PartialEq, Serialize, Deserialize` and additionally for Structs `Default`.
///
///  - **`kube`**: Intended to be placed on types implementing subfields of a kube CRD spec.
/// ```rust,ignore
/// #[derive(Optionable, Clone, Debug, Deserialize, Serialize, JsonSchema)]
/// #[optionable(kube())]
/// pub struct MyCrdSpecTemplate {
///    pub replicas: u32,
/// }
/// ```
#[proc_macro_derive(Optionable, attributes(optionable, optionable_attr))]
pub fn derive_optionable(input: TokenStream) -> TokenStream {
    try_derive_optionable(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into())
}

/// Specialized derive macro to derive the `Optionable` trait for the root of a derived `kube::CustomResources`.
/// Should be derived for the `kube::CustomResource` spec which implicitly creates the root type.
/// Translates to `#[derive(Optionable)]` with the type attribute `#[optionable(kube(resource)]`.
///
/// - Derives for the optioned type `Clone, Debug, PartialEq, Serialize, Deserialize` and additionally for Structs `Default`.
/// - Sets the `metadata` field as required for the optioned type.
/// - Adds `apiVersion` and `kind` to the serialization output with values from the trait constants of the given `kube::Resource` implementation
/// - Derives `kube::Resource` for the optioned type.
///
/// ```rust,ignore
/// #[derive(Optionable, CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
/// #[kube(derive = "OptionableKubeCrd")]
/// #[optionable(kube())]
/// #[kube(group = "example.localhost", version = "v1", kind = "MyCrd", namespaced)]
/// pub struct MyCrdSpec {
///     pub msg: String,
///     pub template: MyCrdSpecTemplate,
/// }
/// ```
#[proc_macro_derive(OptionableKubeCrd)]
pub fn derive_optionable_kube_crd(input: TokenStream) -> TokenStream {
    try_derive_optionable_kube_crd(input)
        .unwrap_or_else(|e| syn::Error::into_compile_error(e).into())
}

/// Internal method for `derive_optionable_kube_crd` to simplify error handling.
fn try_derive_optionable_kube_crd(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let mut input: DeriveInput = syn::parse2(input.into())?;
    input
        .attrs
        .push(parse_quote!(#[optionable(kube(resource))]));
    Ok(optionable_codegen::derive_optionable(input, None)?.into())
}

/// Internal method for `derive_optionable` to simplify error handling.
fn try_derive_optionable(input: TokenStream) -> Result<TokenStream, syn::Error> {
    Ok(optionable_codegen::derive_optionable(syn::parse2(input.into())?, None)?.into())
}
