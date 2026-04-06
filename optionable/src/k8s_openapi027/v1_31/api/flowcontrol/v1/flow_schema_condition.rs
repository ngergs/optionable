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
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::FlowSchemaCondition {
    type Optioned = FlowSchemaConditionAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaConditionAc {
    type Optioned = FlowSchemaConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::FlowSchemaCondition {
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
        crate::OptionableConvert::merge(
            &mut self.last_transition_time,
            other.last_transition_time,
        )?;
        self.message = other.message;
        self.reason = other.reason;
        self.status = other.status;
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::FlowSchemaCondition>
for FlowSchemaConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::FlowSchemaCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1::FlowSchemaCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::FlowSchemaCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
