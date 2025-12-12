#!/usr/bin/env bash

# tests docgen, licenses, msrv
set -euxo pipefail
if [[ $(basename "$PWD") != "testscripts" ]]; then
    echo "Scripts have to be executes with \"testscripts\" being the current working directory."
    exit 1
fi
cd ..

cargo clippy --features std,derive,chrono,serde_json,k8s_openapi_v1_34,kube,kube/derive
cargo +nightly docs-rs -p optionable
cargo msrv verify --path optionable_codegen
cargo msrv verify --path optionable_derive
cargo msrv verify --path optionable
cargo deny check license