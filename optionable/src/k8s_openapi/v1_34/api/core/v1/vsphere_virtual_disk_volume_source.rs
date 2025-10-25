pub struct VsphereVirtualDiskVolumeSourceOpt {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub storage_policy_id: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub storage_policy_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub volume_path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource {
    type Optioned = VsphereVirtualDiskVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for VsphereVirtualDiskVolumeSourceOpt {
    type Optioned = VsphereVirtualDiskVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource {
    fn into_optioned(self) -> VsphereVirtualDiskVolumeSourceOpt {
        VsphereVirtualDiskVolumeSourceOpt {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            storage_policy_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.storage_policy_id),
            storage_policy_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.storage_policy_name),
            volume_path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.volume_path,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: VsphereVirtualDiskVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            storage_policy_id: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.storage_policy_id)?,
            storage_policy_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.storage_policy_name,
            )?,
            volume_path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .volume_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "volume_path",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: VsphereVirtualDiskVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.storage_policy_id,
            other.storage_policy_id,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.storage_policy_name,
            other.storage_policy_name,
        )?;
        if let Some(other_value) = other.volume_path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.volume_path,
                other_value,
            )?;
        }
        Ok(())
    }
}
