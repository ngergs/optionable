#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ScaleIOPersistentVolumeSource represents a persistent ScaleIO volume
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScaleIOPersistentVolumeSourceAc {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// gateway is the host address of the ScaleIO API Gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<std::string::String>,
    /// protectionDomain is the name of the ScaleIO Protection Domain for the configured storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_domain: Option<std::string::String>,
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    /// sslEnabled is the flag to enable/disable SSL communication with Gateway, default false
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
        if self.fs_type.is_none() {
            self.fs_type = crate::OptionableConvert::try_from_optioned(other.fs_type)?;
        } else if let Some(self_value) = self.fs_type.as_mut()
            && let Some(other_value) = other.fs_type
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.gateway {
            self.gateway = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.protection_domain.is_none() {
            self.protection_domain = crate::OptionableConvert::try_from_optioned(
                other.protection_domain,
            )?;
        } else if let Some(self_value) = self.protection_domain.as_mut()
            && let Some(other_value) = other.protection_domain
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.read_only.is_none() {
            self.read_only = crate::OptionableConvert::try_from_optioned(
                other.read_only,
            )?;
        } else if let Some(self_value) = self.read_only.as_mut()
            && let Some(other_value) = other.read_only
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.secret_ref {
            crate::OptionableConvert::merge(&mut self.secret_ref, other_value)?;
        }
        if self.ssl_enabled.is_none() {
            self.ssl_enabled = crate::OptionableConvert::try_from_optioned(
                other.ssl_enabled,
            )?;
        } else if let Some(self_value) = self.ssl_enabled.as_mut()
            && let Some(other_value) = other.ssl_enabled
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.storage_mode.is_none() {
            self.storage_mode = crate::OptionableConvert::try_from_optioned(
                other.storage_mode,
            )?;
        } else if let Some(self_value) = self.storage_mode.as_mut()
            && let Some(other_value) = other.storage_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.storage_pool.is_none() {
            self.storage_pool = crate::OptionableConvert::try_from_optioned(
                other.storage_pool,
            )?;
        } else if let Some(self_value) = self.storage_pool.as_mut()
            && let Some(other_value) = other.storage_pool
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.system {
            self.system = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.volume_name.is_none() {
            self.volume_name = crate::OptionableConvert::try_from_optioned(
                other.volume_name,
            )?;
        } else if let Some(self_value) = self.volume_name.as_mut()
            && let Some(other_value) = other.volume_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for ScaleIOPersistentVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        k8s_openapi027::DeepMerge::merge_from(&mut self.gateway, other.gateway);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.protection_domain,
            other.protection_domain,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        k8s_openapi027::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
        k8s_openapi027::DeepMerge::merge_from(&mut self.ssl_enabled, other.ssl_enabled);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.storage_mode,
            other.storage_mode,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.storage_pool,
            other.storage_pool,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.system, other.system);
        k8s_openapi027::DeepMerge::merge_from(&mut self.volume_name, other.volume_name);
    }
}
