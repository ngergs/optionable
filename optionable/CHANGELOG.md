# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.1](https://github.com/ngergs/optionable/compare/optionable-v0.8.0...optionable-v0.8.1) - 2025-12-04

### Other

- docs

## [0.8.0](https://github.com/ngergs/optionable/compare/optionable-v0.7.0...optionable-v0.8.0) - 2025-12-03

### Fixed

- [**breaking**] no longer generate an optioned type for `WatchEvent` from `k8s-openapi`

### Other

- docs

## [0.7.0](https://github.com/ngergs/optionable/compare/optionable-v0.6.1...optionable-v0.7.0) - 2025-12-02

### Added

- [**breaking**] unseal `OptionedConvert`, drop blanket implementation, add `optioned_type` derive field attribute

### Fixed

- OptionableConvert for k8s-openapi watch event broke compilation
- regenerate k8s-openapi generated code

### Other

- docs

## [0.6.1](https://github.com/ngergs/optionable/compare/optionable-v0.6.0...optionable-v0.6.1) - 2025-12-01

### Other

- docs
- set Cargo edition, rust version, license and repo in workspace settings

## [0.6.0](https://github.com/ngergs/optionable/compare/optionable-v0.5.8...optionable-v0.6.0) - 2025-11-30

### Added

- [**breaking**] derived optionable implementation for plain enums to itself

### Fixed

- document (and verify in release gha) minimal rust version

## [0.5.8](https://github.com/ngergs/optionable/compare/optionable-v0.5.7...optionable-v0.5.8) - 2025-11-30

### Fixed

- regenerate k8s-openapi optionable implementation with relaxed where bounds

## [0.5.7](https://github.com/ngergs/optionable/compare/optionable-v0.5.6...optionable-v0.5.7) - 2025-11-30

### Added

- full support unsized generic types for convert

### Fixed

- nightly docgen config

### Other

- conditional integration tests adjustment
- restructure integration tests

## [0.5.6](https://github.com/ngergs/optionable/compare/optionable-v0.5.5...optionable-v0.5.6) - 2025-11-22

### Fixed

- simplify generated/derived code by using `PhantomData<Self>` for the api envelope

### Other

- document k8s_openapi version used by codegen

## [0.5.5](https://github.com/ngergs/optionable/compare/optionable-v0.5.4...optionable-v0.5.5) - 2025-11-12

### Fixed

- for kube/k8s-openapi use #[serde(rename_all_fields="...")] instead of rename_all for enums

## [0.5.4](https://github.com/ngergs/optionable/compare/optionable-v0.5.3...optionable-v0.5.4) - 2025-11-12

### Fixed

- simplify extract function signature
- avoid deref in deserialize_api_envelope

### Other

- docs

## [0.5.3](https://github.com/ngergs/optionable/compare/optionable-v0.5.2...optionable-v0.5.3) - 2025-11-11

### Added

- optionable::kube::deserialize_envelope function to verify api envelope

## [0.5.2](https://github.com/ngergs/optionable/compare/optionable-v0.5.1...optionable-v0.5.2) - 2025-11-10

### Fixed

- merge derives from the derive, kube and k8s_openapi helper attributes

## [0.5.1](https://github.com/ngergs/optionable/compare/optionable-v0.5.0...optionable-v0.5.1) - 2025-11-10

### Fixed

- blanket `Optionable` impl for &mut T did resolve to &T instead of &mut T

## [0.5.0](https://github.com/ngergs/optionable/compare/optionable-v0.4.0...optionable-v0.5.0) - 2025-11-09

### Fixed

- [**breaking**] optionable kube/k8s_openapi no longer automatically add serde untagged
- avoid cloning in extract function

### Other

- docs

## [0.4.0](https://github.com/ngergs/optionable/compare/optionable-v0.3.0...optionable-v0.4.0) - 2025-11-09

### Added

- [**breaking**] feature-gate the `OptionableConvert`-impls for k8s-openapi
- kube extract functionality

### Fixed

- use trait for extract functionality

### Other

- docs
- readme docs.rs links

## [0.3.0](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.4.3...optionable_codegen-v0.5.0) - 2025-11-08

### Added

