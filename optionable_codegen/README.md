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

## Codegen
The binary `codegen` generates optioned versions for all structs/enums in the given rust file and included internal modules
recursively.
```bash
cargo run --features codegen --bin codegen example/input/src/lib.rs example/output
```

## Limitations
The codegen logic is at the moment very simple and can only handle very minimalistic input rust types
that basically only contain `mod`, `struct` and `enum`. Additional declarations are not an issue if it is ok
to simply ignore them.