#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct JobSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_limit: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_limit_per_index: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_selector: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_indexes: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_failure_policy: <Option<
        ::k8s_openapi::api::batch::v1::PodFailurePolicy,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_replacement_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_policy: <Option<
        ::k8s_openapi::api::batch::v1::SuccessPolicy,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds_after_finished: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::JobSpec {
    type Optioned = JobSpecAc;
}
#[automatically_derived]
impl crate::Optionable for JobSpecAc {
    type Optioned = JobSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::JobSpec {
    fn into_optioned(self) -> JobSpecAc {
        JobSpecAc {
            active_deadline_seconds: crate::OptionableConvert::into_optioned(
                self.active_deadline_seconds,
            ),
            backoff_limit: crate::OptionableConvert::into_optioned(self.backoff_limit),
            backoff_limit_per_index: crate::OptionableConvert::into_optioned(
                self.backoff_limit_per_index,
            ),
            completion_mode: crate::OptionableConvert::into_optioned(
                self.completion_mode,
            ),
            completions: crate::OptionableConvert::into_optioned(self.completions),
            managed_by: crate::OptionableConvert::into_optioned(self.managed_by),
            manual_selector: crate::OptionableConvert::into_optioned(
                self.manual_selector,
            ),
            max_failed_indexes: crate::OptionableConvert::into_optioned(
                self.max_failed_indexes,
            ),
            parallelism: crate::OptionableConvert::into_optioned(self.parallelism),
            pod_failure_policy: crate::OptionableConvert::into_optioned(
                self.pod_failure_policy,
            ),
            pod_replacement_policy: crate::OptionableConvert::into_optioned(
                self.pod_replacement_policy,
            ),
            selector: crate::OptionableConvert::into_optioned(self.selector),
            success_policy: crate::OptionableConvert::into_optioned(self.success_policy),
            suspend: crate::OptionableConvert::into_optioned(self.suspend),
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
            ttl_seconds_after_finished: crate::OptionableConvert::into_optioned(
                self.ttl_seconds_after_finished,
            ),
        }
    }
    fn try_from_optioned(value: JobSpecAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            active_deadline_seconds: crate::OptionableConvert::try_from_optioned(
                value.active_deadline_seconds,
            )?,
            backoff_limit: crate::OptionableConvert::try_from_optioned(
                value.backoff_limit,
            )?,
            backoff_limit_per_index: crate::OptionableConvert::try_from_optioned(
                value.backoff_limit_per_index,
            )?,
            completion_mode: crate::OptionableConvert::try_from_optioned(
                value.completion_mode,
            )?,
            completions: crate::OptionableConvert::try_from_optioned(value.completions)?,
            managed_by: crate::OptionableConvert::try_from_optioned(value.managed_by)?,
            manual_selector: crate::OptionableConvert::try_from_optioned(
                value.manual_selector,
            )?,
            max_failed_indexes: crate::OptionableConvert::try_from_optioned(
                value.max_failed_indexes,
            )?,
            parallelism: crate::OptionableConvert::try_from_optioned(value.parallelism)?,
            pod_failure_policy: crate::OptionableConvert::try_from_optioned(
                value.pod_failure_policy,
            )?,
            pod_replacement_policy: crate::OptionableConvert::try_from_optioned(
                value.pod_replacement_policy,
            )?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
            success_policy: crate::OptionableConvert::try_from_optioned(
                value.success_policy,
            )?,
            suspend: crate::OptionableConvert::try_from_optioned(value.suspend)?,
            template: crate::OptionableConvert::try_from_optioned(
                value
                    .template
                    .ok_or(crate::optionable::Error {
                        missing_field: "template",
                    })?,
            )?,
            ttl_seconds_after_finished: crate::OptionableConvert::try_from_optioned(
                value.ttl_seconds_after_finished,
            )?,
        })
    }
    fn merge(&mut self, other: JobSpecAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.active_deadline_seconds,
            other.active_deadline_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.backoff_limit, other.backoff_limit)?;
        crate::OptionableConvert::merge(
            &mut self.backoff_limit_per_index,
            other.backoff_limit_per_index,
        )?;
        crate::OptionableConvert::merge(
            &mut self.completion_mode,
            other.completion_mode,
        )?;
        crate::OptionableConvert::merge(&mut self.completions, other.completions)?;
        crate::OptionableConvert::merge(&mut self.managed_by, other.managed_by)?;
        crate::OptionableConvert::merge(
            &mut self.manual_selector,
            other.manual_selector,
        )?;
        crate::OptionableConvert::merge(
            &mut self.max_failed_indexes,
            other.max_failed_indexes,
        )?;
        crate::OptionableConvert::merge(&mut self.parallelism, other.parallelism)?;
        crate::OptionableConvert::merge(
            &mut self.pod_failure_policy,
            other.pod_failure_policy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.pod_replacement_policy,
            other.pod_replacement_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        crate::OptionableConvert::merge(&mut self.success_policy, other.success_policy)?;
        crate::OptionableConvert::merge(&mut self.suspend, other.suspend)?;
        if let Some(other_value) = other.template {
            crate::OptionableConvert::merge(&mut self.template, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.ttl_seconds_after_finished,
            other.ttl_seconds_after_finished,
        )?;
        Ok(())
    }
}
