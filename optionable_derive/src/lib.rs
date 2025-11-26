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
/// - **`optioned_type`**: For circumvention of the orphan rule. Uses the provided type for the optioned variant of the field.
///   If `no_convert` is not set the provided type has to implement `OptionedConvert<O>` where `O` is the type of this field in the original type.
///   ```rust,ignore
///   #[derive(optionable)]
///   struct MyStruct{
///     street: String; // will be an `Option<String>` in the derived `MyStructOpt`.
///     #[optionable(optioned_type=MyInt)]
///     number: i32; // will be a `MyInt` in the derived `MyStructOpt`.
///   }
///   ```
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
