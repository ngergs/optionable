#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Photon Controller persistent disk resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PhotonPersistentDiskVolumeSourceAc {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// pdID is the ID that identifies Photon Controller persistent disk
    #[serde(rename = "pdID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pd_id: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::PhotonPersistentDiskVolumeSource {
    type Optioned = PhotonPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PhotonPersistentDiskVolumeSourceAc {
    type Optioned = PhotonPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PhotonPersistentDiskVolumeSource {
    fn into_optioned(self) -> PhotonPersistentDiskVolumeSourceAc {
        PhotonPersistentDiskVolumeSourceAc {
            fs_type: self.fs_type,
            pd_id: Some(self.pd_id),
        }
    }
    fn try_from_optioned(
        value: PhotonPersistentDiskVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            pd_id: value
                .pd_id
                .ok_or(crate::Error {
                    missing_field: "pd_id",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: PhotonPersistentDiskVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if self.fs_type.is_none() {
            self.fs_type = crate::OptionableConvert::try_from_optioned(other.fs_type)?;
        } else if let Some(self_value) = self.fs_type.as_mut()
            && let Some(other_value) = other.fs_type
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.pd_id {
            self.pd_id = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::PhotonPersistentDiskVolumeSource,
> for PhotonPersistentDiskVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PhotonPersistentDiskVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::PhotonPersistentDiskVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PhotonPersistentDiskVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
