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
pub struct StorageOSVolumeSourceAc {
    /// fsType is the filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// volumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<std::string::String>,
    /// volumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_namespace: Option<std::string::String>,
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
            fs_type: self.fs_type,
            read_only: self.read_only,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            volume_name: self.volume_name,
            volume_namespace: self.volume_namespace,
        }
    }
    fn try_from_optioned(value: StorageOSVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            volume_name: value.volume_name,
            volume_namespace: value.volume_namespace,
        })
    }
    fn merge(&mut self, other: StorageOSVolumeSourceAc) -> Result<(), crate::Error> {
        if other.fs_type.is_some() {
            self.fs_type = other.fs_type;
        }
        if other.read_only.is_some() {
            self.read_only = other.read_only;
        }
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        if other.volume_name.is_some() {
            self.volume_name = other.volume_name;
        }
        if other.volume_namespace.is_some() {
            self.volume_namespace = other.volume_namespace;
        }
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
