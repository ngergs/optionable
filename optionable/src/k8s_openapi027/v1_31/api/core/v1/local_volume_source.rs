#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Local represents directly-attached storage with node affinity (Beta feature)
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalVolumeSourceAc {
    /// fsType is the filesystem type to mount. It applies only when the Path is a block device. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default value is to auto-select a filesystem if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// path of the full path to the volume on the node. It can be either a directory or block device (disk, partition, ...).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::LocalVolumeSource {
    type Optioned = LocalVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for LocalVolumeSourceAc {
    type Optioned = LocalVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::LocalVolumeSource {
    fn into_optioned(self) -> LocalVolumeSourceAc {
        LocalVolumeSourceAc {
            fs_type: self.fs_type,
            path: Some(self.path),
        }
    }
    fn try_from_optioned(value: LocalVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
        })
    }
    fn merge(&mut self, other: LocalVolumeSourceAc) -> Result<(), crate::Error> {
        if self.fs_type.is_none() {
            self.fs_type = other.fs_type;
        }
        if let Some(other_value) = other.fs_type {
            crate::OptionableConvert::merge(&mut self.fs_type, other_value)?;
        }
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::LocalVolumeSource>
for LocalVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::LocalVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::LocalVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::LocalVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
