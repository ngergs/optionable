#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeCondition contains condition information for a node.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeConditionAc {
    /// Last time we got an update on a given condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_time: Option<
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
    /// Type of node condition.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeCondition {
    type Optioned = NodeConditionAc;
}
#[automatically_derived]
impl crate::Optionable for NodeConditionAc {
    type Optioned = NodeConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeCondition {
    fn into_optioned(self) -> NodeConditionAc {
        NodeConditionAc {
            last_heartbeat_time: crate::OptionableConvert::into_optioned(
                self.last_heartbeat_time,
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
    fn try_from_optioned(value: NodeConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            last_heartbeat_time: crate::OptionableConvert::try_from_optioned(
                value.last_heartbeat_time,
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
    fn merge(&mut self, other: NodeConditionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.last_heartbeat_time,
            other.last_heartbeat_time,
        )?;
        crate::OptionableConvert::merge(
            &mut self.last_transition_time,
            other.last_transition_time,
        )?;
        self.message = other.message;
        self.reason = other.reason;
        if let Some(other_value) = other.status {
            self.status = other_value;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeCondition>
for NodeConditionAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeCondition) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeCondition, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
