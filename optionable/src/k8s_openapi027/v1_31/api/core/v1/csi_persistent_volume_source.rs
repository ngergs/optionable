#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents storage that is managed by an external CSI volume driver (Beta feature)
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSIPersistentVolumeSourceAc {
    /// controllerExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerExpandVolume call. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_expand_secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    /// controllerPublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI ControllerPublishVolume and ControllerUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_publish_secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    /// driver is the name of the driver to use for this volume. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// fsType to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// nodeExpandSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeExpandVolume call. This field is optional, may be omitted if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_expand_secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    /// nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    /// nodeStageSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodeStageVolume and NodeStageVolume and NodeUnstageVolume calls. This field is optional, and may be empty if no secret is required. If the secret object contains more than one secret, all secrets are passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_stage_secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    /// readOnly value to pass to ControllerPublishVolumeRequest. Defaults to false (read/write).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// volumeAttributes of the volume to publish.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// volumeHandle is the unique volume name returned by the CSI volume plugin’s CreateVolume to refer to the volume on all subsequent calls. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_handle: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::CSIPersistentVolumeSource {
    type Optioned = CSIPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CSIPersistentVolumeSourceAc {
    type Optioned = CSIPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::CSIPersistentVolumeSource {
    fn into_optioned(self) -> CSIPersistentVolumeSourceAc {
        CSIPersistentVolumeSourceAc {
            controller_expand_secret_ref: crate::OptionableConvert::into_optioned(
                self.controller_expand_secret_ref,
            ),
            controller_publish_secret_ref: crate::OptionableConvert::into_optioned(
                self.controller_publish_secret_ref,
            ),
            driver: Some(self.driver),
            fs_type: self.fs_type,
            node_expand_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_expand_secret_ref,
            ),
            node_publish_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_publish_secret_ref,
            ),
            node_stage_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_stage_secret_ref,
            ),
            read_only: self.read_only,
            volume_attributes: self.volume_attributes,
            volume_handle: Some(self.volume_handle),
        }
    }
    fn try_from_optioned(
        value: CSIPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            controller_expand_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.controller_expand_secret_ref,
            )?,
            controller_publish_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.controller_publish_secret_ref,
            )?,
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            fs_type: value.fs_type,
            node_expand_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_expand_secret_ref,
            )?,
            node_publish_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_publish_secret_ref,
            )?,
            node_stage_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_stage_secret_ref,
            )?,
            read_only: value.read_only,
            volume_attributes: value.volume_attributes,
            volume_handle: value
                .volume_handle
                .ok_or(crate::Error {
                    missing_field: "volume_handle",
                })?,
        })
    }
    fn merge(&mut self, other: CSIPersistentVolumeSourceAc) -> Result<(), crate::Error> {
        if self.controller_expand_secret_ref.is_none() {
            self.controller_expand_secret_ref = crate::OptionableConvert::try_from_optioned(
                other.controller_expand_secret_ref,
            )?;
        } else if let Some(self_value) = self.controller_expand_secret_ref.as_mut()
            && let Some(other_value) = other.controller_expand_secret_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.controller_publish_secret_ref.is_none() {
            self.controller_publish_secret_ref = crate::OptionableConvert::try_from_optioned(
                other.controller_publish_secret_ref,
            )?;
        } else if let Some(self_value) = self.controller_publish_secret_ref.as_mut()
            && let Some(other_value) = other.controller_publish_secret_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.driver {
            self.driver = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.fs_type.is_none() {
            self.fs_type = crate::OptionableConvert::try_from_optioned(other.fs_type)?;
        } else if let Some(self_value) = self.fs_type.as_mut()
            && let Some(other_value) = other.fs_type
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.node_expand_secret_ref.is_none() {
            self.node_expand_secret_ref = crate::OptionableConvert::try_from_optioned(
                other.node_expand_secret_ref,
            )?;
        } else if let Some(self_value) = self.node_expand_secret_ref.as_mut()
            && let Some(other_value) = other.node_expand_secret_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.node_publish_secret_ref.is_none() {
            self.node_publish_secret_ref = crate::OptionableConvert::try_from_optioned(
                other.node_publish_secret_ref,
            )?;
        } else if let Some(self_value) = self.node_publish_secret_ref.as_mut()
            && let Some(other_value) = other.node_publish_secret_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.node_stage_secret_ref.is_none() {
            self.node_stage_secret_ref = crate::OptionableConvert::try_from_optioned(
                other.node_stage_secret_ref,
            )?;
        } else if let Some(self_value) = self.node_stage_secret_ref.as_mut()
            && let Some(other_value) = other.node_stage_secret_ref
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
        if self.volume_attributes.is_none() {
            self.volume_attributes = crate::OptionableConvert::try_from_optioned(
                other.volume_attributes,
            )?;
        } else if let Some(self_value) = self.volume_attributes.as_mut()
            && let Some(other_value) = other.volume_attributes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.volume_handle {
            self.volume_handle = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::CSIPersistentVolumeSource>
for CSIPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::CSIPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::CSIPersistentVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::CSIPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
