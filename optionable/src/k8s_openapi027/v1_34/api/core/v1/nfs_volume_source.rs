#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents an NFS mount that lasts the lifetime of a pod. NFS volumes do not support ownership management or SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NFSVolumeSourceAc {
    /// path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// readOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NFSVolumeSource {
    type Optioned = NFSVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for NFSVolumeSourceAc {
    type Optioned = NFSVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NFSVolumeSource {
    fn into_optioned(self) -> NFSVolumeSourceAc {
        NFSVolumeSourceAc {
            path: Some(self.path),
            read_only: self.read_only,
            server: Some(self.server),
        }
    }
    fn try_from_optioned(value: NFSVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            read_only: value.read_only,
            server: value
                .server
                .ok_or(crate::Error {
                    missing_field: "server",
                })?,
        })
    }
    fn merge(&mut self, other: NFSVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        if self.read_only.is_none() {
            self.read_only = other.read_only;
        }
        if let Some(other_value) = other.read_only {
            crate::OptionableConvert::merge(&mut self.read_only, other_value)?;
        }
        if let Some(other_value) = other.server {
            self.server = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NFSVolumeSource>
for NFSVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NFSVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NFSVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NFSVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
