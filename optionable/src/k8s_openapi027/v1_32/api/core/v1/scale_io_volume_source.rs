#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ScaleIOVolumeSource represents a persistent ScaleIO volume
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScaleIOVolumeSourceAc {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// gateway is the host address of the ScaleIO API Gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<std::string::String>,
    /// protectionDomain is the name of the ScaleIO Protection Domain for the configured storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_domain: Option<std::string::String>,
    /// readOnly Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// sslEnabled Flag enable/disable SSL communication with Gateway, default false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_enabled: Option<bool>,
    /// storageMode indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: Option<std::string::String>,
    /// storagePool is the ScaleIO Storage Pool associated with the protection domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_pool: Option<std::string::String>,
    /// system is the name of the storage system as configured in ScaleIO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<std::string::String>,
    /// volumeName is the name of a volume already created in the ScaleIO system that is associated with this volume source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ScaleIOVolumeSource {
    type Optioned = ScaleIOVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ScaleIOVolumeSourceAc {
    type Optioned = ScaleIOVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ScaleIOVolumeSource {
    fn into_optioned(self) -> ScaleIOVolumeSourceAc {
        ScaleIOVolumeSourceAc {
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
    fn try_from_optioned(value: ScaleIOVolumeSourceAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: ScaleIOVolumeSourceAc) -> Result<(), crate::Error> {
        if self.fs_type.is_none() {
            self.fs_type = other.fs_type;
        }
        if let Some(other_value) = other.fs_type {
            crate::OptionableConvert::merge(&mut self.fs_type, other_value)?;
        }
        if let Some(other_value) = other.gateway {
            self.gateway = other_value;
        }
        if self.protection_domain.is_none() {
            self.protection_domain = other.protection_domain;
        }
        if let Some(other_value) = other.protection_domain {
            crate::OptionableConvert::merge(&mut self.protection_domain, other_value)?;
        }
        if self.read_only.is_none() {
            self.read_only = other.read_only;
        }
        if let Some(other_value) = other.read_only {
            crate::OptionableConvert::merge(&mut self.read_only, other_value)?;
        }
        if let Some(other_value) = other.secret_ref {
            self.secret_ref = other_value;
        }
        if self.ssl_enabled.is_none() {
            self.ssl_enabled = other.ssl_enabled;
        }
        if let Some(other_value) = other.ssl_enabled {
            crate::OptionableConvert::merge(&mut self.ssl_enabled, other_value)?;
        }
        if self.storage_mode.is_none() {
            self.storage_mode = other.storage_mode;
        }
        if let Some(other_value) = other.storage_mode {
            crate::OptionableConvert::merge(&mut self.storage_mode, other_value)?;
        }
        if self.storage_pool.is_none() {
            self.storage_pool = other.storage_pool;
        }
        if let Some(other_value) = other.storage_pool {
            crate::OptionableConvert::merge(&mut self.storage_pool, other_value)?;
        }
        if let Some(other_value) = other.system {
            self.system = other_value;
        }
        if self.volume_name.is_none() {
            self.volume_name = other.volume_name;
        }
        if let Some(other_value) = other.volume_name {
            crate::OptionableConvert::merge(&mut self.volume_name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ScaleIOVolumeSource>
for ScaleIOVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ScaleIOVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ScaleIOVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ScaleIOVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
