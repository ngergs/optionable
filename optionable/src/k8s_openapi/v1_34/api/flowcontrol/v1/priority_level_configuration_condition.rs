pub struct PriorityLevelConfigurationConditionOpt {
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
    type Optioned = PriorityLevelConfigurationConditionOpt;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationConditionOpt {
    type Optioned = PriorityLevelConfigurationConditionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::PriorityLevelConfigurationCondition {
    fn into_optioned(self) -> PriorityLevelConfigurationConditionOpt {
        PriorityLevelConfigurationConditionOpt {
            last_transition_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.last_transition_time),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
            status: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.status),
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationConditionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            last_transition_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(
                value.last_transition_time,
            )?,
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
            status: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.status)?,
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.type_)?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationConditionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.last_transition_time,
            other.last_transition_time,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.status, other.status)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
