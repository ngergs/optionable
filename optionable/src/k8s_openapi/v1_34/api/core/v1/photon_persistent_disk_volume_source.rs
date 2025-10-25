pub struct PhotonPersistentDiskVolumeSourceOpt {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub pd_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::photon_persistent_disk_volume_source::PhotonPersistentDiskVolumeSource {
    type Optioned = PhotonPersistentDiskVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for PhotonPersistentDiskVolumeSourceOpt {
    type Optioned = PhotonPersistentDiskVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::photon_persistent_disk_volume_source::PhotonPersistentDiskVolumeSource {
    fn into_optioned(self) -> PhotonPersistentDiskVolumeSourceOpt {
        PhotonPersistentDiskVolumeSourceOpt {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            pd_id: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.pd_id,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: PhotonPersistentDiskVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            pd_id: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .pd_id
                    .ok_or(crate::optionable::Error {
                        missing_field: "pd_id",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PhotonPersistentDiskVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        if let Some(other_value) = other.pd_id {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.pd_id,
                other_value,
            )?;
        }
        Ok(())
    }
}
