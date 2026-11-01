#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeHealthCondition represents an adverse health condition reported for a volume.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeHealthConditionAc {
    /// message is a human-readable description. Maximum permitted length of a message is 1024 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// reason is a brief CamelCase machine-parseable reason. Together with status it forms the unique identity of a condition entry. Maximum permitted length of a reason is 256 bytes.
    pub reason: std::string::String,
    /// status is the machine-parseable health category. Possible values: - "Inaccessible": the volume cannot be accessed. - "DataLoss": data loss has been detected on the volume. - "Degraded": the volume is functioning with reduced capability.
    pub status: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::core::v1::VolumeHealthCondition {
    type Optioned = VolumeHealthConditionAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeHealthConditionAc {
    type Optioned = VolumeHealthConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::core::v1::VolumeHealthCondition {
    fn into_optioned(self) -> VolumeHealthConditionAc {
        VolumeHealthConditionAc {
            message: self.message,
            reason: self.reason,
            status: self.status,
        }
    }
    fn try_from_optioned(value: VolumeHealthConditionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            message: value.message,
            reason: value.reason,
            status: value.status,
        })
    }
    fn merge(&mut self, other: VolumeHealthConditionAc) -> Result<(), crate::Error> {
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else if let Some(self_value) = self.message.as_mut()
            && let Some(other_value) = other.message
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.reason = other.reason;
        self.status = other.status;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::core::v1::VolumeHealthCondition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.reason == other.reason && self.status == other.status
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::core::v1::VolumeHealthCondition>
for VolumeHealthConditionAc {
    fn from_optionable(
        value: k8s_openapi029::api::core::v1::VolumeHealthCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::core::v1::VolumeHealthCondition, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::VolumeHealthCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for VolumeHealthConditionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.message, other.message);
        k8s_openapi029::DeepMerge::merge_from(&mut self.reason, other.reason);
        k8s_openapi029::DeepMerge::merge_from(&mut self.status, other.status);
    }
}
impl crate::merge::MapKeysEq for VolumeHealthConditionAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.reason == other.reason && self.status == other.status
    }
}
