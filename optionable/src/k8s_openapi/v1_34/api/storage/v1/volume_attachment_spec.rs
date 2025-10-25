pub struct VolumeAttachmentSpecOpt {
    pub attacher: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub node_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub source: Option<
        <::k8s_openapi::api::storage::v1::VolumeAttachmentSource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeAttachmentSpec {
    type Optioned = VolumeAttachmentSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentSpecOpt {
    type Optioned = VolumeAttachmentSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::VolumeAttachmentSpec {
    fn into_optioned(self) -> VolumeAttachmentSpecOpt {
        VolumeAttachmentSpecOpt {
            attacher: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.attacher,
                ),
            ),
            node_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.node_name,
                ),
            ),
            source: Some(
                <::k8s_openapi::api::storage::v1::VolumeAttachmentSource as crate::OptionableConvert>::into_optioned(
                    self.source,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: VolumeAttachmentSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            attacher: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .attacher
                    .ok_or(crate::optionable::Error {
                        missing_field: "attacher",
                    })?,
            )?,
            node_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .node_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "node_name",
                    })?,
            )?,
            source: <::k8s_openapi::api::storage::v1::VolumeAttachmentSource as crate::OptionableConvert>::try_from_optioned(
                value
                    .source
                    .ok_or(crate::optionable::Error {
                        missing_field: "source",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeAttachmentSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.attacher {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.attacher,
                other_value,
            )?;
        }
        if let Some(other_value) = other.node_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.node_name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.source {
            <::k8s_openapi::api::storage::v1::VolumeAttachmentSource as crate::OptionableConvert>::merge(
                &mut self.source,
                other_value,
            )?;
        }
        Ok(())
    }
}
