pub struct DeviceClassConfigurationOpt {
    pub opaque: <Option<
        ::k8s_openapi::api::resource::v1beta1::OpaqueDeviceConfiguration,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration {
    type Optioned = DeviceClassConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassConfigurationOpt {
    type Optioned = DeviceClassConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::DeviceClassConfiguration {
    fn into_optioned(self) -> DeviceClassConfigurationOpt {
        DeviceClassConfigurationOpt {
            opaque: crate::OptionableConvert::into_optioned(self.opaque),
        }
    }
    fn try_from_optioned(
        value: DeviceClassConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceClassConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.opaque, other.opaque)?;
        Ok(())
    }
}
