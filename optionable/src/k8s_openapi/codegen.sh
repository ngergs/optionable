#!/bin/bash
set -euo pipefail
for v in {30..34}
do
	cargo run --manifest-path=../../../optionable_codegen/bin/Cargo.toml --features k8s_openapi --bin k8s_openapi -- $1/src/v1_${v}/mod.rs v1_${v}
done
