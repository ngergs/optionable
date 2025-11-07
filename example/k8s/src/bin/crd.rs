use kube::CustomResourceExt;
use optionable_k8s_openapi_example::MyCrd;

fn main() -> Result<(), serde_yaml_ng::Error> {
    println!("{}", serde_yaml_ng::to_string(&MyCrd::crd())?);
    Ok(())
}
