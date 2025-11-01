pub struct PriorityLevelConfigurationConditionAc {
    pub last_transition_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub status: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationCondition {
    type Optioned = PriorityLevelConfigurationConditionAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationConditionAc {
    type Optioned = PriorityLevelConfigurationConditionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationCondition {
    fn into_optioned(self) -> PriorityLevelConfigurationConditionAc {
        PriorityLevelConfigurationConditionAc {
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: crate::OptionableConvert::into_optioned(self.message),
            reason: crate::OptionableConvert::into_optioned(self.reason),
            status: crate::OptionableConvert::into_optioned(self.status),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationConditionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationConditionAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.last_transition_time,
            other.last_transition_time,
        )?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
