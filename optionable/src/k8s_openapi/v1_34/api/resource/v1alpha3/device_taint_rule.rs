#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = DeviceTaintRule)]
pub struct DeviceTaintRuleAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::resource::v1alpha3::DeviceTaintRuleSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintRule {
    type Optioned = DeviceTaintRuleAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintRuleAc {
    type Optioned = DeviceTaintRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintRule {
    fn into_optioned(self) -> DeviceTaintRuleAc {
        DeviceTaintRuleAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(
        value: DeviceTaintRuleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceTaintRuleAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::resource::v1alpha3::DeviceTaintRule;
