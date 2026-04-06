#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RBDVolumeSourceAc {
    /// fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// image is the rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<std::string::String>,
    /// keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyring: Option<std::string::String>,
    /// monitors is a collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<std::vec::Vec<std::string::String>>,
    /// pool is the rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<std::string::String>,
    /// readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// user is the rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::RBDVolumeSource {
    type Optioned = RBDVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for RBDVolumeSourceAc {
    type Optioned = RBDVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::RBDVolumeSource {
    fn into_optioned(self) -> RBDVolumeSourceAc {
        RBDVolumeSourceAc {
            fs_type: self.fs_type,
            image: Some(self.image),
            keyring: self.keyring,
            monitors: Some(self.monitors),
            pool: self.pool,
            read_only: self.read_only,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            user: self.user,
        }
    }
    fn try_from_optioned(value: RBDVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            image: value
                .image
                .ok_or(crate::Error {
                    missing_field: "image",
                })?,
            keyring: value.keyring,
            monitors: value
                .monitors
                .ok_or(crate::Error {
                    missing_field: "monitors",
                })?,
            pool: value.pool,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            user: value.user,
        })
    }
    fn merge(&mut self, other: RBDVolumeSourceAc) -> Result<(), crate::Error> {
        self.fs_type = other.fs_type;
        if let Some(other_value) = other.image {
            self.image = other_value;
        }
        self.keyring = other.keyring;
        if let Some(other_value) = other.monitors {
            self.monitors = other_value;
        }
        self.pool = other.pool;
        self.read_only = other.read_only;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        self.user = other.user;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::RBDVolumeSource>
for RBDVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::RBDVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::RBDVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::RBDVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
