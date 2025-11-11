//! Library to derive `optioned` structs/enums versions of existing types where all fields have been recursively
//! replaced with versions that support setting just a subset of the relevant fields (or none at all).
//!
//! One motivation for this concept is the common problem when expressing patches that for a given rust struct `T`
//! a corresponding struct `T::Optioned` would be required where all fields are recursively optional to specify.
//! While trivial to write for plain structures this quickly becomes tedious for nested structs/enums.
//!
//! ## Kubernetes server-side-apply
//! Examples for the usage of this library for type-safe Kubernetes server-side-apply in Rust can be found [here](https://github.com/ngergs/optionable/tree/main/example/k8s).
//!
//! The library allows to use server-side-apply with built-in Kubernetes types by providing optioned variants for all types
//! from [k8s-openapi](https://crates.io/crates/k8s-openapi). It also provides tooling to derive optioned variants for
//! `kube::CustomResource` implementations via an attribute macro. Furthermore, it also provides [`extract`](kube::ExtractManagedFields)-functionality
//! to get from a Kubernetes resource a partial representation with only the owned fields remaining.
//!
//! For detailed documentation, see the documentation in [`kube`] for the CRD use case and the [examples](https://github.com/ngergs/optionable/tree/main/example/k8s).
//!
//! # Deriving optional structs/enums
//!
//! The core utility of this library is to provide an [`derive@Optionable`]-derive macro that derives such an optioned type
//! and implements the corresponding [`trait@Optionable`]-trait. It supports nested structures, enums as well as various container types.
//!
//! For detailed configuration options via helper attributes, see the [`derive@Optionable`]-derive macro docs.
//!
//! The general logic is the same as for other rust derives, If you want to use the [`derive@Optionable`]-derive macro for a struct/enum
//! every field of it needs to also have implemented the corresponding [`trait@Optionable`] trait:
//!```rust
//! # use optionable::Optionable;
//! # use serde::{Serialize,Deserialize};
//! #
//! #[derive(Optionable)]
//! #[optionable(derive(Default,Serialize,Deserialize))]
//! struct Address {
//!     street_name: String,
//!     number: u8,
//! }
//!
//! let _ = AddressOpt{
//!    street_name: Some("a".to_owned()),
//!    // fill the other fields with `None`
//!    ..Default::default()
//! };
//! ```
//!
//! The generated optioned types are (shown here with resolved associated types) as follows. They can be also referenced as
//! `Address::Optioned` and `AddressEnum::Optioned` respectively.
//! ```rust
//! # use serde::{Serialize,Deserialize};
//! #
//! #[derive(Default,Serialize,Deserialize)]
//! struct AddressOpt {
//!     street_name: Option<String>,
//!     number: Option<u8>,
//! }
//! ```
//!
//! ### Enum support
//! Deriving optioned versions also works with enums:
//! ```rust
//! # use optionable::Optionable;
//! #
//! # #[derive(Optionable)]
//! # struct Address {
//! #     street_name: String,
//! #     number: u8,
//! # }
//! #
//! #[derive(Optionable)]
//! enum AddressEnum {
//!      Plain(String),
//!      AddressExplicit { street: String, number: u32 },
//!      AddressNested(Address)
//! }
//!
//! fn example(){
//!     let _ = AddressEnumOpt::AddressExplicit{
//!         street: Some("a".to_owned()),
//!         number: None
//!     };
//! }
//! ```
//!
//! # Core concept
//! The main [`trait@Optionable`] trait is quite simple:
//! ```rust
//! pub trait Optionable {
//!     type Optioned;
//! }
//! ```
//! It is a marker trait that allows to express for a given type `T` which type should be considered its `T::Optioned` type
//! such that `Option<T::Optioned>` would represent all variants of partial completeness.
//! For types without inner structure this means that the `Optioned` type will just resolve to the type itself, e.g.
//! ```rust,ignore
//! impl Optionable for String {
//!     type Optioned = String;
//! }
//! ```
//! For many primitive types as well as common wrapper or collection types the [`trait@Optionable`]-trait is already implemented.
//!
//! ## Conversion
//! Per default also conversion traits for struct/enums with sized fields will be derived.
//! The relevant traits are [`OptionableConvert`] which is an extension trait for sized-fields only [`trait@Optionable`]
//! objects. From this trait the sealed convenience trait [`OptionedConvert`] is auto-implemented
//! for the optioned object.
//! They are (shown here without comments and `where` clauses):
//! ```rust,ignore
//! pub trait OptionableConvert: Sized + Optionable {
//!     fn into_optioned(self) -> Self::Optioned;
//!     fn try_from_optioned(value: Self::Optioned) -> Result<Self, Error>;
//!     fn merge(&mut self, other: Self::Optioned) -> Result<(), Error>;
//! }
//!
//! // sealed, auto-implemented from `OptionableConvert` for every respective `T::Optioned`
//! pub trait OptionedConvert<T>
//! where
//!     T: Optionable<Optioned=Self> + OptionableConvert,
//! {
//!     fn from_optionable(value: T) -> Self;
//!     fn try_into_optionable(self) -> Result<T, Error>;
//! }
//! ```
//!
//! # Crate features
//! - `derive`: Default-feature, re-exports the [`derive@Optionable`] derive macro.
//! - `std`: Default-feature. Adds `Optionable`-implementations for many [std](https://doc.rust-lang.org/std/)-lib types.
//! - `alloc`: Adds `Optionable`-implementations for [alloc](https://doc.rust-lang.org/alloc/) types (only useful when not enabling the `std` feature).
//! - `chrono`: Derive [`trait@Optionable`] for types from [chrono](https://docs.rs/chrono/latest/chrono/).
//! - `serde_json`: Derive [`trait@Optionable`] for [serde_json](https://docs.rs/serde_json/latest/serde_json/)`::Value`.
//! - `k8s_openapi_v1_(30..=34)`: Adds `Optionable`-implementations for all [k8s-openapi](https://docs.rs/k8s-openapi/latest/k8s_openapi) types. Only on feature version, e.g. `k8s_openapi_v1_34` may be enabled at once.
//! - `k8s_openapi_convert`: Adds `OptionableConvert`-implementations for all optioned [k8s-openapi](https://docs.rs/k8s-openapi/latest/k8s_openapi) types specified by the `k8s_openapi_v1_(30..=34)` feature.
//! - `kube`: Tooling to derive optioned types for [kube](https://github.com/kube-rs/kube) `CustomResource`. Also includes [`extract`](kube::ExtractManagedFields)-functionality for server-side apply.
//!
//! # Limitations
//!
//! ## External types
//! Due to the orphan rule the usage of the library becomes cumbersome if one has a use case which heavily relies on crate-external types.
//! For well-established libraries adding corresponding `impl` to this crate (feature-gated) would be a worthwhile approach.
//!
//! ## Resolving associated types
//! Due to the use of associated types some IDE-hints do not fully resolve the associated types leaving you with
//! `<i32 as Optionable>::Optioned` instead of `i32`. Luckily, for checking type correctness and also for error messages
//! when using wrong types the associated types are resolved.
//!
//! For the derived `Optioned`-structs/enums a related issue is that other derive macros for those derived types won't see the resolved
//! associated types. Therefore, corresponding type bounds have to be added (done by the `Optionable`-derive) to the `Optioned`-structs/enums:
//! ```rust
//! # use optionable::Optionable;
//! # use serde::Serialize;
//! #
//! #[derive(Optionable)]
//! #[optionable(derive(Serialize))]
//! struct DeriveExample<T> {
//!     name: T,
//! }
//!
//! // The generated code for the struct is shown below (name adjusted and simplified)
//! #[derive(Serialize)]
//! struct DeriveExampleOpt2<T>
//! where
//!     T: Optionable,
//!     // extra `Serialize` bound on the struct level
//!     <T as Optionable>::Optioned: Sized + Serialize,
//! {
//!     #[serde(skip_serializing_if = "Option::is_none")]
//!     name: Option<<T as Optionable>::Optioned>
//! }
//! ```
//!
//! # Similar crates
//! One crate with similar scope is [optional_struct](https://crates.io/crates/optional_struct).
//! It focuses specifically on structs (not enums) and offers a more manual approach, especially in respect to nested sub-struct,
//! providing many fine-grained configuration options.
//!
//! Another crate is [struct-patch](https://crates.io/crates/struct-patch).
//! It focuses on patching structs (not enums), especially from serde inputs. Nesting is supported with manual helper annotations.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;
extern crate core;

