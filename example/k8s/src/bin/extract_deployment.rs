use k8s_openapi::api::apps::v1::Deployment;
use kube::{Api, Client};
use optionable::kube::extract;
use optionable_k8s_openapi_example::FIELD_MANAGER;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::try_default().await?;
    let deployment_api: Api<Deployment> = Api::default_namespaced(client);
    let deployment = deployment_api.get("test").await?;
    let deployment_owned_fields = extract(deployment, FIELD_MANAGER)?;
    println!("{deployment_owned_fields:#?}");
    Ok(())
}
