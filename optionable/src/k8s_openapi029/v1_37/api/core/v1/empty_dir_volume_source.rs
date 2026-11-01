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
    /// mode specifies the permission bits for the emptyDir directory, in numeric notation (e.g., 0755, 01777). Must be a value between 0000 and 01777. If not specified, defaults to 0777. This might be in conflict with other options that affect the file mode, like fsGroup. If fsGroup is specified, the fsGroup permissions will override the mode specified here. This field has no effect on Windows. This field is alpha and requires EmptyDirVolumeMode featuregate to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// sizeLimit is the total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::core::v1::EmptyDirVolumeSource {
    type Optioned = EmptyDirVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EmptyDirVolumeSourceAc {
    type Optioned = EmptyDirVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::core::v1::EmptyDirVolumeSource {
    fn into_optioned(self) -> EmptyDirVolumeSourceAc {
        EmptyDirVolumeSourceAc {
            medium: self.medium,
            mode: self.mode,
            size_limit: crate::OptionableConvert::into_optioned(self.size_limit),
        }
    }
    fn try_from_optioned(value: EmptyDirVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            medium: value.medium,
            mode: value.mode,
            size_limit: crate::OptionableConvert::try_from_optioned(value.size_limit)?,
        })
    }
    fn merge(&mut self, other: EmptyDirVolumeSourceAc) -> Result<(), crate::Error> {
        if self.medium.is_none() {
            self.medium = crate::OptionableConvert::try_from_optioned(other.medium)?;
        } else if let Some(self_value) = self.medium.as_mut()
            && let Some(other_value) = other.medium
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.mode.is_none() {
            self.mode = crate::OptionableConvert::try_from_optioned(other.mode)?;
        } else if let Some(self_value) = self.mode.as_mut()
            && let Some(other_value) = other.mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.size_limit.is_none() {
            self.size_limit = crate::OptionableConvert::try_from_optioned(
                other.size_limit,
            )?;
        } else if let Some(self_value) = self.size_limit.as_mut()
            && let Some(other_value) = other.size_limit
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::core::v1::EmptyDirVolumeSource>
for EmptyDirVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi029::api::core::v1::EmptyDirVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::core::v1::EmptyDirVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::EmptyDirVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for EmptyDirVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.medium, other.medium);
        k8s_openapi029::DeepMerge::merge_from(&mut self.mode, other.mode);
        k8s_openapi029::DeepMerge::merge_from(&mut self.size_limit, other.size_limit);
    }
}
