pub struct JobStatusOpt {
    pub active: <Option<i32> as crate::Optionable>::Optioned,
    pub completed_indexes: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub completion_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::batch::v1::JobCondition>,
    > as crate::Optionable>::Optioned,
    pub failed: <Option<i32> as crate::Optionable>::Optioned,
    pub failed_indexes: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ready: <Option<i32> as crate::Optionable>::Optioned,
    pub start_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub succeeded: <Option<i32> as crate::Optionable>::Optioned,
    pub terminating: <Option<i32> as crate::Optionable>::Optioned,
    pub uncounted_terminated_pods: <Option<
        ::k8s_openapi::api::batch::v1::UncountedTerminatedPods,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::JobStatus {
    type Optioned = JobStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for JobStatusOpt {
    type Optioned = JobStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::JobStatus {
    fn into_optioned(self) -> JobStatusOpt {
        JobStatusOpt {
            active: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.active),
            completed_indexes: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.completed_indexes),
            completion_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.completion_time),
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::batch::v1::JobCondition>,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            failed: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.failed),
            failed_indexes: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.failed_indexes),
            ready: <Option<i32> as crate::OptionableConvert>::into_optioned(self.ready),
            start_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.start_time),
            succeeded: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.succeeded),
            terminating: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.terminating),
            uncounted_terminated_pods: <Option<
                ::k8s_openapi::api::batch::v1::UncountedTerminatedPods,
            > as crate::OptionableConvert>::into_optioned(self.uncounted_terminated_pods),
        }
    }
    fn try_from_optioned(value: JobStatusOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            active: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.active)?,
            completed_indexes: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.completed_indexes)?,
            completion_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.completion_time)?,
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::batch::v1::JobCondition>,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            failed: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.failed)?,
            failed_indexes: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.failed_indexes)?,
            ready: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.ready)?,
            start_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.start_time)?,
            succeeded: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.succeeded)?,
            terminating: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.terminating)?,
            uncounted_terminated_pods: <Option<
                ::k8s_openapi::api::batch::v1::UncountedTerminatedPods,
            > as crate::OptionableConvert>::try_from_optioned(
                value.uncounted_terminated_pods,
            )?,
        })
    }
    fn merge(&mut self, other: JobStatusOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.active, other.active)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.completed_indexes,
            other.completed_indexes,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.completion_time,
            other.completion_time,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::batch::v1::JobCondition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.failed, other.failed)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.failed_indexes,
            other.failed_indexes,
        )?;
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.ready, other.ready)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(&mut self.start_time, other.start_time)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.succeeded, other.succeeded)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.terminating, other.terminating)?;
        <Option<
            ::k8s_openapi::api::batch::v1::UncountedTerminatedPods,
        > as crate::OptionableConvert>::merge(
            &mut self.uncounted_terminated_pods,
            other.uncounted_terminated_pods,
        )?;
        Ok(())
    }
}
