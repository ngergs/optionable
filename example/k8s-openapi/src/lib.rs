use kube::CustomResource;
use optionable::{optionable_kube, optionable_kube_cr, Optionable, OptionableKubeCrd};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Example how to use `ApplyConfigurations` with a CRD

// The attribute macros `optionable_kube_cr` and `optionable_kube` append to the derive attribute macros
// and have to be placed before them.
// If you prefer an explicit version of the `Optionable` derive macro configuration: the documentation
// of the attribute macros shows what they resolve to.

#[optionable_kube_cr]
#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
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

#[optionable_kube]
#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct MyCrdSpecTemplate {
    pub replicas: u32,
}
