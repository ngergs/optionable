pub struct JobConditionOpt {
    pub last_probe_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub last_transition_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::job_condition::JobCondition {
    type Optioned = JobConditionOpt;
}
#[automatically_derived]
impl crate::Optionable for JobConditionOpt {
    type Optioned = JobConditionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::batch::v1::job_condition::JobCondition {
    fn into_optioned(self) -> JobConditionOpt {
        JobConditionOpt {
            last_probe_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.last_probe_time),
            last_transition_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.last_transition_time),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
            status: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.status,
                ),
            ),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: JobConditionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            last_probe_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.last_probe_time)?,
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
            status: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: JobConditionOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.last_probe_time,
            other.last_probe_time,
        )?;
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
        if let Some(other_value) = other.status {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.status,
                other_value,
            )?;
        }
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
