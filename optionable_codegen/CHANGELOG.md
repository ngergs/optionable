# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.1](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.8.0...optionable_codegen-v0.8.1) - 2025-12-01

### Other

- set Cargo edition, rust version, license and repo in workspace settings

## [0.8.0](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.7.7...optionable_codegen-v0.8.0) - 2025-11-30

### Added

- [**breaking**] derived optionable implementation for plain enums to itself

### Fixed

- document (and verify in release gha) minimal rust version

## [0.7.7](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.7.6...optionable_codegen-v0.7.7) - 2025-11-30

### Fixed

- reworked where clause broken for adding multiple bounds to existing constraint

## [0.7.6](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.7.5...optionable_codegen-v0.7.6) - 2025-11-30

### Added

- full support unsized generic types for convert

## [0.7.5](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.7.4...optionable_codegen-v0.7.5) - 2025-11-22

### Fixed

- avoid some unecessary clones during derive/codegen
- simplify generated/derived code by using `PhantomData<Self>` for the api envelope

## [0.7.4](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.7.3...optionable_codegen-v0.7.4) - 2025-11-12

### Fixed

- for kube/k8s-openapi use #[serde(rename_all_fields="...")] instead of rename_all for enums

## [0.7.3](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.7.2...optionable_codegen-v0.7.3) - 2025-11-11

### Added

- optionable::kube::deserialize_envelope function to verify api envelope

## [0.7.2](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.7.1...optionable_codegen-v0.7.2) - 2025-11-10

### Fixed

- merge derives from the derive, kube and k8s_openapi helper attributes
- forward serde(rename_all) for kube helper instead of defaulting to camelCase

## [0.7.1](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.7.0...optionable_codegen-v0.7.1) - 2025-11-10

### Added

- filter out `Optionable` out of forward derive attributes

### Other

- docs, update example

## [0.7.0](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.6.0...optionable_codegen-v0.7.0) - 2025-11-09

### Added

- granular option to copy over specified helper attributes to the optioned type

### Fixed

- [**breaking**] optionable kube/k8s_openapi no longer automatically add serde untagged

### Other

- more involved custom resource example utilizing enums and serde rename

## [0.6.0](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.4.3...optionable_codegen-v0.5.0) - 2025-11-08

### Added

- Kubernetes server side apply support (see next two items).
- Generated optioned types for all types from [`k8s-openapi`[(https://crates.io/crates/k8s-openapi)
- Support for deriving optioned types for [`kube`](https://docs.rs/kube/latest/kube/) CustomResources.

### Fixed

- [**breaking**] moved the error type from `optionable::optionable::Error` to `optionable::Error`
- use DeserializeOwned as generic type bound for optioned Deserialize-derives
- simplify generated code (use less explicit associated types)

## [0.5.0](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.4.3...optionable_codegen-v0.5.0) - 2025-10-27

### Fixed

- mv cli code to separate unpublished crate
- clippy

## [0.4.3](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.4.2...optionable_codegen-v0.4.3) - 2025-10-25

### Fixed

- `replacement-crate` codegen options also adjusts where clauses

## [0.4.2](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.4.1...optionable_codegen-v0.4.2) - 2025-10-23

### Added

- codegen/derive: add type bounds for requested `derive` of the optioned type

### Fixed

- simplify codegen
- remove some `clone`-calls from codegen
- error on misused `optionable_attr` helper attribute
- skip where_clause checks if no generic parameters are present
- simplify where clause codegen implementation

### Other

- restructure codegen code

## [0.4.1](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.4.0...optionable_codegen-v0.4.1) - 2025-10-21

### Added

- relax type restrictions for derived types with generic parameter

## [0.4.0](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.3.0...optionable_codegen-v0.4.0) - 2025-10-18

### Added

- also check for full core import path for `is_option` check
- [**breaking**] std and alloc features to support opting-out of linking to the stdlib
- update self-resolving types

### Fixed

- clippy

### Other

- fmt

## [0.3.0](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.2.0...optionable_codegen-v0.3.0) - 2025-10-15

### Added

- implement `Optionable` for ()

### Fixed

- document feature-gated derive macro and impls
- [**breaking**] refactor derive_optionable to avoid Cow in the fn signature

## [0.2.0](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.1.3...optionable_codegen-v0.2.0) - 2025-10-10

### Added

- generalize `Optionable` trait to unsized
- codegen option to prefix type path for generated `Optionable` impl
- codegen settings/flags for targetting optionable
- codegen option to adjust the crate name used for the `Optionable` traits
- rework codegen to start from all files and follow internal module included recursively

### Fixed

- pass codegen flags through for subfolders
- codegen internal fn naming
- simplify derive code
- no automatically_derived on struct/enum definitions, refactor derive code

### Other

- docs
- readme
- codegen internal docs

## [0.1.3](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.1.2...optionable_codegen-v0.1.3) - 2025-10-01

### Fixed

- handle multiple `optionable_attr` attributes

## [0.1.2](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.1.1...optionable_codegen-v0.1.2) - 2025-10-01

### Fixed

- support attribute forwarding also for enum variants

## [0.1.1](https://github.com/ngergs/optionable/compare/optionable_codegen-v0.1.0...optionable_codegen-v0.1.1) - 2025-10-01

### Added

- support forwarding of helper attributes to derived optioned types

### Other

- docs

## [0.1.0](https://github.com/ngergs/optionable/releases/tag/optionable_codegen-v0.1.0) - 2025-09-30

### Added

- default-enabled feature for derive re-export
- codegen option to add derives for generated types
- codegen add public function for attributes
- codegen option for --no-convert
- rename codegen crate, use DeriveInput as input arg

### Fixed

- simplify codegen feature config logic
- codegen for nested directories
- use clap for codegen arg parsing
- osString handling for codegen output

### Other

- docs
- codegen tests
- adjust codegen version
- readme
