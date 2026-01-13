#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CinderVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: <Option<
        ::k8s_openapi027::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "volumeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::CinderVolumeSource {
    type Optioned = CinderVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CinderVolumeSourceAc {
    type Optioned = CinderVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::CinderVolumeSource {
    fn into_optioned(self) -> CinderVolumeSourceAc {
        CinderVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            volume_id: Some(crate::OptionableConvert::into_optioned(self.volume_id)),
        }
    }
    fn try_from_optioned(value: CinderVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            volume_id: crate::OptionableConvert::try_from_optioned(
                value
                    .volume_id
                    .ok_or(crate::Error {
                        missing_field: "volume_id",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CinderVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        if let Some(other_value) = other.volume_id {
            crate::OptionableConvert::merge(&mut self.volume_id, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::CinderVolumeSource>
for CinderVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::CinderVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::CinderVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::CinderVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
