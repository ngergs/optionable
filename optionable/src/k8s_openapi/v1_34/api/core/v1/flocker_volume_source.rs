pub struct FlockerVolumeSourceOpt {
    pub dataset_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub dataset_uuid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::FlockerVolumeSource {
    type Optioned = FlockerVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for FlockerVolumeSourceOpt {
    type Optioned = FlockerVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::FlockerVolumeSource {
    fn into_optioned(self) -> FlockerVolumeSourceOpt {
        FlockerVolumeSourceOpt {
            dataset_name: crate::OptionableConvert::into_optioned(self.dataset_name),
            dataset_uuid: crate::OptionableConvert::into_optioned(self.dataset_uuid),
        }
    }
    fn try_from_optioned(
        value: FlockerVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            dataset_name: crate::OptionableConvert::try_from_optioned(
                value.dataset_name,
            )?,
            dataset_uuid: crate::OptionableConvert::try_from_optioned(
                value.dataset_uuid,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: FlockerVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.dataset_name, other.dataset_name)?;
        crate::OptionableConvert::merge(&mut self.dataset_uuid, other.dataset_uuid)?;
        Ok(())
    }
}
