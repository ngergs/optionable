#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageOSPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_namespace: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::StorageOSPersistentVolumeSource {
    type Optioned = StorageOSPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for StorageOSPersistentVolumeSourceAc {
    type Optioned = StorageOSPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::StorageOSPersistentVolumeSource {
    fn into_optioned(self) -> StorageOSPersistentVolumeSourceAc {
        StorageOSPersistentVolumeSourceAc {
            fs_type: self.fs_type,
            read_only: self.read_only,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            volume_name: self.volume_name,
            volume_namespace: self.volume_namespace,
        }
    }
    fn try_from_optioned(
        value: StorageOSPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            volume_name: value.volume_name,
            volume_namespace: value.volume_namespace,
        })
    }
    fn merge(
        &mut self,
        other: StorageOSPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        self.fs_type = other.fs_type;
        self.read_only = other.read_only;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        self.volume_name = other.volume_name;
        self.volume_namespace = other.volume_namespace;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::StorageOSPersistentVolumeSource,
> for StorageOSPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::StorageOSPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::StorageOSPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::StorageOSPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
