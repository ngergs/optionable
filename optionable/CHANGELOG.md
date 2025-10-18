# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
