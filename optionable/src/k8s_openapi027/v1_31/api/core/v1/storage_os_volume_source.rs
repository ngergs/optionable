#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageOSVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: <Option<
        ::k8s_openapi027::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::StorageOSVolumeSource {
    type Optioned = StorageOSVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for StorageOSVolumeSourceAc {
    type Optioned = StorageOSVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::StorageOSVolumeSource {
    fn into_optioned(self) -> StorageOSVolumeSourceAc {
        StorageOSVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            volume_name: crate::OptionableConvert::into_optioned(self.volume_name),
            volume_namespace: crate::OptionableConvert::into_optioned(
                self.volume_namespace,
            ),
        }
    }
    fn try_from_optioned(value: StorageOSVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            volume_name: crate::OptionableConvert::try_from_optioned(value.volume_name)?,
            volume_namespace: crate::OptionableConvert::try_from_optioned(
                value.volume_namespace,
            )?,
        })
    }
    fn merge(&mut self, other: StorageOSVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        crate::OptionableConvert::merge(&mut self.volume_name, other.volume_name)?;
        crate::OptionableConvert::merge(
            &mut self.volume_namespace,
            other.volume_namespace,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::StorageOSVolumeSource>
for StorageOSVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::StorageOSVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::StorageOSVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::StorageOSVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
