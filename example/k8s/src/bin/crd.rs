use kube::CustomResourceExt;
use optionable_k8s_example::CustomCrd;

// CRD definition is in ../lib.rs

fn main() -> Result<(), serde_saphyr::ser_error::Error> {
    println!("{}", serde_saphyr::to_string(&CustomCrd::crd())?);
    Ok(())
}
