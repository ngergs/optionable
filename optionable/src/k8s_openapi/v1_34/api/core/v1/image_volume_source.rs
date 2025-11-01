pub struct ImageVolumeSourceAc {
    pub pull_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reference: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ImageVolumeSource {
    type Optioned = ImageVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ImageVolumeSourceAc {
    type Optioned = ImageVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ImageVolumeSource {
    fn into_optioned(self) -> ImageVolumeSourceAc {
        ImageVolumeSourceAc {
            pull_policy: crate::OptionableConvert::into_optioned(self.pull_policy),
            reference: crate::OptionableConvert::into_optioned(self.reference),
        }
    }
    fn try_from_optioned(
        value: ImageVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            pull_policy: crate::OptionableConvert::try_from_optioned(value.pull_policy)?,
            reference: crate::OptionableConvert::try_from_optioned(value.reference)?,
        })
    }
    fn merge(
        &mut self,
        other: ImageVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.pull_policy, other.pull_policy)?;
        crate::OptionableConvert::merge(&mut self.reference, other.reference)?;
        Ok(())
    }
}
