pub struct ImageVolumeSourceOpt {
    pub pull_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reference: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::image_volume_source::ImageVolumeSource {
    type Optioned = ImageVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for ImageVolumeSourceOpt {
    type Optioned = ImageVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::image_volume_source::ImageVolumeSource {
    fn into_optioned(self) -> ImageVolumeSourceOpt {
        ImageVolumeSourceOpt {
            pull_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.pull_policy),
            reference: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reference),
        }
    }
    fn try_from_optioned(
        value: ImageVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            pull_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.pull_policy)?,
            reference: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reference)?,
        })
    }
    fn merge(
        &mut self,
        other: ImageVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.pull_policy, other.pull_policy)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reference, other.reference)?;
        Ok(())
    }
}
