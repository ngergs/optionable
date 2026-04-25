#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// JobCondition describes current state of a job.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JobConditionAc {
    /// Last time the condition was checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Last time the condition transit from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Human readable message indicating details about last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// (brief) reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// Type of job condition, Complete or Failed.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::JobCondition {
    type Optioned = JobConditionAc;
}
#[automatically_derived]
impl crate::Optionable for JobConditionAc {
    type Optioned = JobConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::JobCondition {
    fn into_optioned(self) -> JobConditionAc {
        JobConditionAc {
            last_probe_time: crate::OptionableConvert::into_optioned(
                self.last_probe_time,
            ),
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: self.message,
            reason: self.reason,
            status: Some(self.status),
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(value: JobConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            last_probe_time: crate::OptionableConvert::try_from_optioned(
                value.last_probe_time,
            )?,
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            message: value.message,
            reason: value.reason,
            status: value
                .status
                .ok_or(crate::Error {
                    missing_field: "status",
                })?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: JobConditionAc) -> Result<(), crate::Error> {
        if self.last_probe_time.is_none() {
            self.last_probe_time = crate::OptionableConvert::try_from_optioned(
                other.last_probe_time,
            )?;
        } else if let Some(self_value) = self.last_probe_time.as_mut()
            && let Some(other_value) = other.last_probe_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.last_transition_time.is_none() {
            self.last_transition_time = crate::OptionableConvert::try_from_optioned(
                other.last_transition_time,
            )?;
        } else if let Some(self_value) = self.last_transition_time.as_mut()
            && let Some(other_value) = other.last_transition_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else if let Some(self_value) = self.message.as_mut()
            && let Some(other_value) = other.message
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = crate::OptionableConvert::try_from_optioned(other.reason)?;
        } else if let Some(self_value) = self.reason.as_mut()
            && let Some(other_value) = other.reason
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.status {
            self.status = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::JobCondition>
for JobConditionAc {
    fn from_optionable(value: k8s_openapi027::api::batch::v1::JobCondition) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::JobCondition, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::JobCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for JobConditionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.last_probe_time,
            other.last_probe_time,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.last_transition_time,
            other.last_transition_time,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.message, other.message);
        k8s_openapi027::DeepMerge::merge_from(&mut self.reason, other.reason);
        k8s_openapi027::DeepMerge::merge_from(&mut self.status, other.status);
        k8s_openapi027::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}
