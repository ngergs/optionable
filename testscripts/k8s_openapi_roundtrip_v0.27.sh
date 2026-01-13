#!/usr/bin/env bash

set -euxo pipefail
if [[ $(basename "$PWD") != "testscripts" ]]; then
    echo "Scripts have to be executes with \"testscripts\" being the current working directory."
    exit 1
fi
cd ..

cp Cargo.toml Cargo.toml.backup
cleanup() {
  mv Cargo.toml.backup Cargo.toml
}
trap cleanup EXIT

echo 'k8s-openapi = { git = "https://github.com/ngergs/k8s-openapi", tag="v0.27.0" }' >> Cargo.toml
export RUSTFLAGS="--cfg test_k8s_openapi_roundtrip"

cargo test --manifest-path=optionable/Cargo.toml --features k8s-openapi027/std,k8s_openapi_convert,k8s_openapi027_v1_31
cargo test --manifest-path=optionable/Cargo.toml --features k8s-openapi027/std,k8s_openapi_convert,k8s_openapi027_v1_32
cargo test --manifest-path=optionable/Cargo.toml --features k8s-openapi027/std,k8s_openapi_convert,k8s_openapi027_v1_33
cargo test --manifest-path=optionable/Cargo.toml --features k8s-openapi027/std,k8s_openapi_convert,k8s_openapi027_v1_34
cargo test --manifest-path=optionable/Cargo.toml --features k8s-openapi027/std,k8s_openapi_convert,k8s_openapi027_v1_35
