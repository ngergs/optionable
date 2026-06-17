use std::collections::BTreeSet;

fn main() {
    let mut enabled_versions = enabled_k8s_openapi_versions("027", 31..=35);
    enabled_versions.extend(enabled_k8s_openapi_versions("028", 32..=36));
    assert!(
        enabled_versions.len() <= 1,
        "\nOnly one `k8s_openapi*` feature may be enabled at a given time, found: {enabled_versions:?}`\n"
    );
}

/// Returns the enabled k8s-openapi_versions.
/// The first argument should be the `k8s-openapi` dependency suffix as used in thei `Cargo.toml` of this project
/// and the second the range of relevant k8s openapi versions supported by upstream `k8s-openapi` for this released version.
///
/// ```
///   enabled_k8s_openapi_versions("028", 32..=36)
/// ```
fn enabled_k8s_openapi_versions(
    version: &str,
    range: impl IntoIterator<Item = usize>,
) -> BTreeSet<String> {
    range
        .into_iter()
        .map(|v| format!("V1_{v}"))
        .filter(|v| std::env::var(format!("CARGO_FEATURE_K8S_OPENAPI{version}_{v}")).is_ok())
        .collect()
}
