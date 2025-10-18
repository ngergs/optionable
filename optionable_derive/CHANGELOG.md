# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
