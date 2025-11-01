pub struct PhotonPersistentDiskVolumeSourceAc {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub pd_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource {
    type Optioned = PhotonPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PhotonPersistentDiskVolumeSourceAc {
    type Optioned = PhotonPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource {
    fn into_optioned(self) -> PhotonPersistentDiskVolumeSourceAc {
        PhotonPersistentDiskVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            pd_id: Some(crate::OptionableConvert::into_optioned(self.pd_id)),
        }
    }
    fn try_from_optioned(
        value: PhotonPersistentDiskVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            pd_id: crate::OptionableConvert::try_from_optioned(
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
        other: PhotonPersistentDiskVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        if let Some(other_value) = other.pd_id {
            crate::OptionableConvert::merge(&mut self.pd_id, other_value)?;
        }
        Ok(())
    }
}
