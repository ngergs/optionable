#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_error: <Option<
        ::k8s_openapi::api::storage::v1::VolumeError,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_metadata: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_error: <Option<
        ::k8s_openapi::api::storage::v1::VolumeError,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeAttachmentStatus {
    type Optioned = VolumeAttachmentStatusAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentStatusAc {
    type Optioned = VolumeAttachmentStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1::VolumeAttachmentStatus {
    fn into_optioned(self) -> VolumeAttachmentStatusAc {
        VolumeAttachmentStatusAc {
            attach_error: crate::OptionableConvert::into_optioned(self.attach_error),
            attached: Some(self.attached),
            attachment_metadata: crate::OptionableConvert::into_optioned(
                self.attachment_metadata,
            ),
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
            attachment_metadata: crate::OptionableConvert::try_from_optioned(
                value.attachment_metadata,
            )?,
            detach_error: crate::OptionableConvert::try_from_optioned(
                value.detach_error,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeAttachmentStatusAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::storage::v1::VolumeAttachmentStatus>
for VolumeAttachmentStatusAc {
    fn from_optionable(
        value: ::k8s_openapi::api::storage::v1::VolumeAttachmentStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::storage::v1::VolumeAttachmentStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::storage::v1::VolumeAttachmentStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
