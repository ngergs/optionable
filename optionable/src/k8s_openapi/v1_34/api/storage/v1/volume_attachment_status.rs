pub struct VolumeAttachmentStatusOpt {
    pub attach_error: <Option<
        ::k8s_openapi::api::storage::v1::VolumeError,
    > as crate::Optionable>::Optioned,
    pub attached: Option<bool>,
    pub attachment_metadata: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub detach_error: <Option<
        ::k8s_openapi::api::storage::v1::VolumeError,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeAttachmentStatus {
    type Optioned = VolumeAttachmentStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentStatusOpt {
    type Optioned = VolumeAttachmentStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1::VolumeAttachmentStatus {
    fn into_optioned(self) -> VolumeAttachmentStatusOpt {
        VolumeAttachmentStatusOpt {
            attach_error: crate::OptionableConvert::into_optioned(self.attach_error),
            attached: Some(self.attached),
            attachment_metadata: crate::OptionableConvert::into_optioned(
                self.attachment_metadata,
            ),
            detach_error: crate::OptionableConvert::into_optioned(self.detach_error),
        }
    }
    fn try_from_optioned(
        value: VolumeAttachmentStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            attach_error: crate::OptionableConvert::try_from_optioned(
                value.attach_error,
            )?,
            attached: value
                .attached
                .ok_or(crate::optionable::Error {
                    missing_field: "attached",
                })?,
            attachment_metadata: crate::OptionableConvert::try_from_optioned(
                value.attachment_metadata,
            )?,
            detach_error: crate::OptionableConvert::try_from_optioned(
                value.detach_error,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeAttachmentStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.attach_error, other.attach_error)?;
        if let Some(other_value) = other.attached {
            self.attached = other_value;
        }
        crate::OptionableConvert::merge(
            &mut self.attachment_metadata,
            other.attachment_metadata,
        )?;
        crate::OptionableConvert::merge(&mut self.detach_error, other.detach_error)?;
        Ok(())
    }
}
