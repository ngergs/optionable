#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not support ownership management or SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CephFSVolumeSourceAc {
    /// monitors is Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<std::vec::Vec<std::string::String>>,
    /// path is Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// readOnly is Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretFile is Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_file: Option<std::string::String>,
    /// secretRef is Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// user is optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::CephFSVolumeSource {
    type Optioned = CephFSVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CephFSVolumeSourceAc {
    type Optioned = CephFSVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::CephFSVolumeSource {
    fn into_optioned(self) -> CephFSVolumeSourceAc {
        CephFSVolumeSourceAc {
            monitors: Some(self.monitors),
            path: self.path,
            read_only: self.read_only,
            secret_file: self.secret_file,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            user: self.user,
        }
    }
    fn try_from_optioned(value: CephFSVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            monitors: value
                .monitors
                .ok_or(crate::Error {
                    missing_field: "monitors",
                })?,
            path: value.path,
            read_only: value.read_only,
            secret_file: value.secret_file,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            user: value.user,
        })
    }
    fn merge(&mut self, other: CephFSVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.monitors {
            self.monitors = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.path.is_none() {
            self.path = crate::OptionableConvert::try_from_optioned(other.path)?;
        } else if let Some(self_value) = self.path.as_mut()
            && let Some(other_value) = other.path
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
        if self.secret_file.is_none() {
            self.secret_file = crate::OptionableConvert::try_from_optioned(
                other.secret_file,
            )?;
        } else if let Some(self_value) = self.secret_file.as_mut()
            && let Some(other_value) = other.secret_file
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
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.user.is_none() {
            self.user = crate::OptionableConvert::try_from_optioned(other.user)?;
        } else if let Some(self_value) = self.user.as_mut()
            && let Some(other_value) = other.user
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::CephFSVolumeSource>
for CephFSVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::CephFSVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::CephFSVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::CephFSVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for CephFSVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        self.monitors = other.monitors;
        k8s_openapi027::DeepMerge::merge_from(&mut self.path, other.path);
        k8s_openapi027::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        k8s_openapi027::DeepMerge::merge_from(&mut self.secret_file, other.secret_file);
        self.secret_ref = other.secret_ref;
        k8s_openapi027::DeepMerge::merge_from(&mut self.user, other.user);
    }
}
