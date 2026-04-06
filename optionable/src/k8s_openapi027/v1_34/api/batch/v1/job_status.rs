#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JobStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_indexes: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi027::api::batch::v1::JobCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_indexes: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncounted_terminated_pods: <Option<
        ::k8s_openapi027::api::batch::v1::UncountedTerminatedPods,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::JobStatus {
    type Optioned = JobStatusAc;
}
#[automatically_derived]
impl crate::Optionable for JobStatusAc {
    type Optioned = JobStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::JobStatus {
    fn into_optioned(self) -> JobStatusAc {
        JobStatusAc {
            active: self.active,
            completed_indexes: crate::OptionableConvert::into_optioned(
                self.completed_indexes,
            ),
            completion_time: crate::OptionableConvert::into_optioned(
                self.completion_time,
            ),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            failed: self.failed,
            failed_indexes: crate::OptionableConvert::into_optioned(self.failed_indexes),
            ready: self.ready,
            start_time: crate::OptionableConvert::into_optioned(self.start_time),
            succeeded: self.succeeded,
            terminating: self.terminating,
            uncounted_terminated_pods: crate::OptionableConvert::into_optioned(
                self.uncounted_terminated_pods,
            ),
        }
    }
    fn try_from_optioned(value: JobStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active: value.active,
            completed_indexes: crate::OptionableConvert::try_from_optioned(
                value.completed_indexes,
            )?,
            completion_time: crate::OptionableConvert::try_from_optioned(
                value.completion_time,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            failed: value.failed,
            failed_indexes: crate::OptionableConvert::try_from_optioned(
                value.failed_indexes,
            )?,
            ready: value.ready,
            start_time: crate::OptionableConvert::try_from_optioned(value.start_time)?,
            succeeded: value.succeeded,
            terminating: value.terminating,
            uncounted_terminated_pods: crate::OptionableConvert::try_from_optioned(
                value.uncounted_terminated_pods,
            )?,
        })
    }
    fn merge(&mut self, other: JobStatusAc) -> Result<(), crate::Error> {
        self.active = other.active;
        crate::OptionableConvert::merge(
            &mut self.completed_indexes,
            other.completed_indexes,
        )?;
        crate::OptionableConvert::merge(
            &mut self.completion_time,
            other.completion_time,
        )?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.failed = other.failed;
        crate::OptionableConvert::merge(&mut self.failed_indexes, other.failed_indexes)?;
        self.ready = other.ready;
        crate::OptionableConvert::merge(&mut self.start_time, other.start_time)?;
        self.succeeded = other.succeeded;
        self.terminating = other.terminating;
        crate::OptionableConvert::merge(
            &mut self.uncounted_terminated_pods,
            other.uncounted_terminated_pods,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::JobStatus> for JobStatusAc {
    fn from_optionable(value: k8s_openapi027::api::batch::v1::JobStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::JobStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::JobStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
