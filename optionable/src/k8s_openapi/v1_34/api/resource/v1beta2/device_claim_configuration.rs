pub struct DeviceClaimConfigurationOpt {
    pub opaque: <Option<
        ::k8s_openapi::api::resource::v1beta2::OpaqueDeviceConfiguration,
    > as crate::Optionable>::Optioned,
    pub requests: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta2::DeviceClaimConfiguration {
    type Optioned = DeviceClaimConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceClaimConfigurationOpt {
    type Optioned = DeviceClaimConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::DeviceClaimConfiguration {
    fn into_optioned(self) -> DeviceClaimConfigurationOpt {
        DeviceClaimConfigurationOpt {
            opaque: crate::OptionableConvert::into_optioned(self.opaque),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(
        value: DeviceClaimConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            opaque: crate::OptionableConvert::try_from_optioned(value.opaque)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceClaimConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.opaque, other.opaque)?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
