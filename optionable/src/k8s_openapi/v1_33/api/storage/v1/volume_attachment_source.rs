#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAttachmentSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_volume_spec: <Option<
        ::k8s_openapi::api::core::v1::PersistentVolumeSpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeAttachmentSource {
    type Optioned = VolumeAttachmentSourceAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentSourceAc {
    type Optioned = VolumeAttachmentSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1::VolumeAttachmentSource {
    fn into_optioned(self) -> VolumeAttachmentSourceAc {
        VolumeAttachmentSourceAc {
            inline_volume_spec: crate::OptionableConvert::into_optioned(
                self.inline_volume_spec,
            ),
            persistent_volume_name: crate::OptionableConvert::into_optioned(
                self.persistent_volume_name,
            ),
        }
    }
    fn try_from_optioned(value: VolumeAttachmentSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            inline_volume_spec: crate::OptionableConvert::try_from_optioned(
                value.inline_volume_spec,
            )?,
            persistent_volume_name: crate::OptionableConvert::try_from_optioned(
                value.persistent_volume_name,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeAttachmentSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.inline_volume_spec,
            other.inline_volume_spec,
        )?;
        crate::OptionableConvert::merge(
            &mut self.persistent_volume_name,
            other.persistent_volume_name,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::storage::v1::VolumeAttachmentSource>
for VolumeAttachmentSourceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::storage::v1::VolumeAttachmentSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::storage::v1::VolumeAttachmentSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::storage::v1::VolumeAttachmentSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
