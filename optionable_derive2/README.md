# optionable_derive

This package is as a library not intended to be used by others explicitly.
It it consumed by [optionable_derive](https://crates.io/crates/optionable_derive) to implement
derive macros from a common code basis as the codegen provided here for the [optionable](https://crates.io/crates/optionable) package. 
The relevant docs can be also found there.

This is only a separate crate as the proc-macro crate [optionable_derive](https://crates.io/crates/optionable_derive)
can't export its functions (even the proc_macro2 ones) for the usage by the codegen part.

## codegen
Has a binary `codegen` that traverses all `.rs`-files in a given folder and generates optioned versions
of the found struct and enums.
```bash
cargo run --features codegen --bin codegen example/input example/output
```