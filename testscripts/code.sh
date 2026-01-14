#!/usr/bin/env bash

set -euxo pipefail
if [[ $(basename "$PWD") != "testscripts" ]]; then
    echo "Scripts have to be executes with \"testscripts\" being the current working directory."
    exit 1
fi
cd ..

cargo test --manifest-path=optionable/Cargo.toml --features derive,kube3,kube3/derive,k8s-openapi027/v1_34
cargo test --manifest-path=optionable/Cargo.toml --no-default-features --features derive,chrono04,jiff02,serde_json
cargo test --manifest-path=optionable/Cargo.toml --no-default-features --features std,derive,chrono04,jiff02,serde_json
cargo test --manifest-path=optionable/Cargo.toml --no-default-features --features alloc,derive,chrono04,jiff02,serde_json
cargo test --workspace --exclude optionable_k8s_example --features k8s_openapi027_v1_31,kube3,k8s_openapi_convert,kube3/derive
cargo test --workspace --exclude optionable_k8s_example --features k8s_openapi027_v1_32,kube3,k8s_openapi_convert,kube3/derive
cargo test --workspace --exclude optionable_k8s_example --features k8s_openapi027_v1_33,kube3,k8s_openapi_convert,kube3/derive
cargo test --workspace --exclude optionable_k8s_example --features k8s_openapi027_v1_34,kube3,k8s_openapi_convert,kube3/derive
cargo test --workspace --exclude optionable_k8s_example --features k8s_openapi027_v1_35,kube3,k8s_openapi_convert,kube3/derive
