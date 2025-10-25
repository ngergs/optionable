pub struct VolumeErrorOpt {
    pub error_code: <Option<i32> as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::volume_error::VolumeError {
    type Optioned = VolumeErrorOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeErrorOpt {
    type Optioned = VolumeErrorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::storage::v1::volume_error::VolumeError {
    fn into_optioned(self) -> VolumeErrorOpt {
        VolumeErrorOpt {
            error_code: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.error_code),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.time),
        }
    }
    fn try_from_optioned(
        value: VolumeErrorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            error_code: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.error_code)?,
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.time)?,
        })
    }
    fn merge(&mut self, other: VolumeErrorOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.error_code, other.error_code)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(&mut self.time, other.time)?;
        Ok(())
    }
}
