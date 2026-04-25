#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// AttachedVolume describes a volume attached to a node
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AttachedVolumeAc {
    /// DevicePath represents the device path where the volume should be available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_path: Option<std::string::String>,
    /// Name of the attached volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::AttachedVolume {
    type Optioned = AttachedVolumeAc;
}
#[automatically_derived]
impl crate::Optionable for AttachedVolumeAc {
    type Optioned = AttachedVolumeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::AttachedVolume {
    fn into_optioned(self) -> AttachedVolumeAc {
        AttachedVolumeAc {
            device_path: Some(self.device_path),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: AttachedVolumeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            device_path: value
                .device_path
                .ok_or(crate::Error {
                    missing_field: "device_path",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(&mut self, other: AttachedVolumeAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.device_path {
            self.device_path = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::AttachedVolume>
for AttachedVolumeAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::AttachedVolume) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::AttachedVolume, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::AttachedVolume,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for AttachedVolumeAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.device_path, other.device_path);
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
