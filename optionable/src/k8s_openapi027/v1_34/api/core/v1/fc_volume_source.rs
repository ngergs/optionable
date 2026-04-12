#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FCVolumeSourceAc {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// lun is Optional: FC target lun number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    /// readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// targetWWNs is Optional: FC target worldwide names (WWNs)
    #[serde(rename = "targetWWNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_wwns: Option<std::vec::Vec<std::string::String>>,
    /// wwids Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wwids: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::FCVolumeSource {
    type Optioned = FCVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for FCVolumeSourceAc {
    type Optioned = FCVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::FCVolumeSource {
    fn into_optioned(self) -> FCVolumeSourceAc {
        FCVolumeSourceAc {
            fs_type: self.fs_type,
            lun: self.lun,
            read_only: self.read_only,
            target_wwns: self.target_wwns,
            wwids: self.wwids,
        }
    }
    fn try_from_optioned(value: FCVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            lun: value.lun,
            read_only: value.read_only,
            target_wwns: value.target_wwns,
            wwids: value.wwids,
        })
    }
    fn merge(&mut self, other: FCVolumeSourceAc) -> Result<(), crate::Error> {
        if self.fs_type.is_none() {
            self.fs_type = crate::OptionableConvert::try_from_optioned(other.fs_type)?;
        } else {
            crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        }
        if self.lun.is_none() {
            self.lun = crate::OptionableConvert::try_from_optioned(other.lun)?;
        } else {
            crate::OptionableConvert::merge(&mut self.lun, other.lun)?;
        }
        if self.read_only.is_none() {
            self.read_only = crate::OptionableConvert::try_from_optioned(
                other.read_only,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        }
        if self.target_wwns.is_none() {
            self.target_wwns = crate::OptionableConvert::try_from_optioned(
                other.target_wwns,
            )?;
        } else {
            self.target_wwns = crate::OptionableConvert::try_from_optioned(
                other.target_wwns,
            )?;
        }
        if self.wwids.is_none() {
            self.wwids = crate::OptionableConvert::try_from_optioned(other.wwids)?;
        } else {
            self.wwids = crate::OptionableConvert::try_from_optioned(other.wwids)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::FCVolumeSource>
for FCVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::FCVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::FCVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::FCVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
