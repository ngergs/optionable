//! This crate is on its own not really usable. It is re-exported together with the required
//! traits in the [optionable](https://docs.rs/optionable/) crate. The relevant docs can be also found there.
//!
//! This is only a separate crate as derive macros have to be a separate crate.
use proc_macro::TokenStream;

/// Derive macro to derive the `Optionable` trait for structs/enums recursively. All non-required
/// fields have to implement the `Optionable` trait. This trait is already implemented by this library
/// for many primitive types, wrapper and container types.
///
/// ### Type-level attributes (on the struct/enum level)
/// - **`derive`**: Allows to specify derive attributes that should be attached to the generate optioned struct/enum.
///   Example:
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(derive(Deserialize, Serialize))]
///   struct MyStruct{}
///   ```
/// - **`no_convert`**: Does not derive the `OptionableConvert` implementation.
///   Might be required if the target type involves smart pointers or unsized fields.
///   Example:
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(no_convert)]
///   struct MyStruct{}
///   ```
/// - **`suffix`**: The name of the generated optioned struct/enum will be `<original><suffix>` with suffix
///   defaulting to `"Opt"`. The suffix value can be adjusted via e.g. `#[optionable(suffix="Ac")]`.
///   Example:
///   ```rust,ignore
///   #[derive(optionable)]
///   #[optionable(suffix="Ac")]
///   struct MyStruct{}
///   ```
/// ### Field-level attributes (for structs and struct-typed enum variants)
/// - **`required`**: The annotated field will be kept as is and won't be transformed into some optional variant
///   for the derived optioned Struct.
///   Example:
///   ```rust,ignore
///   #[derive(optionable)]
///   struct MyStruct{
///     street: String; // will be an `Option<String>` in the derived `MyStructOpt`.
///     #[optionable(required)]
///     number: u32; // will also be a u32 in the derived `MyStructOpt`.
///   }
///   ```
#[proc_macro_derive(Optionable, attributes(optionable))]
pub fn derive_optionable(input: TokenStream) -> TokenStream {
    try_derive_optionable(input).unwrap_or_else(|e| syn::Error::into_compile_error(e).into())
}

/// Internal method for `derive_optionable` to simplify error handling
fn try_derive_optionable(input: TokenStream) -> Result<TokenStream, syn::Error> {
    Ok(optionable_codegen::derive_optionable(syn::parse2(input.into())?)?.into())
}
