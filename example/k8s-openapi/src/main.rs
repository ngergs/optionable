use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::{Api, Client};
use optionable::k8s_openapi::k8s_openapi::api::apps::v1::Deployment;
use optionable::k8s_openapi::v1_34::api::apps::v1::{DeploymentAc, DeploymentSpecAc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::try_default().await?;
    let deployments: Api<Deployment> = Api::default_namespaced(client);
    let patch = DeploymentAc {
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
    let d = deployments
        .patch(
            &patch.metadata.name.clone().ok_or("name missing")?,
            &PatchParams::apply("rust-manager").force(),
            &Patch::Apply(patch),
        )
        .await?;
    Ok(())
}
