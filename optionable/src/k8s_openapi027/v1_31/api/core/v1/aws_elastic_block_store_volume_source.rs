#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Persistent Disk resource in AWS.
///
/// An AWS EBS disk must exist before mounting to a container. The disk must also be in the same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS EBS volumes support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AWSElasticBlockStoreVolumeSourceAc {
    /// fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// partition is the partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    /// readOnly value true will force the readOnly setting in VolumeMounts. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// volumeID is unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(rename = "volumeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::AWSElasticBlockStoreVolumeSource {
    type Optioned = AWSElasticBlockStoreVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for AWSElasticBlockStoreVolumeSourceAc {
    type Optioned = AWSElasticBlockStoreVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::AWSElasticBlockStoreVolumeSource {
    fn into_optioned(self) -> AWSElasticBlockStoreVolumeSourceAc {
        AWSElasticBlockStoreVolumeSourceAc {
            fs_type: self.fs_type,
            partition: self.partition,
            read_only: self.read_only,
            volume_id: Some(self.volume_id),
        }
    }
    fn try_from_optioned(
        value: AWSElasticBlockStoreVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            partition: value.partition,
            read_only: value.read_only,
            volume_id: value
                .volume_id
                .ok_or(crate::Error {
                    missing_field: "volume_id",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: AWSElasticBlockStoreVolumeSourceAc,
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
        if self.read_only.is_none() {
            self.read_only = crate::OptionableConvert::try_from_optioned(
                other.read_only,
            )?;
        } else if let Some(self_value) = self.read_only.as_mut()
            && let Some(other_value) = other.read_only
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.volume_id {
            self.volume_id = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::AWSElasticBlockStoreVolumeSource,
> for AWSElasticBlockStoreVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::AWSElasticBlockStoreVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::AWSElasticBlockStoreVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::AWSElasticBlockStoreVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
