#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeAttachmentSpecAc {
    /// attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName().
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacher: Option<std::string::String>,
    /// nodeName represents the node that the volume should be attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// source represents the volume that should be attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<
        <::k8s_openapi027::api::storage::v1::VolumeAttachmentSource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::VolumeAttachmentSpec {
    type Optioned = VolumeAttachmentSpecAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentSpecAc {
    type Optioned = VolumeAttachmentSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::storage::v1::VolumeAttachmentSpec {
    fn into_optioned(self) -> VolumeAttachmentSpecAc {
        VolumeAttachmentSpecAc {
            attacher: Some(self.attacher),
            node_name: Some(self.node_name),
            source: Some(crate::OptionableConvert::into_optioned(self.source)),
        }
    }
    fn try_from_optioned(value: VolumeAttachmentSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            attacher: value
                .attacher
                .ok_or(crate::Error {
                    missing_field: "attacher",
                })?,
            node_name: value
                .node_name
                .ok_or(crate::Error {
                    missing_field: "node_name",
                })?,
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
            self.attacher = other_value;
        }
        if let Some(other_value) = other.node_name {
            self.node_name = other_value;
        }
        if let Some(other_value) = other.source {
            self.source = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::VolumeAttachmentSpec>
for VolumeAttachmentSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::storage::v1::VolumeAttachmentSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::VolumeAttachmentSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::VolumeAttachmentSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
