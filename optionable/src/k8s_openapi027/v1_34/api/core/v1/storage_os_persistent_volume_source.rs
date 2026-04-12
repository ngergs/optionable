#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a StorageOS persistent volume resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageOSPersistentVolumeSourceAc {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    /// volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<std::string::String>,
    /// volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
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
        if self.fs_type.is_none() {
            self.fs_type = crate::OptionableConvert::try_from_optioned(other.fs_type)?;
        } else if let Some(self_value) = self.fs_type.as_mut()
            && let Some(other_value) = other.fs_type
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
        if self.secret_ref.is_none() {
            self.secret_ref = crate::OptionableConvert::try_from_optioned(
                other.secret_ref,
            )?;
        } else if let Some(self_value) = self.secret_ref.as_mut()
            && let Some(other_value) = other.secret_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
        if self.volume_namespace.is_none() {
            self.volume_namespace = crate::OptionableConvert::try_from_optioned(
                other.volume_namespace,
            )?;
        } else if let Some(self_value) = self.volume_namespace.as_mut()
            && let Some(other_value) = other.volume_namespace
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
