pub struct PriorityLevelConfigurationStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationCondition,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationStatus {
    type Optioned = PriorityLevelConfigurationStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationStatusOpt {
    type Optioned = PriorityLevelConfigurationStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationStatus {
    fn into_optioned(self) -> PriorityLevelConfigurationStatusOpt {
        PriorityLevelConfigurationStatusOpt {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
