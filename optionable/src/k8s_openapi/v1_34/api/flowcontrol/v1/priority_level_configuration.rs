#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = PriorityLevelConfiguration)]
pub struct PriorityLevelConfigurationAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationSpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfiguration {
    type Optioned = PriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationAc {
    type Optioned = PriorityLevelConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfiguration {
    fn into_optioned(self) -> PriorityLevelConfigurationAc {
        PriorityLevelConfigurationAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfiguration;
