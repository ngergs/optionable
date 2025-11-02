fn main() {
    let enabled_versions = (30..=34)
        .map(|v| format!("V1_{v}"))
        .filter(|v| std::env::var(format!("CARGO_FEATURE_K8S_OPENAPI_{v}")).is_ok())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        enabled_versions.len() <= 1,
        "\nOnly one `k8s_openapi_*` feature may be enabled at a given time, found: {enabled_versions:?}`\n"
    );
}
