# optionable_codegen_cli

Non-feature complete experimental `cli` for advanced logic utilizing the core codegen logic
also used in the `derive`-macro of the `optionable`-crate.

## Usage
The binary cli in the "bin" `codegen` generates optioned versions for all structs/enums in the given rust file
and included internal modules recursively.
```bash
cargo run example/input/src/lib.rs example/output
```