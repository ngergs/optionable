#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HorizontalPodAutoscalerCondition describes the state of a HorizontalPodAutoscaler at a certain point.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerConditionAc {
    /// lastTransitionTime is the last time the condition transitioned from one status to another
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// message is a human-readable explanation containing details about the transition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// reason is the reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// status is the status of the condition (True, False, Unknown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// type describes the current condition
    #[serde(rename = "type")]
    pub type_: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerCondition {
    type Optioned = HorizontalPodAutoscalerConditionAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerConditionAc {
    type Optioned = HorizontalPodAutoscalerConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerCondition {
    fn into_optioned(self) -> HorizontalPodAutoscalerConditionAc {
        HorizontalPodAutoscalerConditionAc {
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: self.message,
            reason: self.reason,
            status: Some(self.status),
            type_: self.type_,
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerConditionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
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
            type_: value.type_,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerConditionAc,
    ) -> Result<(), crate::Error> {
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
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerCondition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.type_ == other.type_
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerCondition,
> for HorizontalPodAutoscalerConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
