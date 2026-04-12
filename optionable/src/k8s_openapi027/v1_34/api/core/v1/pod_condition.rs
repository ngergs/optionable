#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodCondition contains details for the current condition of this pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodConditionAc {
    /// Last time we probed the condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Last time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Human-readable message indicating details about last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// If set, this represents the .metadata.generation that the pod condition was set based upon. This is an alpha field. Enable PodObservedGenerationTracking to be able to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// Type is the type of the condition. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    #[serde(rename = "type")]
    pub type_: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodCondition {
    type Optioned = PodConditionAc;
}
#[automatically_derived]
impl crate::Optionable for PodConditionAc {
    type Optioned = PodConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodCondition {
    fn into_optioned(self) -> PodConditionAc {
        PodConditionAc {
            last_probe_time: crate::OptionableConvert::into_optioned(
                self.last_probe_time,
            ),
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: self.message,
            observed_generation: self.observed_generation,
            reason: self.reason,
            status: Some(self.status),
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: PodConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            last_probe_time: crate::OptionableConvert::try_from_optioned(
                value.last_probe_time,
            )?,
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            message: value.message,
            observed_generation: value.observed_generation,
            reason: value.reason,
            status: value
                .status
                .ok_or(crate::Error {
                    missing_field: "status",
                })?,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: PodConditionAc) -> Result<(), crate::Error> {
        if self.last_probe_time.is_none() {
            self.last_probe_time = other.last_probe_time;
        }
        if let Some(other_value) = other.last_probe_time {
            crate::OptionableConvert::merge(&mut self.last_probe_time, other_value)?;
        }
        if self.last_transition_time.is_none() {
            self.last_transition_time = other.last_transition_time;
        }
        if let Some(other_value) = other.last_transition_time {
            crate::OptionableConvert::merge(
                &mut self.last_transition_time,
                other_value,
            )?;
        }
        if self.message.is_none() {
            self.message = other.message;
        }
        if let Some(other_value) = other.message {
            crate::OptionableConvert::merge(&mut self.message, other_value)?;
        }
        if self.observed_generation.is_none() {
            self.observed_generation = other.observed_generation;
        }
        if let Some(other_value) = other.observed_generation {
            crate::OptionableConvert::merge(&mut self.observed_generation, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = other.reason;
        }
        if let Some(other_value) = other.reason {
            crate::OptionableConvert::merge(&mut self.reason, other_value)?;
        }
        if let Some(other_value) = other.status {
            self.status = other_value;
        }
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::PodCondition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.type_ == other.type_
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodCondition>
for PodConditionAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodCondition) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodCondition, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
