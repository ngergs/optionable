pub struct VolumeDeviceOpt {
    pub device_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeDevice {
    type Optioned = VolumeDeviceOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeDeviceOpt {
    type Optioned = VolumeDeviceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeDevice {
    fn into_optioned(self) -> VolumeDeviceOpt {
        VolumeDeviceOpt {
            device_path: Some(crate::OptionableConvert::into_optioned(self.device_path)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: VolumeDeviceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            device_path: crate::OptionableConvert::try_from_optioned(
                value
                    .device_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "device_path",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeDeviceOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.device_path {
            crate::OptionableConvert::merge(&mut self.device_path, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
