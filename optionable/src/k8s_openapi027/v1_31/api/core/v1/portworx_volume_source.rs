#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PortworxVolumeSource represents a Portworx volume resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortworxVolumeSourceAc {
    /// fSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// volumeID uniquely identifies a Portworx volume
    #[serde(rename = "volumeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PortworxVolumeSource {
    type Optioned = PortworxVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PortworxVolumeSourceAc {
    type Optioned = PortworxVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PortworxVolumeSource {
    fn into_optioned(self) -> PortworxVolumeSourceAc {
        PortworxVolumeSourceAc {
            fs_type: self.fs_type,
            read_only: self.read_only,
            volume_id: Some(self.volume_id),
        }
    }
    fn try_from_optioned(value: PortworxVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            read_only: value.read_only,
            volume_id: value
                .volume_id
                .ok_or(crate::Error {
                    missing_field: "volume_id",
                })?,
        })
    }
    fn merge(&mut self, other: PortworxVolumeSourceAc) -> Result<(), crate::Error> {
        if self.fs_type.is_none() {
            self.fs_type = other.fs_type;
        }
        if let Some(other_value) = other.fs_type {
            crate::OptionableConvert::merge(&mut self.fs_type, other_value)?;
        }
        if self.read_only.is_none() {
            self.read_only = other.read_only;
        }
        if let Some(other_value) = other.read_only {
            crate::OptionableConvert::merge(&mut self.read_only, other_value)?;
        }
        if let Some(other_value) = other.volume_id {
            self.volume_id = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PortworxVolumeSource>
for PortworxVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PortworxVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PortworxVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PortworxVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
