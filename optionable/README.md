# optionable

A rust library to derive `optioned` structs/enums versions of existing types where all fields have been recursively
replaced with versions that support setting just a subset of the relevant fields (or none at all).

One motivation for this concept is the common problem when expressing patches that for a given rust struct `T`
a corresponding struct `T::Optioned` would be required where all fields are recursively optional to specify.
While trivial to write for plain structures this quickly becomes tedious for nested structs/enums.

#### Links

- [crates.io](https://crates.io/crates/optionable)
- [docs.rs documentation](https://docs.rs/optionable/)

## Kubernetes server-side-apply

The library allows to use server-side-apply with built-in Kubernetes types by providing optioned variants for all types
from [k8s-openapi](https://crates.io/crates/k8s-openapi). It also provides tooling to derive optioned variants for
`kube::CustomResource` implementations.

For detailed documentation, see the documentation
in [kube3 module](https://docs.rs/optionable/latest/optionable/kube3/index.html)
for the CRD use case and the [examples](https://github.com/ngergs/optionable/tree/main/example/k8s).

## Deriving optional structs/enums

The core utility of this library is to provide an [`Optionable`](https://docs.rs/optionable/latest/optionable/derive.Optionable.html) derive macro that derives such an
optioned type and implements the corresponding [`Optionable`](https://docs.rs/optionable/latest/optionable/trait.Optionable.html) trait (see below for details).
It supports nested structures, enums as well as various container types.

The general logic is the same as for other rust derives. If you want to use the [`Optionable`](https://docs.rs/optionable/latest/optionable/derive.Optionable.html) derive macro for a struct/enum
every type used for a field needs to also have implemented the corresponding [`Optionable`](https://docs.rs/optionable/latest/optionable/trait.Optionable.html) trait:

```rust
#[derive(Optionable)]
#[optionable(derive(Default, Serialize, Deserialize))]
struct Address {
    street_name: String,
    number: u8,
}

fn example() {
    let _ = AddressOpt {
        street_name: Some("a".to_owned()),
        // fill the other fields with `None`
        ..Default::default()
    };
}
```

The generated optioned type is (shown here with resolved associated types) as follows:

```rust
#[derive(Default, Serialize, Deserialize)]
struct AddressOpt {
    street_name: Option<String>,
    number: Option<u8>,
}
```

### Enum support

Deriving optioned versions also works with enums:

```rust
#[derive(Optionable)]
enum AddressEnum {
    Plain(String),
    AddressExplicit { street: String, number: u32 },
    AddressNested(Address)
}

fn example() {
    let _ = AddressEnumOpt::AddressExplicit {
        street: Some("a".to_owned()),
        number: None
    };
}
```

## Core concept

The main [`Optionable`](https://docs.rs/optionable/latest/optionable/trait.Optionable.html) trait is quite simple:

```rust
pub trait Optionable {
    type Optioned;
}
```

It is a marker trait that allows to express for a given type `T` which type should be considered its `T::Optioned` type
such that `Option<T::Optioned>` would represent all variants of partial completeness.
For types without inner structure this means that the `Optioned` type will just resolve to the type itself, e.g.

```rust
impl Optionable for String {
    type Optioned = String;
}
```

For many primitive types as well as common wrapper or collection types the [
`Optionable`](https://docs.rs/optionable/latest/optionable/trait.Optionable.html) trait is already implemented.

### Conversion

Per default also conversion traits for struct/enums with sized fields will be derived.
The relevant traits are (shown here without comments and with some `where` clauses omitted):

```rust
pub trait OptionableConvert: Sized + Optionable {
    fn into_optioned(self) -> Self::Optioned;
    fn try_from_optioned(value: Self::Optioned) -> Result<Self, Error>;
    fn merge(&mut self, other: Self::Optioned) -> Result<(), Error>;
}

pub trait OptionedConvert<T>: Sized
{
    fn from_optionable(value: T) -> Self;
    fn try_into_optionable(self) -> Result<T, Error>;
}
```

## Crate features

- `derive`: Default-feature, re-exports the [`Optionable`](https://docs.rs/optionable/latest/optionable/derive.Optionable.html) derive macro.
- `std`: Default-feature. Adds `Optionable`-implementations for many [std](https://doc.rust-lang.org/std/)-lib types.
- `alloc`: Adds `Optionable`-implementations for [alloc](https://doc.rust-lang.org/alloc/) types (only useful when not enabling the `std` feature).
- `serde_json`: Derive [`trait@Optionable`] for [serde_json](https://docs.rs/serde_json/latest/serde_json/)`::Value`.
- `chrono04`: Derive [`trait@Optionable`] for types from [chrono](https://docs.rs/chrono/latest/chrono/) v0.4.
- `jiff02`: Derive `Optionable` for types from [jiff](https://docs.rs/jiff/latest/chrono/) v0.2.
- `k8s_openapi027_v1_(31..=35)`: Adds `Optionable`-implementations for all [k8s-openapi](https://docs.rs/k8s-openapi/latest/k8s_openapi) v0.27 types. 
   Only one feature version, e.g. `k8s_openapi027_v1_35` may be enabled at once.
- `k8s_openapi_convert`: Adds `OptionableConvert`-implementations for all optioned [k8s-openapi](https://docs.rs/k8s-openapi/latest/k8s_openapi) types specified by the `k8s_openapi027_v1_(31..=35)` feature.
- `kube3`: Tooling to derive optioned types for [kube](https://github.com/kube-rs/kube) v3 `CustomResource`. Also includes [`extract`](kube::ExtractManagedFields)-functionality for server-side apply.

## Limitations

### External types

Due to the orphan rule the usage of the library becomes cumbersome if one has a use case which heavily relies on
crate-external types.

If just have a few types from external crates don't have an `Optionable` impl
the [example/orphanrule](example/orphanrule) illustrates how to circumvent this limitation.

For well-established libraries adding corresponding `impﬀkﬀl` to this crate (feature-gated) would be a worthwhile approach.

### Resolving associated types

Due to the use of associated types some IDE-hints do not fully resolve the associated types leaving you with
`<i32 as Optionable>::Optioned` instead of `i32`. Luckily, for checking type correctness and also for error messages
when using wrong types the associated types are resolved.

For the derived `Optioned`-structs/enums a related issue is that other derive macros for those derived types won't see
the resolved associated types. Therefore, corresponding type bounds have to be added(done by the [
`Optionable`](https://docs.rs/optionable/latest/optionable/derive.Optionable.html) derive macro) to the `Optioned`-structs/enums:

```rust
#[derive(Optionable)]
#[optionable(derive(Serialize))]
struct DeriveExample<T> {
    name: T,
}

// The generated code for the struct is shown below (simplified)
#[derive(Serialize)]
struct DeriveExampleOpt<T>
where
    T: Optionable,
// extra `Serialize` bound on the struct level
    <T as Optionable>::Optioned: Sized + Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<<T as Optionable>::Optioned>
}
```

## Similar crates

One crate with similar scope is [optional_struct](https://crates.io/crates/optional_struct).
It focuses specifically on structs (not enums) and offers a more manual approach, especially in respect to nested
sub-struct, providing many fine-grained configuration options.

Another crate is [struct-patch](https://crates.io/crates/struct-patch).
It focuses on patching structs (not enums), especially from serde inputs. Nesting is supported with manual helper
annotations.

## License

You can use this under the conditions of the [MIT license](LICENSE-MIT) or
the [Apache License, Version 2.0](LICENSE-APACHE) at your option.

### Contributing

Any contributor has to agree to have their contribution also dual-licensed under the MIT as well as Apache-2.0 license
as specified above in the `License` subsection.
