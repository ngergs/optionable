#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = ComponentStatus)]
pub struct ComponentStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ComponentCondition>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ComponentStatus {
    type Optioned = ComponentStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ComponentStatusAc {
    type Optioned = ComponentStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ComponentStatus {
    fn into_optioned(self) -> ComponentStatusAc {
        ComponentStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(
        value: ComponentStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            metadata: value.metadata,
        })
    }
    fn merge(
        &mut self,
        other: ComponentStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.metadata = other.metadata;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::core::v1::ComponentStatus;
