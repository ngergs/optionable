#[derive(kube::Resource)]
#[resource(inherit = "k8s_openapi::api::apps::v1::Deployment")]
pub struct DeploymentOpt {
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<::k8s_openapi::api::apps::v1::DeploymentSpec> as crate::Optionable>::Optioned,
    pub status:
        <Option<::k8s_openapi::api::apps::v1::DeploymentStatus> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::Deployment {
    type Optioned = DeploymentOpt;
}
#[automatically_derived]
impl crate::Optionable for DeploymentOpt {
    type Optioned = DeploymentOpt;
}
