#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a source location of a volume to mount, managed by an external CSI driver
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSIVolumeSourceAc {
    /// driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// fsType to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// nodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: Option<
        <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// readOnly specifies a read-only configuration for the volume. Defaults to false (read/write).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// volumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::CSIVolumeSource {
    type Optioned = CSIVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CSIVolumeSourceAc {
    type Optioned = CSIVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::CSIVolumeSource {
    fn into_optioned(self) -> CSIVolumeSourceAc {
        CSIVolumeSourceAc {
            driver: Some(self.driver),
            fs_type: self.fs_type,
            node_publish_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_publish_secret_ref,
            ),
            read_only: self.read_only,
            volume_attributes: self.volume_attributes,
        }
    }
    fn try_from_optioned(value: CSIVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            fs_type: value.fs_type,
            node_publish_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_publish_secret_ref,
            )?,
            read_only: value.read_only,
            volume_attributes: value.volume_attributes,
        })
    }
    fn merge(&mut self, other: CSIVolumeSourceAc) -> Result<(), crate::Error> {
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
        if self.node_publish_secret_ref.is_none() {
            self.node_publish_secret_ref = crate::OptionableConvert::try_from_optioned(
                other.node_publish_secret_ref,
            )?;
        } else if let Some(self_value) = self.node_publish_secret_ref.as_mut()
            && let Some(other_value) = other.node_publish_secret_ref
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
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
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::CSIVolumeSource>
for CSIVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::CSIVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::CSIVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::CSIVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for CSIVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.driver, other.driver);
        k8s_openapi027::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        self.node_publish_secret_ref = other.node_publish_secret_ref;
        k8s_openapi027::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        crate::k8s_openapi::merge::merge_granular_option_wrapped(
            &mut self.volume_attributes,
            other.volume_attributes,
        );
    }
}
