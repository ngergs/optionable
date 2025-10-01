# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
