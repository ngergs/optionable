#!/bin/bash
set -euo pipefail

K8S_OPENAPI_VERSION=$(git -C "$1" describe --exact-match --tags)
echo "The \`v1_*\` folders are generated based on [k8s-openapi](https://github.com/Arnavion/k8s-openapi) ${K8S_OPENAPI_VERSION}" > version.md

for v in {32..36}
do
  (cd "$2" && git checkout $(git tag -l "v1.${v}.*" --sort=-v:refname | head -n1))
	cargo run --manifest-path=../../../optionable_codegen/bin/Cargo.toml --features k8s_openapi --bin k8s_openapi -- "--input-file" "$1/src/v1_${v}/mod.rs" "--output-dir" "v1_${v}" "--package-name" "k8s_openapi028" "--k8s-openapi-v3-dir" "${2}/api/openapi-spec/v3"
done
