#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeErrorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::VolumeError {
    type Optioned = VolumeErrorAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeErrorAc {
    type Optioned = VolumeErrorAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::VolumeError {
    fn into_optioned(self) -> VolumeErrorAc {
        VolumeErrorAc {
            error_code: crate::OptionableConvert::into_optioned(self.error_code),
            message: crate::OptionableConvert::into_optioned(self.message),
            time: crate::OptionableConvert::into_optioned(self.time),
        }
    }
    fn try_from_optioned(
        value: VolumeErrorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            error_code: crate::OptionableConvert::try_from_optioned(value.error_code)?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            time: crate::OptionableConvert::try_from_optioned(value.time)?,
        })
    }
    fn merge(&mut self, other: VolumeErrorAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.error_code, other.error_code)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.time, other.time)?;
        Ok(())
    }
}
