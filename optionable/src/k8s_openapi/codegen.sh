#!/bin/bash
cargo run --manifest-path=../../../optionable_codegen/bin/Cargo.toml --features k8s_openapi --bin k8s_openapi -- ../../../../k8s-openapi/src/v1_34/mod.rs v1_34


