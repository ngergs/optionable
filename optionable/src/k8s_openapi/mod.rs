#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use k8s_openapi::Resource;
use serde::ser::SerializeMap;
use serde::Serializer;
use std::marker::PhantomData;

mod optionable;
#[cfg(feature = "k8s_openapi_v1_34")]
pub mod v1_34;

/// Serializes a PhantomData marker to add the API envelope fields `apiVersion` and `kind`.
/// Intended use is together with `#[serde(flatten)]` for the marker field.
pub(crate) fn serialize_api_envelope<S: Serializer, R: Resource>(
    _: &PhantomData<R>,
    s: S,
) -> Result<S::Ok, S::Error> {
    let mut map = s.serialize_map(Some(2))?;
    map.serialize_entry("apiVersion", R::API_VERSION)?;
    map.serialize_entry("kind", R::KIND)?;
    map.end()
}

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
