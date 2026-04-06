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
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(rename = "storagePolicyID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_policy_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_policy_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_path: Option<<std::string::String as crate::Optionable>::Optioned>,
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
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            storage_policy_id: crate::OptionableConvert::into_optioned(
                self.storage_policy_id,
            ),
            storage_policy_name: crate::OptionableConvert::into_optioned(
                self.storage_policy_name,
            ),
            volume_path: Some(crate::OptionableConvert::into_optioned(self.volume_path)),
        }
    }
    fn try_from_optioned(
        value: VsphereVirtualDiskVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            storage_policy_id: crate::OptionableConvert::try_from_optioned(
                value.storage_policy_id,
            )?,
            storage_policy_name: crate::OptionableConvert::try_from_optioned(
                value.storage_policy_name,
            )?,
            volume_path: crate::OptionableConvert::try_from_optioned(
                value
                    .volume_path
                    .ok_or(crate::Error {
                        missing_field: "volume_path",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: VsphereVirtualDiskVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(
            &mut self.storage_policy_id,
            other.storage_policy_id,
        )?;
        crate::OptionableConvert::merge(
            &mut self.storage_policy_name,
            other.storage_policy_name,
        )?;
        if let Some(other_value) = other.volume_path {
            crate::OptionableConvert::merge(&mut self.volume_path, other_value)?;
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