use core::fmt::{Display, Formatter};
#[cfg(feature = "derive")]
#[doc(inline)]
pub use optionable_derive::Optionable;

mod optionable;

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(any(
    feature = "k8s_openapi_v1_30",
    feature = "k8s_openapi_v1_31",
    feature = "k8s_openapi_v1_32",
    feature = "k8s_openapi_v1_33",
    feature = "k8s_openapi_v1_34"
))]
pub mod k8s_openapi;
#[cfg(feature = "kube")]
pub mod kube;

#[cfg(feature = "serde_json")]
mod serde_json;

/// Marker trait that associated this type with a corresponding type where potential
/// inner sub-fields are recursively optional if possible for the given use case of the type.
/// Implementations of the trait can decide that some fields are also non-optional for the optioned type.
///
/// In detail this means that an `Option<T::Optioned>` should allow for every combination
/// of itself being set as well as just partial subfields of itself being set except
/// for fields that are always required.
/// Hence, for types without inner structure like `i32` the `Optioned` type will resolve to itself,
/// as e.g. `Option<i32>` already expresses the needed granularity.
pub trait Optionable {
    /// The associated type where fields (if possible for the given use case) are recursively optional.
    type Optioned: ?Sized;
}

/// Extension trait for sized [`trait@Optionable`] to transform in and from optioned objects as well as merging.
pub trait OptionableConvert: Sized + Optionable {
    /// Transforms this object into an optioned variant which all fields set.
    ///
    /// We cannot implement `Into` from the stdlib as we need to implement this
    /// for various stdlib primitives and containers.
    fn into_optioned(self) -> Self::Optioned;

