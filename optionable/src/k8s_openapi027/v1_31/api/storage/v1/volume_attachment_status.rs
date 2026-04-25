#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeAttachmentStatusAc {
    /// attachError represents the last error encountered during attach operation, if any. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_error: Option<
        <::k8s_openapi027::api::storage::v1::VolumeError as crate::Optionable>::Optioned,
    >,
    /// attached indicates the volume is successfully attached. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached: Option<bool>,
    /// attachmentMetadata is populated with any information returned by the attach operation, upon successful attach, that must be passed into subsequent WaitForAttach or Mount calls. This field must only be set by the entity completing the attach operation, i.e. the external-attacher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_metadata: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// detachError represents the last error encountered during detach operation, if any. This field must only be set by the entity completing the detach operation, i.e. the external-attacher.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_error: Option<
        <::k8s_openapi027::api::storage::v1::VolumeError as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::VolumeAttachmentStatus {
    type Optioned = VolumeAttachmentStatusAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentStatusAc {
    type Optioned = VolumeAttachmentStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::storage::v1::VolumeAttachmentStatus {
    fn into_optioned(self) -> VolumeAttachmentStatusAc {
        VolumeAttachmentStatusAc {
            attach_error: crate::OptionableConvert::into_optioned(self.attach_error),
            attached: Some(self.attached),
            attachment_metadata: self.attachment_metadata,
            detach_error: crate::OptionableConvert::into_optioned(self.detach_error),
        }
    }
    fn try_from_optioned(value: VolumeAttachmentStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            attach_error: crate::OptionableConvert::try_from_optioned(
                value.attach_error,
            )?,
            attached: value
                .attached
                .ok_or(crate::Error {
                    missing_field: "attached",
                })?,
            attachment_metadata: value.attachment_metadata,
            detach_error: crate::OptionableConvert::try_from_optioned(
                value.detach_error,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeAttachmentStatusAc) -> Result<(), crate::Error> {
        if self.attach_error.is_none() {
            self.attach_error = crate::OptionableConvert::try_from_optioned(
                other.attach_error,
            )?;
        } else if let Some(self_value) = self.attach_error.as_mut()
            && let Some(other_value) = other.attach_error
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.attached {
            self.attached = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.attachment_metadata.is_none() {
            self.attachment_metadata = crate::OptionableConvert::try_from_optioned(
                other.attachment_metadata,
            )?;
        } else if let Some(self_value) = self.attachment_metadata.as_mut()
            && let Some(other_value) = other.attachment_metadata
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.detach_error.is_none() {
            self.detach_error = crate::OptionableConvert::try_from_optioned(
                other.detach_error,
            )?;
        } else if let Some(self_value) = self.detach_error.as_mut()
            && let Some(other_value) = other.detach_error
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::VolumeAttachmentStatus>
for VolumeAttachmentStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::storage::v1::VolumeAttachmentStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::VolumeAttachmentStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::VolumeAttachmentStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for VolumeAttachmentStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.attach_error,
            other.attach_error,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.attached, other.attached);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.attachment_metadata,
            other.attachment_metadata,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.detach_error,
            other.detach_error,
        );
    }
}
