#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Describes the state of the storageVersion at a certain point.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageVersionConditionAc {
    /// Last time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// A human readable message indicating details about the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// If set, this represents the .metadata.generation that the condition was set based upon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// The reason for the condition's last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// Type of the condition.
    #[serde(rename = "type")]
    pub type_: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition {
    type Optioned = StorageVersionConditionAc;
}
#[automatically_derived]
impl crate::Optionable for StorageVersionConditionAc {
    type Optioned = StorageVersionConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition {
    fn into_optioned(self) -> StorageVersionConditionAc {
        StorageVersionConditionAc {
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: Some(self.message),
            observed_generation: self.observed_generation,
            reason: Some(self.reason),
            status: Some(self.status),
            type_: self.type_,
        }
    }
    fn try_from_optioned(
        value: StorageVersionConditionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            message: value
                .message
                .ok_or(crate::Error {
                    missing_field: "message",
                })?,
            observed_generation: value.observed_generation,
            reason: value
                .reason
                .ok_or(crate::Error {
                    missing_field: "reason",
                })?,
            status: value
                .status
                .ok_or(crate::Error {
                    missing_field: "status",
                })?,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: StorageVersionConditionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.last_transition_time,
            other.last_transition_time,
        )?;
        if let Some(other_value) = other.message {
            self.message = other_value;
        }
        self.observed_generation = other.observed_generation;
        if let Some(other_value) = other.reason {
            self.reason = other_value;
        }
        if let Some(other_value) = other.status {
            self.status = other_value;
        }
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition,
> for StorageVersionConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apiserverinternal::v1alpha1::StorageVersionCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
