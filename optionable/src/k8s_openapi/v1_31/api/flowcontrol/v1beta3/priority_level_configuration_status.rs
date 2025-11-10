#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelConfigurationStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::flowcontrol::v1beta3::PriorityLevelConfigurationCondition,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1beta3::PriorityLevelConfigurationStatus {
    type Optioned = PriorityLevelConfigurationStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationStatusAc {
    type Optioned = PriorityLevelConfigurationStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1beta3::PriorityLevelConfigurationStatus {
    fn into_optioned(self) -> PriorityLevelConfigurationStatusAc {
        PriorityLevelConfigurationStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationStatusAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
