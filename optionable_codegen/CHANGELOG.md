# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
