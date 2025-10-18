# optionable

A rust library to derive `optioned` structs/enums versions of existing types where all fields have been recursively
replaced
with versions that support setting just a subset of the relevant fields (or none at all).

One motivation for this concept is the common problem when expressing patches e.g.
for [Kubernetes apply configurations](https://pkg.go.dev/k8s.io/client-go/applyconfigurations)
that for a given rust struct `T` a corresponding struct `T::Optioned` would be required where all fields are recursively
optional
to specify.

While trivial to write for plain structures this quickly becomes tedious for nested structs/enums.

#### Links

- [crates.io](https://crates.io/crates/optionable)
- [rust documentation](https://docs.rs/optionable/)

## Deriving optional structs/enums

The core utility of this library is to provide an `Optionable`-derive macro that derives such an optioned type
and implements the corresponding `Optionable`-trait (see below for details).
It supports nested structures, enums as well as various container types.

For detailed configuration options via helper attributes, see the [
`Optionable`-derive macro docs](https://docs.rs/optionable/latest/optionable/derive.Optionable.html).

The general logic is the same as for other rust derives. If you want to use the derive `Optionable` for a struct/enum
every type used for a field needs to also have implemented the corresponding `Optionable` trait:

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

The main `Optionable` trait is quite simple:

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

For many primitive types as well as common wrapper or collection types the `Optionable`-trait is already implemented.

### Conversion

Per default also conversion traits for struct/enums with sized fields will be derived.
The relevant traits are (shown here without comments and with some `where` clauses omitted):

```rust
pub trait OptionableConvert: Sized + Optionable {
    fn into_optioned(self) -> Self::Optioned;
    fn try_from_optioned(value: Self::Optioned) -> Result<Self, Error>;
    fn merge(&mut self, other: Self::Optioned) -> Result<(), Error>;
}

// sealed, auto-implemented from `OptionableConvert` for every respective `T::Optioned`
pub trait OptionedConvert<T>
where
    T: Optionable<Optioned=Self> + OptionableConvert,
{
    fn from_optionable(value: T) -> Self;
    fn try_into_optionable(self) -> Result<T, Error>;
}
```

## Crate features

- `derive`: Default-feature, re-exports the `Optionable` derive macro.
- `std`: Default-feature. Adds `Optionable`-implementations for many stdlib types.
- `alloc`: Adds `Optionable`-implementations for `alloc` types (only useful when not enabling the `std` feature).
- `chrono`: Derive `Optionable` for types from [chrono](https://docs.rs/chrono/latest/chrono/)
- `serde_json`: Derive `Optionable` for [serde_json](https://docs.rs/serde_json/latest/serde_json/)::Value

## Limitations

### External types

Due to the orphan rule the usage of the library becomes cumbersome if one has a use case which heavily relies on
crate-external types.
For well-established libraries adding corresponding `impl` to this crate (feature-gated) would be a worthwhile approach.

### IDE: Resolving associated types

Due to the use of associated types some IDE-hints do not fully resolve the associated types leaving you with
`<i32 as Optionable>::Optioned` instead of `i32`. Luckily, for checking type correctness and also for error messages
when using wrong types the associated types are resolved.

## Similar crates

One crate with similar scope is [optional_struct](https://crates.io/crates/optional_struct).
It focuses specifically on structs (not enums) and offers a more manual approach, especially in respect to nested
sub-struct,
providing many fine-grained configuration options.

Another crate is [struct-patch](https://crates.io/crates/struct-patch).
It focuses on patching structs (not enums), especially from serde inputs. Nesting is supported with manual helper
annotations.

### License

You can use this under the conditions of the [MIT license](LICENSE-MIT) or
the [Apache License, Version 2.0](LICENSE-APACHE) at your option.

#### Contributing

Any contributor has to agree to have their contribution also dual-licensed under the MIT as well as Apache-2.0 license
as
specified above in the `License` subsection.
