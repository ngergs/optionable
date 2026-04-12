#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents an empty directory for a pod. Empty directory volumes support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmptyDirVolumeSourceAc {
    /// medium represents what type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<std::string::String>,
    /// sizeLimit is the total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EmptyDirVolumeSource {
    type Optioned = EmptyDirVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EmptyDirVolumeSourceAc {
    type Optioned = EmptyDirVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EmptyDirVolumeSource {
    fn into_optioned(self) -> EmptyDirVolumeSourceAc {
        EmptyDirVolumeSourceAc {
            medium: self.medium,
            size_limit: crate::OptionableConvert::into_optioned(self.size_limit),
        }
    }
    fn try_from_optioned(value: EmptyDirVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            medium: value.medium,
            size_limit: crate::OptionableConvert::try_from_optioned(value.size_limit)?,
        })
    }
    fn merge(&mut self, other: EmptyDirVolumeSourceAc) -> Result<(), crate::Error> {
        if other.medium.is_some() {
            self.medium = other.medium;
        }
        crate::OptionableConvert::merge(&mut self.size_limit, other.size_limit)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EmptyDirVolumeSource>
for EmptyDirVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::EmptyDirVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EmptyDirVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EmptyDirVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
