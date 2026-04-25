#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not support ownership management or SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GlusterfsVolumeSourceAc {
    /// endpoints is the endpoint name that details Glusterfs topology.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::string::String>,
    /// path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// readOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::GlusterfsVolumeSource {
    type Optioned = GlusterfsVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GlusterfsVolumeSourceAc {
    type Optioned = GlusterfsVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::GlusterfsVolumeSource {
    fn into_optioned(self) -> GlusterfsVolumeSourceAc {
        GlusterfsVolumeSourceAc {
            endpoints: Some(self.endpoints),
            path: Some(self.path),
            read_only: self.read_only,
        }
    }
    fn try_from_optioned(value: GlusterfsVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            endpoints: value
                .endpoints
                .ok_or(crate::Error {
                    missing_field: "endpoints",
                })?,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            read_only: value.read_only,
        })
    }
    fn merge(&mut self, other: GlusterfsVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.endpoints {
            self.endpoints = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.path {
            self.path = crate::OptionableConvert::try_from_optioned(other_value)?;
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
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::GlusterfsVolumeSource>
for GlusterfsVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::GlusterfsVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::GlusterfsVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::GlusterfsVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for GlusterfsVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.endpoints, other.endpoints);
        k8s_openapi027::DeepMerge::merge_from(&mut self.path, other.path);
        k8s_openapi027::DeepMerge::merge_from(&mut self.read_only, other.read_only);
    }
}
