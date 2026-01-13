fn main() {
    let enabled_versions_026 = (30..=34)
        .map(|v| format!("V1_{v}"))
        .filter(|v| std::env::var(format!("CARGO_FEATURE_K8S_OPENAPI026_{v}")).is_ok())
        .collect::<std::collections::BTreeSet<_>>();
    let enabled_versions_027 = (31..=35)
        .map(|v| format!("V1_{v}"))
        .filter(|v| std::env::var(format!("CARGO_FEATURE_K8S_OPENAPI027_{v}")).is_ok())
        .collect::<std::collections::BTreeSet<_>>();
    let enabled_versions = enabled_versions_026
        .union(&enabled_versions_027)
        .collect::<Vec<_>>();
    assert!(
        enabled_versions.len() <= 1,
        "\nOnly one `k8s_openapi*` feature may be enabled at a given time, found: {enabled_versions:?}`\n"
    );
}
