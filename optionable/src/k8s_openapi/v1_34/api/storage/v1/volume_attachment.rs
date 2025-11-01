pub struct VolumeAttachmentOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: Option<
        <::k8s_openapi::api::storage::v1::VolumeAttachmentSpec as crate::Optionable>::Optioned,
    >,
    pub status: <Option<
        ::k8s_openapi::api::storage::v1::VolumeAttachmentStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeAttachment {
    type Optioned = VolumeAttachmentOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentOpt {
    type Optioned = VolumeAttachmentOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::VolumeAttachment {
    fn into_optioned(self) -> VolumeAttachmentOpt {
        VolumeAttachmentOpt {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: VolumeAttachmentOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeAttachmentOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
