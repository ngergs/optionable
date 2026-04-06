#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CinderPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    #[serde(rename = "volumeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::CinderPersistentVolumeSource {
    type Optioned = CinderPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CinderPersistentVolumeSourceAc {
    type Optioned = CinderPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::CinderPersistentVolumeSource {
    fn into_optioned(self) -> CinderPersistentVolumeSourceAc {
        CinderPersistentVolumeSourceAc {
            fs_type: self.fs_type,
            read_only: self.read_only,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            volume_id: Some(self.volume_id),
        }
    }
    fn try_from_optioned(
        value: CinderPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            volume_id: value
                .volume_id
                .ok_or(crate::Error {
                    missing_field: "volume_id",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: CinderPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        self.fs_type = other.fs_type;
        self.read_only = other.read_only;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        if let Some(other_value) = other.volume_id {
            self.volume_id = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::CinderPersistentVolumeSource>
for CinderPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::CinderPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::CinderPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::CinderPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
