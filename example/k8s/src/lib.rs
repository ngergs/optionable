// Example how to use `ApplyConfigurations` derived from `optionable` with a CRD

use kube::CustomResource;
use optionable::Optionable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const FIELD_MANAGER: &str = "rust-manager";

// Repeating the derives for the spec and subfields as well as for `optionable` can be cumbersome
// so let's define a simple helper macro for that
macro_rules! with_common_derives {
    ($item:item) => {
        #[derive(Optionable, Clone, Debug, Deserialize, Serialize, JsonSchema)]
        #[optionable(kube(), derive(Clone, Debug, Default, Deserialize, Serialize))]
        #[serde(rename_all = "camelCase")]
        $item
    };
}

with_common_derives! {
    #[derive(CustomResource)]
    #[kube(
        group = "example.localhost",
        version = "v1",
        kind = "CustomCrd",
        namespaced,
        // Also derive `optionable` for the root type
        derive = "Optionable",
        // Instruct `optionable` to derive the traits and specialized handling for the root type. Detailed docs are here:
        // https://docs.rs/optionable/latest/optionable/derive.Optionable.html
        attr = "optionable(kube(resource), derive(Clone, Debug, Default, Deserialize, Serialize))",
    )]
    pub struct CustomCrdSpec {
        pub msg: String,
        pub template: CustomCrdSpecTemplate,
    }
}

with_common_derives! {
    pub struct CustomCrdSpecTemplate {
        // example for a rename use case as `type` is a reserved keyword in rust
        #[serde(rename = "type")]
        pub type_: CustomCrdSpecTemplateType,
        pub body: String,
    }
}

with_common_derives! {
    pub enum CustomCrdSpecTemplateType {
        V1alpha1,
        V1beta1,
        // To derive defaults for the optioned `ApplyConfigurations` we have to forward
        // to the optioned type which value should be the default
        #[optionable_attr(default)]
        V1,
    }
}
