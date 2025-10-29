pub struct DeviceOpt {
    pub basic: <Option<
        ::k8s_openapi::api::resource::v1beta1::BasicDevice,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::Device {
    type Optioned = DeviceOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceOpt {
    type Optioned = DeviceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::Device {
    fn into_optioned(self) -> DeviceOpt {
        DeviceOpt {
            basic: crate::OptionableConvert::into_optioned(self.basic),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: DeviceOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            basic: crate::OptionableConvert::try_from_optioned(value.basic)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.basic, other.basic)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
