#![allow(clippy::all)]
#![allow(clippy::pedantic)]

#[cfg(feature = "k8s-openapi_v1_34")]
mod optionable;
#[cfg(feature = "k8s-openapi_v1_34")]
mod v1_34;

#[cfg(feature = "k8s-openapi_v1_34")]
#[cfg(test)]
mod test {
    use crate::k8s_openapi::v1_34::api::apps::v1::DeploymentOpt;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    #[test]
    fn deployment_opt() {
        let _ = DeploymentOpt {
            metadata: ObjectMeta {
                ..Default::default()
            },
            spec: None,
            status: None,
        };
    }
}
