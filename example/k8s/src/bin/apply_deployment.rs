use std::collections::BTreeMap;

use autodefault::autodefault;
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::{Api, Client};
use optionable::k8s_openapi::api::apps::v1::{DeploymentAc, DeploymentSpecAc};
use optionable::k8s_openapi::api::core::v1::{ContainerAc, PodSpecAc, PodTemplateSpecAc};
use optionable_k8s_example::FIELD_MANAGER;

#[tokio::main]
#[autodefault]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::try_default().await?;
    let deployment_api: Api<Deployment> = Api::default_namespaced(client);
    let deployment_patch = DeploymentAc {
        metadata: ObjectMeta {
            name: Some("test".to_owned()),
            labels: Some(BTreeMap::from([("hello2".to_owned(), "world2".to_owned())])),
        },
        spec: Some(DeploymentSpecAc {
            replicas: Some(2),
            template: Some(PodTemplateSpecAc {
                spec: Some(PodSpecAc {
                    containers: Some(vec![ContainerAc {
                        name: Some("app".to_owned()),
                        image_pull_policy: Some("Always".to_owned()),
                        ..Default::default()
                    }]),
                }),
            }),
        }),
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
