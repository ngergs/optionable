#!/bin/bash
set -euo pipefail

K8S_OPENAPI_VERSION=$(git -C "$1" describe --exact-match --tags)
echo "The \`v1_*\` folders are generated based on [k8s-openapi](https://github.com/Arnavion/k8s-openapi) ${K8S_OPENAPI_VERSION}" > version.md

for v in {31..35}
do
	cargo run --manifest-path=../../../optionable_codegen/bin/Cargo.toml --features k8s_openapi --bin k8s_openapi -- "$1/src/v1_${v}/mod.rs" "v1_${v}" "--package-name" "k8s_openapi027"
done
