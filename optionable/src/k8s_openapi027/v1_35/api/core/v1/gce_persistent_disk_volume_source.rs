#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Persistent Disk resource in Google Compute Engine.
///
/// A GCE PD must exist before mounting to a container. The disk must also be in the same GCE project and zone as the kubelet. A GCE PD can only be mounted as read/write once or read-only many times. GCE PDs support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GCEPersistentDiskVolumeSourceAc {
    /// fsType is filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// partition is the partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    /// pdName is unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pd_name: Option<std::string::String>,
    /// readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::GCEPersistentDiskVolumeSource {
    type Optioned = GCEPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GCEPersistentDiskVolumeSourceAc {
    type Optioned = GCEPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::GCEPersistentDiskVolumeSource {
    fn into_optioned(self) -> GCEPersistentDiskVolumeSourceAc {
        GCEPersistentDiskVolumeSourceAc {
            fs_type: self.fs_type,
            partition: self.partition,
            pd_name: Some(self.pd_name),
            read_only: self.read_only,
        }
    }
    fn try_from_optioned(
        value: GCEPersistentDiskVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            partition: value.partition,
            pd_name: value
                .pd_name
                .ok_or(crate::Error {
                    missing_field: "pd_name",
                })?,
            read_only: value.read_only,
        })
    }
    fn merge(
        &mut self,
        other: GCEPersistentDiskVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if self.fs_type.is_none() {
            self.fs_type = crate::OptionableConvert::try_from_optioned(other.fs_type)?;
        } else if let Some(self_value) = self.fs_type.as_mut()
            && let Some(other_value) = other.fs_type
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.partition.is_none() {
            self.partition = crate::OptionableConvert::try_from_optioned(
                other.partition,
            )?;
        } else if let Some(self_value) = self.partition.as_mut()
            && let Some(other_value) = other.partition
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.pd_name {
            self.pd_name = crate::OptionableConvert::try_from_optioned(other_value)?;
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
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::GCEPersistentDiskVolumeSource>
for GCEPersistentDiskVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::GCEPersistentDiskVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::GCEPersistentDiskVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::GCEPersistentDiskVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for GCEPersistentDiskVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        k8s_openapi027::DeepMerge::merge_from(&mut self.partition, other.partition);
        k8s_openapi027::DeepMerge::merge_from(&mut self.pd_name, other.pd_name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.read_only, other.read_only);
    }
}
