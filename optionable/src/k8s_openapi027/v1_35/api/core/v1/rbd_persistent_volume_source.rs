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
pub struct RBDPersistentVolumeSourceAc {
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
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    /// user is the rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::RBDPersistentVolumeSource {
    type Optioned = RBDPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for RBDPersistentVolumeSourceAc {
    type Optioned = RBDPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::RBDPersistentVolumeSource {
    fn into_optioned(self) -> RBDPersistentVolumeSourceAc {
        RBDPersistentVolumeSourceAc {
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
    fn try_from_optioned(
        value: RBDPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: RBDPersistentVolumeSourceAc) -> Result<(), crate::Error> {
        if self.fs_type.is_none() {
            self.fs_type = crate::OptionableConvert::try_from_optioned(other.fs_type)?;
        } else if let Some(self_value) = self.fs_type.as_mut()
            && let Some(other_value) = other.fs_type
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.image {
            self.image = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.keyring.is_none() {
            self.keyring = crate::OptionableConvert::try_from_optioned(other.keyring)?;
        } else if let Some(self_value) = self.keyring.as_mut()
            && let Some(other_value) = other.keyring
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.monitors {
            self.monitors = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.pool.is_none() {
            self.pool = crate::OptionableConvert::try_from_optioned(other.pool)?;
        } else if let Some(self_value) = self.pool.as_mut()
            && let Some(other_value) = other.pool
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
        if self.secret_ref.is_none() {
            self.secret_ref = crate::OptionableConvert::try_from_optioned(
                other.secret_ref,
            )?;
        } else if let Some(self_value) = self.secret_ref.as_mut()
            && let Some(other_value) = other.secret_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::RBDPersistentVolumeSource>
for RBDPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::RBDPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::RBDPersistentVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::RBDPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
