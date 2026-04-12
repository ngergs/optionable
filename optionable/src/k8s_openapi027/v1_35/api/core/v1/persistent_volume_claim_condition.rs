#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PersistentVolumeClaimCondition contains details about state of pvc
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistentVolumeClaimConditionAc {
    /// lastProbeTime is the time we probed the condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_probe_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// lastTransitionTime is the time the condition transitioned from one status to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// message is the human-readable message indicating details about last transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// reason is a unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports "Resizing" that means the underlying persistent volume is being resized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Status is the status of the condition. Can be True, False, Unknown. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=state%20of%20pvc-,conditions.status,-(string)%2C%20required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// Type is the type of the condition. More info: https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/persistent-volume-claim-v1/#:~:text=set%20to%20%27ResizeStarted%27.-,PersistentVolumeClaimCondition,-contains%20details%20about
    #[serde(rename = "type")]
    pub type_: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::PersistentVolumeClaimCondition {
    type Optioned = PersistentVolumeClaimConditionAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimConditionAc {
    type Optioned = PersistentVolumeClaimConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PersistentVolumeClaimCondition {
    fn into_optioned(self) -> PersistentVolumeClaimConditionAc {
        PersistentVolumeClaimConditionAc {
            last_probe_time: crate::OptionableConvert::into_optioned(
                self.last_probe_time,
            ),
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
        value: PersistentVolumeClaimConditionAc,
    ) -> Result<Self, crate::Error> {
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
            type_: value.type_,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimConditionAc,
    ) -> Result<(), crate::Error> {
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
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::core::v1::PersistentVolumeClaimCondition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.type_ == other.type_
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::PersistentVolumeClaimCondition,
> for PersistentVolumeClaimConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PersistentVolumeClaimCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::PersistentVolumeClaimCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PersistentVolumeClaimCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
