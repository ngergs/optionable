#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PriorityLevelConfigurationCondition defines the condition of priority level.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriorityLevelConfigurationConditionAc {
    /// `lastTransitionTime` is the last time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// `message` is a human-readable message indicating details about last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// `reason` is a unique, one-word, CamelCase reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// `status` is the status of the condition. Can be True, False, Unknown. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// `type` is the type of the condition. Required.
    #[serde(rename = "type")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationCondition {
    type Optioned = PriorityLevelConfigurationConditionAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationConditionAc {
    type Optioned = PriorityLevelConfigurationConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationCondition {
    fn into_optioned(self) -> PriorityLevelConfigurationConditionAc {
        PriorityLevelConfigurationConditionAc {
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: self.message,
            reason: self.reason,
            status: self.status,
            type_: self.type_,
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationConditionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            message: value.message,
            reason: value.reason,
            status: value.status,
            type_: value.type_,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationConditionAc,
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
        if self.status.is_none() {
            self.status = crate::OptionableConvert::try_from_optioned(other.status)?;
        } else if let Some(self_value) = self.status.as_mut()
            && let Some(other_value) = other.status
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationCondition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.type_ == other.type_
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationCondition,
> for PriorityLevelConfigurationConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::PriorityLevelConfigurationCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
