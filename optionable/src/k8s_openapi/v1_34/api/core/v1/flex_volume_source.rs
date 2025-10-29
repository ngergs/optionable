pub struct FlexVolumeSourceOpt {
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub options: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::FlexVolumeSource {
    type Optioned = FlexVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for FlexVolumeSourceOpt {
    type Optioned = FlexVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::FlexVolumeSource {
    fn into_optioned(self) -> FlexVolumeSourceOpt {
        FlexVolumeSourceOpt {
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            options: crate::OptionableConvert::into_optioned(self.options),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
        }
    }
    fn try_from_optioned(
        value: FlexVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            driver: crate::OptionableConvert::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            options: crate::OptionableConvert::try_from_optioned(value.options)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
        })
    }
    fn merge(
        &mut self,
        other: FlexVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.driver {
            crate::OptionableConvert::merge(&mut self.driver, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.options, other.options)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        Ok(())
    }
}
