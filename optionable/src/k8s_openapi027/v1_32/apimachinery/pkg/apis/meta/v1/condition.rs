#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Condition contains details for one aspect of the current state of this API Resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConditionAc {
    /// lastTransitionTime is the last time the condition transitioned from one status to another. This should be when the underlying condition changed.  If that is not known, then using the time when the API field changed is acceptable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// message is a human readable message indicating details about the transition. This may be an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// observedGeneration represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.conditions\[x\].observedGeneration is 9, the condition is out of date with respect to the current state of the instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// reason contains a programmatic identifier indicating the reason for the condition's last transition. Producers of specific condition types may define expected values and meanings for this field, and whether the values are considered a guaranteed API. The value should be a CamelCase string. This field may not be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// status of the condition, one of True, False, Unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// type of condition in CamelCase or in foo.example.com/CamelCase.
    #[serde(rename = "type")]
    pub type_: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition {
    type Optioned = ConditionAc;
}
#[automatically_derived]
impl crate::Optionable for ConditionAc {
    type Optioned = ConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition {
    fn into_optioned(self) -> ConditionAc {
        ConditionAc {
            last_transition_time: Some(
                crate::OptionableConvert::into_optioned(self.last_transition_time),
            ),
            message: Some(self.message),
            observed_generation: self.observed_generation,
            reason: Some(self.reason),
            status: Some(self.status),
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: ConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value
                    .last_transition_time
                    .ok_or(crate::Error {
                        missing_field: "last_transition_time",
                    })?,
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
    fn merge(&mut self, other: ConditionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.last_transition_time {
            self.last_transition_time = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.message {
            self.message = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.observed_generation.is_none() {
            self.observed_generation = crate::OptionableConvert::try_from_optioned(
                other.observed_generation,
            )?;
        } else if let Some(self_value) = self.observed_generation.as_mut()
            && let Some(other_value) = other.observed_generation
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.reason {
            self.reason = crate::OptionableConvert::try_from_optioned(other_value)?;
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
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.type_ == other.type_
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition>
for ConditionAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
