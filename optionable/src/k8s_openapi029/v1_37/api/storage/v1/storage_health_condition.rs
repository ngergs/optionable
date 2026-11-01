#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StorageHealthCondition represents an adverse health condition reported by a CSI driver for its storage backend on a node.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageHealthConditionAc {
    /// accessMode is the access mode affected. Nil means all access modes are affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<std::string::String>,
    /// lastTransitionTime is when this condition first appeared at its current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// message is a human-readable description. Maximum permitted length of a message is 1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// reason is a brief CamelCase machine-parseable reason. Maximum permitted length of a reason is 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// status is the health status category. One of "StorageUnreachable", "StorageDegraded".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// volumeMode is the volume mode affected. Nil means both are affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::storage::v1::StorageHealthCondition {
    type Optioned = StorageHealthConditionAc;
}
#[automatically_derived]
impl crate::Optionable for StorageHealthConditionAc {
    type Optioned = StorageHealthConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::storage::v1::StorageHealthCondition {
    fn into_optioned(self) -> StorageHealthConditionAc {
        StorageHealthConditionAc {
            access_mode: self.access_mode,
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            message: self.message,
            reason: Some(self.reason),
            status: Some(self.status),
            volume_mode: self.volume_mode,
        }
    }
    fn try_from_optioned(value: StorageHealthConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            access_mode: value.access_mode,
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            message: value.message,
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
            volume_mode: value.volume_mode,
        })
    }
    fn merge(&mut self, other: StorageHealthConditionAc) -> Result<(), crate::Error> {
        if self.access_mode.is_none() {
            self.access_mode = crate::OptionableConvert::try_from_optioned(
                other.access_mode,
            )?;
        } else if let Some(self_value) = self.access_mode.as_mut()
            && let Some(other_value) = other.access_mode
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
        if let Some(other_value) = other.reason {
            self.reason = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.status {
            self.status = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.volume_mode.is_none() {
            self.volume_mode = crate::OptionableConvert::try_from_optioned(
                other.volume_mode,
            )?;
        } else if let Some(self_value) = self.volume_mode.as_mut()
            && let Some(other_value) = other.volume_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::storage::v1::StorageHealthCondition>
for StorageHealthConditionAc {
    fn from_optionable(
        value: k8s_openapi029::api::storage::v1::StorageHealthCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::storage::v1::StorageHealthCondition, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::storage::v1::StorageHealthCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for StorageHealthConditionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.access_mode, other.access_mode);
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.last_transition_time,
            other.last_transition_time,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.message, other.message);
        k8s_openapi029::DeepMerge::merge_from(&mut self.reason, other.reason);
        k8s_openapi029::DeepMerge::merge_from(&mut self.status, other.status);
        k8s_openapi029::DeepMerge::merge_from(&mut self.volume_mode, other.volume_mode);
    }
}
