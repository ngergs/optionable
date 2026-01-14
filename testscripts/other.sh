#!/usr/bin/env bash

# tests docgen, licenses, msrv
set -euxo pipefail
if [[ $(basename "$PWD") != "testscripts" ]]; then
    echo "Scripts have to be executes with \"testscripts\" being the current working directory."
    exit 1
fi
cd ..

cargo audit
cargo clippy --workspace --exclude optionable_k8s_example --features std,derive,chrono04,jiff02,serde_json,k8s_openapi027_v1_35,kube3,kube3/derive
cargo +nightly docs-rs -p optionable
cargo msrv verify --path optionable_codegen
cargo msrv verify --path optionable_derive
cargo msrv verify --path optionable
cargo deny check license
