pub struct VolumeErrorOpt {
    pub error_code: <Option<i32> as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeError {
    type Optioned = VolumeErrorOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeErrorOpt {
    type Optioned = VolumeErrorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::VolumeError {
    fn into_optioned(self) -> VolumeErrorOpt {
        VolumeErrorOpt {
            error_code: crate::OptionableConvert::into_optioned(self.error_code),
            message: crate::OptionableConvert::into_optioned(self.message),
            time: crate::OptionableConvert::into_optioned(self.time),
        }
    }
    fn try_from_optioned(
        value: VolumeErrorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            error_code: crate::OptionableConvert::try_from_optioned(value.error_code)?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            time: crate::OptionableConvert::try_from_optioned(value.time)?,
        })
    }
    fn merge(&mut self, other: VolumeErrorOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.error_code, other.error_code)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.time, other.time)?;
        Ok(())
    }
}
