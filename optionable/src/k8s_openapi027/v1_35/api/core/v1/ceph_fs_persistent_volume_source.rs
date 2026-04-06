#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CephFSPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_file: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::CephFSPersistentVolumeSource {
    type Optioned = CephFSPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CephFSPersistentVolumeSourceAc {
    type Optioned = CephFSPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::CephFSPersistentVolumeSource {
    fn into_optioned(self) -> CephFSPersistentVolumeSourceAc {
        CephFSPersistentVolumeSourceAc {
            monitors: Some(self.monitors),
            path: self.path,
            read_only: self.read_only,
            secret_file: self.secret_file,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            user: self.user,
        }
    }
    fn try_from_optioned(
        value: CephFSPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            monitors: value
                .monitors
                .ok_or(crate::Error {
                    missing_field: "monitors",
                })?,
            path: value.path,
            read_only: value.read_only,
            secret_file: value.secret_file,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            user: value.user,
        })
    }
    fn merge(
        &mut self,
        other: CephFSPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.monitors {
            self.monitors = other_value;
        }
        self.path = other.path;
        self.read_only = other.read_only;
        self.secret_file = other.secret_file;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        self.user = other.user;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::CephFSPersistentVolumeSource>
for CephFSPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::CephFSPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::CephFSPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::CephFSPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
