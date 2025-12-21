#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PhotonPersistentDiskVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(rename = "pdID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pd_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::core::v1::PhotonPersistentDiskVolumeSource {
    type Optioned = PhotonPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PhotonPersistentDiskVolumeSourceAc {
    type Optioned = PhotonPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::core::v1::PhotonPersistentDiskVolumeSource {
    fn into_optioned(self) -> PhotonPersistentDiskVolumeSourceAc {
        PhotonPersistentDiskVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            pd_id: Some(crate::OptionableConvert::into_optioned(self.pd_id)),
        }
    }
    fn try_from_optioned(
        value: PhotonPersistentDiskVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            pd_id: crate::OptionableConvert::try_from_optioned(
                value
                    .pd_id
                    .ok_or(crate::Error {
                        missing_field: "pd_id",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PhotonPersistentDiskVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        if let Some(other_value) = other.pd_id {
            crate::OptionableConvert::merge(&mut self.pd_id, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::core::v1::PhotonPersistentDiskVolumeSource,
> for PhotonPersistentDiskVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi026::api::core::v1::PhotonPersistentDiskVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::core::v1::PhotonPersistentDiskVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::PhotonPersistentDiskVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
