pub struct AllocationResultOpt {
    pub allocation_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub devices: <Option<
        ::k8s_openapi::api::resource::v1beta2::DeviceAllocationResult,
    > as crate::Optionable>::Optioned,
    pub node_selector: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::AllocationResult {
    type Optioned = AllocationResultOpt;
}
#[automatically_derived]
impl crate::Optionable for AllocationResultOpt {
    type Optioned = AllocationResultOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::AllocationResult {
    fn into_optioned(self) -> AllocationResultOpt {
        AllocationResultOpt {
            allocation_timestamp: crate::OptionableConvert::into_optioned(
                self.allocation_timestamp,
            ),
            devices: crate::OptionableConvert::into_optioned(self.devices),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
        }
    }
    fn try_from_optioned(
        value: AllocationResultOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allocation_timestamp: crate::OptionableConvert::try_from_optioned(
                value.allocation_timestamp,
            )?,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: AllocationResultOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.allocation_timestamp,
            other.allocation_timestamp,
        )?;
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        Ok(())
    }
}
