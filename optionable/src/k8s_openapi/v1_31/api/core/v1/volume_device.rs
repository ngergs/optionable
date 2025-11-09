#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeDeviceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeDevice {
    type Optioned = VolumeDeviceAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeDeviceAc {
    type Optioned = VolumeDeviceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeDevice {
    fn into_optioned(self) -> VolumeDeviceAc {
        VolumeDeviceAc {
            device_path: Some(crate::OptionableConvert::into_optioned(self.device_path)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: VolumeDeviceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            device_path: crate::OptionableConvert::try_from_optioned(
                value
                    .device_path
                    .ok_or(crate::Error {
                        missing_field: "device_path",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeDeviceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.device_path {
            crate::OptionableConvert::merge(&mut self.device_path, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
