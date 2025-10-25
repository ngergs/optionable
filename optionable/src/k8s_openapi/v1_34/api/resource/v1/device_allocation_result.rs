pub struct DeviceAllocationResultOpt {
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration>,
    > as crate::Optionable>::Optioned,
    pub results: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceRequestAllocationResult>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1::device_allocation_result::DeviceAllocationResult {
    type Optioned = DeviceAllocationResultOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceAllocationResultOpt {
    type Optioned = DeviceAllocationResultOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::device_allocation_result::DeviceAllocationResult {
    fn into_optioned(self) -> DeviceAllocationResultOpt {
        DeviceAllocationResultOpt {
            config: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration,
                >,
            > as crate::OptionableConvert>::into_optioned(self.config),
            results: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1::DeviceRequestAllocationResult,
                >,
            > as crate::OptionableConvert>::into_optioned(self.results),
        }
    }
    fn try_from_optioned(
        value: DeviceAllocationResultOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.config)?,
            results: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1::DeviceRequestAllocationResult,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.results)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAllocationResultOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::resource::v1::DeviceAllocationConfiguration,
            >,
        > as crate::OptionableConvert>::merge(&mut self.config, other.config)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::resource::v1::DeviceRequestAllocationResult,
            >,
        > as crate::OptionableConvert>::merge(&mut self.results, other.results)?;
        Ok(())
    }
}
