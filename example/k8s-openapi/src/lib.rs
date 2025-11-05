use kube::CustomResource;
use optionable::{Optionable, OptionableKubeCrd};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Example how to use `ApplyConfigurations` with a CRD
#[derive(Optionable, CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(derive = "OptionableKubeCrd")]
#[optionable(k8s_openapi)]
#[kube(
    group = "example.localhost",
    version = "v1",
    kind = "MyCrd",
    namespaced
)]
pub struct MyCrdSpec {
    pub msg: String,
    pub template: MyCrdSpecTemplate,
}

#[derive(Optionable, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[optionable(k8s_openapi)]
pub struct MyCrdSpecTemplate {
    pub replicas: u32,
}
