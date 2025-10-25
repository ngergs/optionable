pub struct VolumeAttachmentSourceOpt {
    pub inline_volume_spec: <Option<
        ::k8s_openapi::api::core::v1::PersistentVolumeSpec,
    > as crate::Optionable>::Optioned,
    pub persistent_volume_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeAttachmentSource {
    type Optioned = VolumeAttachmentSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttachmentSourceOpt {
    type Optioned = VolumeAttachmentSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1::VolumeAttachmentSource {
    fn into_optioned(self) -> VolumeAttachmentSourceOpt {
        VolumeAttachmentSourceOpt {
            inline_volume_spec: <Option<
                ::k8s_openapi::api::core::v1::PersistentVolumeSpec,
            > as crate::OptionableConvert>::into_optioned(self.inline_volume_spec),
            persistent_volume_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.persistent_volume_name),
        }
    }
    fn try_from_optioned(
        value: VolumeAttachmentSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            inline_volume_spec: <Option<
                ::k8s_openapi::api::core::v1::PersistentVolumeSpec,
            > as crate::OptionableConvert>::try_from_optioned(value.inline_volume_spec)?,
            persistent_volume_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.persistent_volume_name,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeAttachmentSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::PersistentVolumeSpec,
        > as crate::OptionableConvert>::merge(
            &mut self.inline_volume_spec,
            other.inline_volume_spec,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.persistent_volume_name,
            other.persistent_volume_name,
        )?;
        Ok(())
    }
}
