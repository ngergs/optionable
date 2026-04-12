#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FlowSchemaCondition describes conditions for a FlowSchema.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowSchemaConditionAc {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaCondition {
    type Optioned = FlowSchemaConditionAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaConditionAc {
    type Optioned = FlowSchemaConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaCondition {
    fn into_optioned(self) -> FlowSchemaConditionAc {
        FlowSchemaConditionAc {
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: self.message,
            reason: self.reason,
            status: self.status,
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: FlowSchemaConditionAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: FlowSchemaConditionAc) -> Result<(), crate::Error> {
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
        if self.reason.is_none() {
            self.reason = other.reason;
        }
        if let Some(other_value) = other.reason {
            crate::OptionableConvert::merge(&mut self.reason, other_value)?;
        }
        if self.status.is_none() {
            self.status = other.status;
        }
        if let Some(other_value) = other.status {
            crate::OptionableConvert::merge(&mut self.status, other_value)?;
        }
        if self.type_.is_none() {
            self.type_ = other.type_;
        }
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaCondition,
> for FlowSchemaConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1beta3::FlowSchemaCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
