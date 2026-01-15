use autodefault::autodefault;
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::{Api, Client};
use optionable::k8s_openapi::api::apps::v1::{DeploymentAc, DeploymentSpecAc};
use optionable_k8s_example::FIELD_MANAGER;

#[tokio::main]
#[autodefault]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::try_default().await?;
    let deployment_api: Api<Deployment> = Api::default_namespaced(client);
    let deployment_patch = DeploymentAc {
        metadata: ObjectMeta {
            name: Some("test".to_owned()),
        },
        spec: Some(DeploymentSpecAc { replicas: Some(2) }),
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
