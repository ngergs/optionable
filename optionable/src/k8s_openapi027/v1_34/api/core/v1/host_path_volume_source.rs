#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a host path mapped into a pod. Host path volumes do not support ownership management or SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostPathVolumeSourceAc {
    /// path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// type for HostPath Volume Defaults to "" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::HostPathVolumeSource {
    type Optioned = HostPathVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for HostPathVolumeSourceAc {
    type Optioned = HostPathVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::HostPathVolumeSource {
    fn into_optioned(self) -> HostPathVolumeSourceAc {
        HostPathVolumeSourceAc {
            path: Some(self.path),
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: HostPathVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: HostPathVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.path {
            self.path = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.type_.is_none() {
            self.type_ = crate::OptionableConvert::try_from_optioned(other.type_)?;
        } else if let Some(self_value) = self.type_.as_mut()
            && let Some(other_value) = other.type_
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::HostPathVolumeSource>
for HostPathVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::HostPathVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::HostPathVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::HostPathVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
