#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JobSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_limit_per_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_mode: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_selector: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_indexes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_failure_policy: Option<
        <::k8s_openapi027::api::batch::v1::PodFailurePolicy as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_replacement_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_policy: Option<
        <::k8s_openapi027::api::batch::v1::SuccessPolicy as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds_after_finished: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::JobSpec {
    type Optioned = JobSpecAc;
}
#[automatically_derived]
impl crate::Optionable for JobSpecAc {
    type Optioned = JobSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::JobSpec {
    fn into_optioned(self) -> JobSpecAc {
        JobSpecAc {
            active_deadline_seconds: self.active_deadline_seconds,
            backoff_limit: self.backoff_limit,
            backoff_limit_per_index: self.backoff_limit_per_index,
            completion_mode: self.completion_mode,
            completions: self.completions,
            managed_by: self.managed_by,
            manual_selector: self.manual_selector,
            max_failed_indexes: self.max_failed_indexes,
            parallelism: self.parallelism,
            pod_failure_policy: crate::OptionableConvert::into_optioned(
                self.pod_failure_policy,
            ),
            pod_replacement_policy: self.pod_replacement_policy,
            selector: crate::OptionableConvert::into_optioned(self.selector),
            success_policy: crate::OptionableConvert::into_optioned(self.success_policy),
            suspend: self.suspend,
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
            ttl_seconds_after_finished: self.ttl_seconds_after_finished,
        }
    }
    fn try_from_optioned(value: JobSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active_deadline_seconds: value.active_deadline_seconds,
            backoff_limit: value.backoff_limit,
            backoff_limit_per_index: value.backoff_limit_per_index,
            completion_mode: value.completion_mode,
            completions: value.completions,
            managed_by: value.managed_by,
            manual_selector: value.manual_selector,
            max_failed_indexes: value.max_failed_indexes,
            parallelism: value.parallelism,
            pod_failure_policy: crate::OptionableConvert::try_from_optioned(
                value.pod_failure_policy,
            )?,
            pod_replacement_policy: value.pod_replacement_policy,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
            success_policy: crate::OptionableConvert::try_from_optioned(
                value.success_policy,
            )?,
            suspend: value.suspend,
            template: crate::OptionableConvert::try_from_optioned(
                value
                    .template
                    .ok_or(crate::Error {
                        missing_field: "template",
                    })?,
            )?,
            ttl_seconds_after_finished: value.ttl_seconds_after_finished,
        })
    }
    fn merge(&mut self, other: JobSpecAc) -> Result<(), crate::Error> {
        self.active_deadline_seconds = other.active_deadline_seconds;
        self.backoff_limit = other.backoff_limit;
        self.backoff_limit_per_index = other.backoff_limit_per_index;
        self.completion_mode = other.completion_mode;
        self.completions = other.completions;
        self.managed_by = other.managed_by;
        self.manual_selector = other.manual_selector;
        self.max_failed_indexes = other.max_failed_indexes;
        self.parallelism = other.parallelism;
        crate::OptionableConvert::merge(
            &mut self.pod_failure_policy,
            other.pod_failure_policy,
        )?;
        self.pod_replacement_policy = other.pod_replacement_policy;
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        crate::OptionableConvert::merge(&mut self.success_policy, other.success_policy)?;
        self.suspend = other.suspend;
        if let Some(other_value) = other.template {
            crate::OptionableConvert::merge(&mut self.template, other_value)?;
        }
        self.ttl_seconds_after_finished = other.ttl_seconds_after_finished;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::JobSpec> for JobSpecAc {
    fn from_optionable(value: k8s_openapi027::api::batch::v1::JobSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::JobSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::JobSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