    /// Try to build this full type from its optioned variant.
    ///
    /// We cannot implement `TryFrom` from the stdlib as we need to implement this
    /// for various stdlib primitives and containers.
    ///
    /// # Errors
    /// - If fields required by the full type are not set.
    fn try_from_optioned(value: Self::Optioned) -> Result<Self, Error>;
    /// Merge the optioned values into this full type. List-like types are overwritten if set in `other`.
    /// Maps are merged per key.
    ///
    /// # Errors
    /// - There are scenarios where the full type allows some missing fields but the optioned type
    ///   also does not hold enough subfields to constructs a full entry with the respective `try_from`.
    ///   An example would be a field with type `Option<T>` and value `None` for `self` and type `Option<T::Optioned>`
    ///   and `Some` value for `other`. The `T::try_from(T::Optioned)` can fail is fields are missing for this subfield.
    fn merge(&mut self, other: Self::Optioned) -> Result<(), Error>;
}

/// Prevent implementation outside of this crate.
trait OptionedConvertSealed<T> {}

/// Sealed helper trait to transform from the perspective of the optioned type.
/// Will be automatically implemented for every target `<T as Optionable>:Optioned`.
#[allow(private_bounds)]
pub trait OptionedConvert<T>: Sized + OptionedConvertSealed<T>
where
    T: Optionable<Optioned = Self> + OptionableConvert,
{
    /// Gets an optioned variant with all fields set.
    ///
    /// We cannot implement `From` from the stdlib as we need to implement this
    /// for various stdlib primitives and containers.
    fn from_optionable(value: T) -> Self {
        value.into_optioned()
    }

    /// Try to build a full type from this optioned variant.
    /// # Errors
    /// - If fields required by the full type are not set.
    fn try_into_optionable(self) -> Result<T, Error> {
        T::try_from_optioned(self)
    }
}

impl<T, TOpt> OptionedConvertSealed<T> for TOpt where
    T: Optionable<Optioned = TOpt> + OptionableConvert
{
}
impl<T, TOpt> OptionedConvert<T> for TOpt where T: Optionable<Optioned = TOpt> + OptionableConvert {}

/// Represents errors that occur when trying to build a full type from its optioned variant.
#[derive(Debug)]
pub struct Error {
    /// Field that is missing
    pub missing_field: &'static str,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "The following required field is missing: {:?}",
            self.missing_field
        )
    }
}

impl core::error::Error for Error {}
