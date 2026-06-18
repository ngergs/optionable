fn main() {
    assert!(
        !(std::env::var("CARGO_FEATURE_KUBE3").is_ok()
            && std::env::var("CARGO_FEATURE_KUBE4").is_ok()),
        "Only either kub3 or kube4 feature can be enabled at a given time"
    );
}