- Kubernetes server side apply support (see next two items).
- Generated optioned types for all types from [`k8s-openapi`](https://crates.io/crates/k8s-openapi)
- Support for deriving optioned types for [`kube`](https://docs.rs/kube/latest/kube/) CustomResources.

### Fixed

- [**breaking**] moved the error type from `optionable::optionable::Error` to `optionable::Error`
- use DeserializeOwned as generic type bound for optioned Deserialize-derives
- simplify generated code (use less explicit associated types)


## [0.2.6](https://github.com/ngergs/optionable/compare/optionable-v0.2.5...optionable-v0.2.6) - 2025-10-25

### Other

- document associated types limitations

## [0.2.5](https://github.com/ngergs/optionable/compare/optionable-v0.2.4...optionable-v0.2.5) - 2025-10-23

### Added

- codegen/derive: add type bounds for requested `derive` of the optioned type

### Fixed

- simplify where clause codegen implementation

## [0.2.4](https://github.com/ngergs/optionable/compare/optionable-v0.2.3...optionable-v0.2.4) - 2025-10-21

### Added

- relax type restrictions for derived types with generic parameter

## [0.2.3](https://github.com/ngergs/optionable/compare/optionable-v0.2.2...optionable-v0.2.3) - 2025-10-20

### Fixed

- make `Optionable` tuple implementation consistent with pre-existing derived logic

## [0.2.2](https://github.com/ngergs/optionable/compare/optionable-v0.2.1...optionable-v0.2.2) - 2025-10-20

### Added

- implement `Optionable` for tuples with up to 16 elements
- blanket `Optionable` implementation for mutable references
- implement `Optionable` for slices `[T]`

## [0.2.1](https://github.com/ngergs/optionable/compare/optionable-v0.2.0...optionable-v0.2.1) - 2025-10-18

### Fixed

- add missing documentation for feature-dependent availability of some `OptionableConvert` implementations

## [0.2.0](https://github.com/ngergs/optionable/compare/optionable-v0.1.13...optionable-v0.2.0) - 2025-10-18

### Added

- [**breaking**] std and alloc features to support opting-out of linking to the stdlib
- implement `Optionable` for `PathBuf` and `Path`
- implement `Optionable` for `Duration`
- implement `Optionable` for `OsString` and `OsStr`
- implement `Optionable` for weak `Rc`/`Arc`

### Other

- clippy, make partially unused code feature-dependent
- document std/alloc features
- docs
- fmt

## [0.1.13](https://github.com/ngergs/optionable/compare/optionable-v0.1.12...optionable-v0.1.13) - 2025-10-16

### Added

- implement Error trait for our Error struct
- implement `Optionable` for `[T;N]`

### Other

- readme

## [0.1.12](https://github.com/ngergs/optionable/compare/optionable-v0.1.11...optionable-v0.1.12) - 2025-10-16

### Fixed

- docrs build

## [0.1.11](https://github.com/ngergs/optionable/compare/optionable-v0.1.10...optionable-v0.1.11) - 2025-10-15

### Added

- implement `Optionable` for ()

### Fixed

- document feature-gated derive macro and impls
- readme, docs

### Other

- readme

## [0.1.10](https://github.com/ngergs/optionable/compare/optionable-v0.1.9...optionable-v0.1.10) - 2025-10-10

### Added

- generalize `Optionable` trait to unsized
- implement the `Optionable` trait for `Cow`
- codegen option to prefix type path for generated `Optionable` impl

### Fixed

- no automatically_derived on struct/enum definitions, refactor derive code

## [0.1.9](https://github.com/ngergs/optionable/compare/optionable-v0.1.8...optionable-v0.1.9) - 2025-10-01

### Fixed

- handle multiple `optionable_attr` attributes

### Other

- docs

## [0.1.8](https://github.com/ngergs/optionable/compare/optionable-v0.1.7...optionable-v0.1.8) - 2025-10-01

### Fixed

- support attribute forwarding also for enum variants

## [0.1.7](https://github.com/ngergs/optionable/compare/optionable-v0.1.6...optionable-v0.1.7) - 2025-10-01

### Added

- support forwarding of helper attributes to derived optioned types

### Other

- docs

## [0.1.6](https://github.com/ngergs/optionable/compare/optionable-v0.1.5...optionable-v0.1.6) - 2025-09-30

### Added

- default-enabled feature for derive re-export

### Other

- cleanup gitignore
- integration tests
- docs
- docs
- docs
- docs
- readme

## [0.1.5](https://github.com/ngergs/optionable/compare/optionable-v0.1.4...optionable-v0.1.5) - 2025-09-24

### Fixed

- dual-license under apache version 2.0 in addition to MIT

## [0.1.4](https://github.com/ngergs/optionable/compare/optionable-v0.1.3...optionable-v0.1.4) - 2025-09-24

### Added

- resolve self-resolving types like i32 immediately when deriving optioned structs/enums

## [0.1.3](https://github.com/ngergs/optionable/compare/optionable-v0.1.2...optionable-v0.1.3) - 2025-09-17

### Added

- implement OptionableConvert for chrono::DateTime
- auto-implement trait to convert back from optioned to optionable
- implement OptionableConvert
- add impl for Result

### Fixed

- dependency updates
- adjust Optionable-impl for Option

### Other

- docs
- docs
- docs
- document OptionaleConvert
- tests
- minimal test for conversion
- docs typo
- docs

## [0.1.2](https://github.com/ngergs/optionable/compare/optionable-v0.1.1...optionable-v0.1.2) - 2025-09-10

### Fixed

- docs

## [0.1.1](https://github.com/ngergs/optionable/compare/optionable-v0.1.0...optionable-v0.1.1) - 2025-09-10

### Added

- 'required' helper attribute

### Fixed

- docs
- docs

## [0.1.0](https://github.com/ngergs/optionable/releases/tag/optionable-v0.1.0) - 2025-09-09

### Added

- simplify optionable derive macro import
- add serde helper attributes to derived structs to skip serializing Option::None
- feature-gated impl for chrono and serde_json

### Fixed

- clippy
- dependency updates
- keep visibility same for derived optional structs/enums

### Other

- prepare release
- initial release of optionable_derive
- prepare release
- readme
- readme
- prepare publish
- document similar crates
- docs
- readme
- readme
- docs
- test for structwrap trick, simplify impl
- docs
- rename to optionable
