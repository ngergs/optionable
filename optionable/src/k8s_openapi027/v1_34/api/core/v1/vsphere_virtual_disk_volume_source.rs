#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a vSphere volume resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VsphereVirtualDiskVolumeSourceAc {
    /// fsType is filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// storagePolicyID is the storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
    #[serde(rename = "storagePolicyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_policy_id: Option<std::string::String>,
    /// storagePolicyName is the storage Policy Based Management (SPBM) profile name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_policy_name: Option<std::string::String>,
    /// volumePath is the path that identifies vSphere volume vmdk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_path: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::VsphereVirtualDiskVolumeSource {
    type Optioned = VsphereVirtualDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for VsphereVirtualDiskVolumeSourceAc {
    type Optioned = VsphereVirtualDiskVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::VsphereVirtualDiskVolumeSource {
    fn into_optioned(self) -> VsphereVirtualDiskVolumeSourceAc {
        VsphereVirtualDiskVolumeSourceAc {
            fs_type: self.fs_type,
            storage_policy_id: self.storage_policy_id,
            storage_policy_name: self.storage_policy_name,
            volume_path: Some(self.volume_path),
        }
    }
    fn try_from_optioned(
        value: VsphereVirtualDiskVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            storage_policy_id: value.storage_policy_id,
            storage_policy_name: value.storage_policy_name,
            volume_path: value
                .volume_path
                .ok_or(crate::Error {
                    missing_field: "volume_path",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: VsphereVirtualDiskVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if self.fs_type.is_none() {
            self.fs_type = other.fs_type;
        }
        if let Some(other_value) = other.fs_type {
            crate::OptionableConvert::merge(&mut self.fs_type, other_value)?;
        }
        if self.storage_policy_id.is_none() {
            self.storage_policy_id = other.storage_policy_id;
        }
        if let Some(other_value) = other.storage_policy_id {
            crate::OptionableConvert::merge(&mut self.storage_policy_id, other_value)?;
        }
        if self.storage_policy_name.is_none() {
            self.storage_policy_name = other.storage_policy_name;
        }
        if let Some(other_value) = other.storage_policy_name {
            crate::OptionableConvert::merge(&mut self.storage_policy_name, other_value)?;
        }
        if let Some(other_value) = other.volume_path {
            self.volume_path = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::VsphereVirtualDiskVolumeSource,
> for VsphereVirtualDiskVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::VsphereVirtualDiskVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::VsphereVirtualDiskVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::VsphereVirtualDiskVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
