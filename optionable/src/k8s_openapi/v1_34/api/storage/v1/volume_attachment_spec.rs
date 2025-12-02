#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacher: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<
        <::k8s_openapi::api::storage::v1::VolumeAttachmentSource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeAttachmentSpec {
    type Optioned = VolumeAttachmentSpecAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentSpecAc {
    type Optioned = VolumeAttachmentSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::VolumeAttachmentSpec {
    fn into_optioned(self) -> VolumeAttachmentSpecAc {
        VolumeAttachmentSpecAc {
            attacher: Some(crate::OptionableConvert::into_optioned(self.attacher)),
            node_name: Some(crate::OptionableConvert::into_optioned(self.node_name)),
            source: Some(crate::OptionableConvert::into_optioned(self.source)),
        }
    }
    fn try_from_optioned(value: VolumeAttachmentSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            attacher: crate::OptionableConvert::try_from_optioned(
                value
                    .attacher
                    .ok_or(crate::Error {
                        missing_field: "attacher",
                    })?,
            )?,
            node_name: crate::OptionableConvert::try_from_optioned(
                value
                    .node_name
                    .ok_or(crate::Error {
                        missing_field: "node_name",
                    })?,
            )?,
            source: crate::OptionableConvert::try_from_optioned(
                value
                    .source
                    .ok_or(crate::Error {
                        missing_field: "source",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeAttachmentSpecAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.attacher {
            crate::OptionableConvert::merge(&mut self.attacher, other_value)?;
        }
        if let Some(other_value) = other.node_name {
            crate::OptionableConvert::merge(&mut self.node_name, other_value)?;
        }
        if let Some(other_value) = other.source {
            crate::OptionableConvert::merge(&mut self.source, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::storage::v1::VolumeAttachmentSpec>
for VolumeAttachmentSpecAc {
    fn from_optionable(
        value: ::k8s_openapi::api::storage::v1::VolumeAttachmentSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::storage::v1::VolumeAttachmentSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::storage::v1::VolumeAttachmentSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
