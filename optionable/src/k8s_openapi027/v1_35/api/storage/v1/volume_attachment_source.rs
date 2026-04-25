#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeAttachmentSource represents a volume that should be attached. Right now only PersistentVolumes can be attached via external attacher, in the future we may allow also inline volumes in pods. Exactly one member can be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeAttachmentSourceAc {
    /// inlineVolumeSpec contains all the information necessary to attach a persistent volume defined by a pod's inline VolumeSource. This field is populated only for the CSIMigration feature. It contains translated fields from a pod's inline VolumeSource to a PersistentVolumeSpec. This field is beta-level and is only honored by servers that enabled the CSIMigration feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_volume_spec: Option<
        <::k8s_openapi027::api::core::v1::PersistentVolumeSpec as crate::Optionable>::Optioned,
    >,
    /// persistentVolumeName represents the name of the persistent volume to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::VolumeAttachmentSource {
    type Optioned = VolumeAttachmentSourceAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentSourceAc {
    type Optioned = VolumeAttachmentSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::storage::v1::VolumeAttachmentSource {
    fn into_optioned(self) -> VolumeAttachmentSourceAc {
        VolumeAttachmentSourceAc {
            inline_volume_spec: crate::OptionableConvert::into_optioned(
                self.inline_volume_spec,
            ),
            persistent_volume_name: self.persistent_volume_name,
        }
    }
    fn try_from_optioned(value: VolumeAttachmentSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            inline_volume_spec: crate::OptionableConvert::try_from_optioned(
                value.inline_volume_spec,
            )?,
            persistent_volume_name: value.persistent_volume_name,
        })
    }
    fn merge(&mut self, other: VolumeAttachmentSourceAc) -> Result<(), crate::Error> {
        if self.inline_volume_spec.is_none() {
            self.inline_volume_spec = crate::OptionableConvert::try_from_optioned(
                other.inline_volume_spec,
            )?;
        } else if let Some(self_value) = self.inline_volume_spec.as_mut()
            && let Some(other_value) = other.inline_volume_spec
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.persistent_volume_name.is_none() {
            self.persistent_volume_name = crate::OptionableConvert::try_from_optioned(
                other.persistent_volume_name,
            )?;
        } else if let Some(self_value) = self.persistent_volume_name.as_mut()
            && let Some(other_value) = other.persistent_volume_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::VolumeAttachmentSource>
for VolumeAttachmentSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::storage::v1::VolumeAttachmentSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::VolumeAttachmentSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::VolumeAttachmentSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for VolumeAttachmentSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.inline_volume_spec,
            other.inline_volume_spec,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.persistent_volume_name,
            other.persistent_volume_name,
        );
    }
}
