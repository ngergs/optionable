use kube::CustomResource;
use optionable::kube::{optionable_kube, optionable_kube_cr};
use optionable::Optionable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// Example how to use `ApplyConfigurations` with a CRD

// The attribute macros `optionable_kube_cr` and `optionable_kube` append to the derive attribute macros
// and have to be placed before them.
// If you prefer an explicit version of the `Optionable` derive macro configuration: the documentation
// of the attribute macros shows what they resolve to.

pub const FIELD_MANAGER: &str = "rust-manager";

#[optionable_kube_cr]
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[kube(
    group = "example.localhost",
    version = "v1",
    kind = "CustomCrd",
    namespaced
)]
pub struct CustomCrdSpec {
    pub msg: String,
    pub template: CustomCrdSpecTemplate,
}

#[optionable_kube]
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomCrdSpecTemplate {
    // example for a rename use case as `type` is a reserved keyword in rust
    #[serde(rename = "type")]
    pub type_: CustomCrdSpecTemplateType,
}

#[optionable_kube]
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum CustomCrdSpecTemplateType {
    V1alpha1,
    V1beta1,
    V1,
}
