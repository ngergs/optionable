pub struct PriorityLevelConfigurationStatusAc {
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationCondition,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationStatus {
    type Optioned = PriorityLevelConfigurationStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationStatusAc {
    type Optioned = PriorityLevelConfigurationStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationStatus {
    fn into_optioned(self) -> PriorityLevelConfigurationStatusAc {
        PriorityLevelConfigurationStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
