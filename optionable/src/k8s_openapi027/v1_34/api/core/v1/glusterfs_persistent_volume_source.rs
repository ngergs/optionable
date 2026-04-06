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
pub struct GlusterfsPersistentVolumeSourceAc {
    /// endpoints is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::string::String>,
    /// endpointsNamespace is the namespace that contains Glusterfs endpoint. If this field is empty, the EndpointNamespace defaults to the same namespace as the bound PVC. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints_namespace: Option<std::string::String>,
    /// path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// readOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource {
    type Optioned = GlusterfsPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GlusterfsPersistentVolumeSourceAc {
    type Optioned = GlusterfsPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource {
    fn into_optioned(self) -> GlusterfsPersistentVolumeSourceAc {
        GlusterfsPersistentVolumeSourceAc {
            endpoints: Some(self.endpoints),
            endpoints_namespace: self.endpoints_namespace,
            path: Some(self.path),
            read_only: self.read_only,
        }
    }
    fn try_from_optioned(
        value: GlusterfsPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            endpoints: value
                .endpoints
                .ok_or(crate::Error {
                    missing_field: "endpoints",
                })?,
            endpoints_namespace: value.endpoints_namespace,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            read_only: value.read_only,
        })
    }
    fn merge(
        &mut self,
        other: GlusterfsPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.endpoints {
            self.endpoints = other_value;
        }
        self.endpoints_namespace = other.endpoints_namespace;
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        self.read_only = other.read_only;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource,
> for GlusterfsPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
