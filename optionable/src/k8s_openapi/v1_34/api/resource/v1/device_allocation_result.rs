pub struct DeviceAllocationResultOpt {
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration>,
    > as crate::Optionable>::Optioned,
    pub results: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceRequestAllocationResult>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceAllocationResult {
    type Optioned = DeviceAllocationResultOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceAllocationResultOpt {
    type Optioned = DeviceAllocationResultOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::DeviceAllocationResult {
    fn into_optioned(self) -> DeviceAllocationResultOpt {
        DeviceAllocationResultOpt {
            config: crate::OptionableConvert::into_optioned(self.config),
            results: crate::OptionableConvert::into_optioned(self.results),
        }
    }
    fn try_from_optioned(
        value: DeviceAllocationResultOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            results: crate::OptionableConvert::try_from_optioned(value.results)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAllocationResultOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(&mut self.results, other.results)?;
        Ok(())
    }
}
