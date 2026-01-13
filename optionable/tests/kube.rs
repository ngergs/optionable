#![cfg(feature = "kube")]

use k8s_openapi027::apimachinery::pkg::apis::meta::v1::{FieldsV1, ManagedFieldsEntry, ObjectMeta};
use kube::CustomResource;
use optionable::kube::ExtractManagedFields;
use optionable::{Optionable, OptionableConvert, OptionedConvert};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Repeating the derives for the spec and subfields as well as for `optionable` can be cumbersome
// so let's define a simple helper macro for that
macro_rules! with_common_derives {
    ($item:item) => {
        #[derive(Optionable, Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq)]
        #[optionable(
            kube(),
            derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)
        )]
        #[serde(rename_all = "camelCase")]
        $item
    };
}

with_common_derives! {
    #[derive(CustomResource)]
    #[kube(
        crates(k8s_openapi = "::k8s_openapi027"),
        group = "example.localhost",
        version = "v1",
        kind = "CustomCrd",
        namespaced,
        // Also derive `optionable` for the root type
        derive = "Optionable",
        derive = "PartialEq",
        // Instruct `optionable` to derive the traits and specialized handling for the root type. Detailed docs are here:
        // https://docs.rs/optionable/latest/optionable/derive.Optionable.html
        attr = "optionable(kube(resource, k8s_openapi_crate=\"::k8s_openapi027\"), derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq))",
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

#[test]
fn kube_crd() {
    let field_manager = "mgr".to_owned();
    let my_cr = CustomCrd {
        metadata: ObjectMeta {
            name: Some("hello".to_owned()),
            managed_fields: Some(vec![ManagedFieldsEntry {
                api_version: None,
                fields_type: Some("FieldsV1".to_string()),
                fields_v1: Some(FieldsV1(json!( {
                    "f:spec": {
                        "f:template": {
                            "f:type": {}
                        }
                    }
                }))),
                manager: Some(field_manager.clone()),
                ..Default::default()
            }]),
            ..Default::default()
        },
        spec: CustomCrdSpec {
            msg: "hello".to_owned(),
            template: CustomCrdSpecTemplate {
                body: "world".to_owned(),
                type_: CustomCrdSpecTemplateType::V1alpha1,
            },
        },
    };

    let my_cr_optioned: CustomCrdAc = my_cr.clone().into_optioned();
    assert_eq!(my_cr, my_cr_optioned.try_into_optionable().unwrap());
    let my_cr_managed = my_cr.clone().extract(&field_manager).unwrap();
    let my_cr_managed_expected = Some(CustomCrdAc {
        metadata: ObjectMeta {
            name: my_cr.metadata.name,
            ..Default::default()
        },
        spec: Some(CustomCrdSpecAc {
            template: Some(CustomCrdSpecTemplateAc {
                type_: Some(my_cr.spec.template.type_.into_optioned()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    });
    assert_eq!(my_cr_managed, my_cr_managed_expected);
}
