use k8s_openapi::api::apps::v1::Deployment;
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::{Api, Client};
use optionable::k8s_openapi::v1_34::api::apps::v1::{DeploymentAc, DeploymentSpecAc};
use optionable_k8s_openapi_example::FIELD_MANAGER;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::try_default().await?;
    let deployment_api: Api<Deployment> = Api::default_namespaced(client);
    let deployment_patch = DeploymentAc {
        metadata: ObjectMeta {
            name: Some("test".to_owned()),
            ..Default::default()
        },
        spec: Some(DeploymentSpecAc {
            replicas: Some(2),
            ..Default::default()
        }),
        ..Default::default()
    };
    deployment_api
        .patch(
            deployment_patch
                .metadata
                .name
                .as_ref()
                .ok_or("name missing")?,
            &PatchParams::apply(FIELD_MANAGER).force(),
            &Patch::Apply(&deployment_patch),
        )
        .await?;
    Ok(())
}
