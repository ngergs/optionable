#!/bin/bash
cargo run --manifest-path=../../../optionable_codegen/Cargo.toml --features codegen --bin codegen -- --replace-crate-name k8s_openapi ../../../../k8s-openapi/src/v1_34/mod.rs v1_34


