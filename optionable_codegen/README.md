# optionable_codegen

The relevant main crate is [optionable](https://crates.io/crates/optionable). The docs can be found there.

## Purpose
This code generation `proc_macro2` library serves two purposes:
- Used by [optionable_derive](https://crates.io/crates/optionable_derive) to implement the `#[derive(Optionable)]`-macro 
re-exported by [optionable](https://crates.io/crates/optionable_derive).
- Used by the [bin/codegen.rs](bin/codegen.rs) crate to support generating `Optionable`-implementations for external packages.
Due to the orphan rule  the generated code has to be added to the `Optionable`-package (PRs welcome).

It has to be a separate crate from [optionable_derive](https://crates.io/crates/optionable_derive) as the proc-macro crates
can't export its non-macro functions (even the proc_macro2 ones) for the usage by the codegen part.

## codegen
The binary `codegen` target traverses all `.rs`-files in a given folder and generates optioned versions
of the found struct and enums.
```bash
cargo run --features codegen --bin codegen example/input example/output
```