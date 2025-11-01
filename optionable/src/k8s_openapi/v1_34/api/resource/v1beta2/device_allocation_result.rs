pub struct DeviceAllocationResultAc {
    pub config: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::resource::v1beta2::DeviceAllocationConfiguration,
        >,
    > as crate::Optionable>::Optioned,
    pub results: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::resource::v1beta2::DeviceRequestAllocationResult,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta2::DeviceAllocationResult {
    type Optioned = DeviceAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAllocationResultAc {
    type Optioned = DeviceAllocationResultAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::DeviceAllocationResult {
    fn into_optioned(self) -> DeviceAllocationResultAc {
        DeviceAllocationResultAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            results: crate::OptionableConvert::into_optioned(self.results),
        }
    }
    fn try_from_optioned(
        value: DeviceAllocationResultAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            results: crate::OptionableConvert::try_from_optioned(value.results)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAllocationResultAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(&mut self.results, other.results)?;
        Ok(())
    }
}
