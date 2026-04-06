#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VsphereVirtualDiskVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(rename = "storagePolicyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_policy_id: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_policy_name: Option<std::string::String>,
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
        self.fs_type = other.fs_type;
        self.storage_policy_id = other.storage_policy_id;
        self.storage_policy_name = other.storage_policy_name;
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
