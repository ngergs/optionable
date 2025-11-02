#![allow(clippy::all)]
#![allow(clippy::pedantic)]

#[cfg(feature = "k8s-openapi_v1_34")]
mod optionable;
#[cfg(feature = "k8s-openapi_v1_34")]
mod v1_34;

/*mod test {
    use crate::k8s_openapi::v1_34::api::apps::v1::{DeploymentAc, DeploymentSpecAc};
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    use kube::api::{Patch, PatchParams};
    use kube::{Api, Client};

    #[test]
    fn deployment_opt() {
        let client: Client = Client::try_default().unwrap();
        let deployments: Api<DeploymentAc> = Api::default_namespaced(client);
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
        let params = PatchParams::apply("test-manager");
        deployments
            .patch(
                &patch.metadata.name.unwrap(),
                &PatchParams::apply("test-manager"),
                &Patch::Apply(patch),
            )
            .await
            .unwrap();
    }
}*/
