#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScaleIOPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_domain: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_pool: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ScaleIOPersistentVolumeSource {
    type Optioned = ScaleIOPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ScaleIOPersistentVolumeSourceAc {
    type Optioned = ScaleIOPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ScaleIOPersistentVolumeSource {
    fn into_optioned(self) -> ScaleIOPersistentVolumeSourceAc {
        ScaleIOPersistentVolumeSourceAc {
            fs_type: self.fs_type,
            gateway: Some(self.gateway),
            protection_domain: self.protection_domain,
            read_only: self.read_only,
            secret_ref: Some(crate::OptionableConvert::into_optioned(self.secret_ref)),
            ssl_enabled: self.ssl_enabled,
            storage_mode: self.storage_mode,
            storage_pool: self.storage_pool,
            system: Some(self.system),
            volume_name: self.volume_name,
        }
    }
    fn try_from_optioned(
        value: ScaleIOPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            gateway: value
                .gateway
                .ok_or(crate::Error {
                    missing_field: "gateway",
                })?,
            protection_domain: value.protection_domain,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .secret_ref
                    .ok_or(crate::Error {
                        missing_field: "secret_ref",
                    })?,
            )?,
            ssl_enabled: value.ssl_enabled,
            storage_mode: value.storage_mode,
            storage_pool: value.storage_pool,
            system: value
                .system
                .ok_or(crate::Error {
                    missing_field: "system",
                })?,
            volume_name: value.volume_name,
        })
    }
    fn merge(
        &mut self,
        other: ScaleIOPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        self.fs_type = other.fs_type;
        if let Some(other_value) = other.gateway {
            self.gateway = other_value;
        }
        self.protection_domain = other.protection_domain;
        self.read_only = other.read_only;
        if let Some(other_value) = other.secret_ref {
            crate::OptionableConvert::merge(&mut self.secret_ref, other_value)?;
        }
        self.ssl_enabled = other.ssl_enabled;
        self.storage_mode = other.storage_mode;
        self.storage_pool = other.storage_pool;
        if let Some(other_value) = other.system {
            self.system = other_value;
        }
        self.volume_name = other.volume_name;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ScaleIOPersistentVolumeSource>
for ScaleIOPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ScaleIOPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ScaleIOPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ScaleIOPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
