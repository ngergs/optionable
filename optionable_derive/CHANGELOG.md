# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1](https://github.com/ngergs/optionable/compare/optionable_derive-v0.5.0...optionable_derive-v0.5.1) - 2025-12-01

### Other

- set Cargo edition, rust version, license and repo in workspace settings

## [0.4.5](https://github.com/ngergs/optionable/compare/optionable_derive-v0.4.4...optionable_derive-v0.4.5) - 2025-11-30

### Fixed

- document (and verify in release gha) minimal rust version

## [0.4.4](https://github.com/ngergs/optionable/compare/optionable_derive-v0.4.3...optionable_derive-v0.4.4) - 2025-11-12

### Other

- docs

## [0.4.3](https://github.com/ngergs/optionable/compare/optionable_derive-v0.4.2...optionable_derive-v0.4.3) - 2025-11-11

### Other

- simplify derive

## [0.4.2](https://github.com/ngergs/optionable/compare/optionable_derive-v0.4.1...optionable_derive-v0.4.2) - 2025-11-10

### Fixed

- forward `serde(rename_all)` for kube helper instead of deaulting to `camelCase`

## [0.4.1](https://github.com/ngergs/optionable/compare/optionable_derive-v0.4.0...optionable_derive-v0.4.1) - 2025-11-10

### Other

- docs
- docs
- docs

## [0.4.0](https://github.com/ngergs/optionable/compare/optionable_derive-v0.3.0...optionable_derive-v0.4.0) - 2025-11-09

### Added

- granular option to copy over specified helper attributes to the optioned type

### Fixed

- [**breaking**] breaking change in codegen (no longer adds serde untagged for kube/k8s_openapi enums)

## [0.3.0](https://github.com/ngergs/optionable/compare/optionable_derive-v0.2.0...optionable_derive-v0.2.1) - 2025-11-08

### Added

- Kubernetes server side apply support (see next two items).
- Generated optioned types for all types from [`k8s-openapi`[(https://crates.io/crates/k8s-openapi)
- Support for deriving optioned types for [`kube`](https://docs.rs/kube/latest/kube/) CustomResources.

### Fixed

- [**breaking**] moved the error type from `optionable::optionable::Error` to `optionable::Error`

## [0.2.1](https://github.com/ngergs/optionable/compare/optionable_derive-v0.2.0...optionable_derive-v0.2.1) - 2025-10-27

### Other

- prepare release

## [0.2.0](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.9...optionable_derive-v0.2.0) - 2025-10-18

### Added

- [**breaking**] update codegen dependency

## [0.1.9](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.8...optionable_derive-v0.1.9) - 2025-10-15

### Fixed

- readme, docs

## [0.1.8](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.7...optionable_derive-v0.1.8) - 2025-10-10

### Added

- codegen settings/flags for targetting optionable

### Other

- docs

## [0.1.7](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.6...optionable_derive-v0.1.7) - 2025-10-01

### Added

- support forwarding of helper attributes to derived optioned types

## [0.1.6](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.5...optionable_derive-v0.1.6) - 2025-09-30

### Added

- default-enabled feature for derive re-export
- rename codegen crate, use DeriveInput as input arg
- codegen to simplify implementation for external packages

### Fixed

- update dependencies

### Other

- readme

## [0.1.5](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.4...optionable_derive-v0.1.5) - 2025-09-24

### Fixed

- dual-license under apache version 2.0 in addition to MIT

## [0.1.4](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.3...optionable_derive-v0.1.4) - 2025-09-24

### Added

- resolve self-resolving types like i32 immediately when deriving optioned structs/enums

### Fixed

- simplify derive for enums

## [0.1.3](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.2...optionable_derive-v0.1.3) - 2025-09-17

### Added

- implement OptionableConvert for chrono::DateTime
- implement OptionableConvert

### Fixed

- reduce derive macro dependencies
- simplify derive logic
- adjust Optionable-impl for Option

### Other

- docs

## [0.1.2](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.1...optionable_derive-v0.1.2) - 2025-09-10

### Added

- 'required' helper attribute

### Fixed

- docs

## [0.1.1](https://github.com/ngergs/optionable/compare/optionable_derive-v0.1.0...optionable_derive-v0.1.1) - 2025-09-09

### Fixed

- derive readme url typo

## [0.1.0](https://github.com/ngergs/optionable/releases/tag/optionable_derive-v0.1.0) - 2025-09-09

### Added

- add serde helper attributes to derived structs to skip serializing Option::None

### Fixed

- clippy
- use darling default attribute for derive macro implementation
- keep visibility same for derived optional structs/enums
- handle visibility modifier in derive

### Other

- readme
- prepare publish
- document similar crates
- docs
- rename to optionable
