pub struct ResourceClaimStatusAc {
    pub allocation: <Option<
        ::k8s_openapi::api::resource::v1beta1::AllocationResult,
    > as crate::Optionable>::Optioned,
    pub devices: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::AllocatedDeviceStatus>,
    > as crate::Optionable>::Optioned,
    pub reserved_for: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::resource::v1beta1::ResourceClaimConsumerReference,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::ResourceClaimStatus {
    type Optioned = ResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimStatusAc {
    type Optioned = ResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::ResourceClaimStatus {
    fn into_optioned(self) -> ResourceClaimStatusAc {
        ResourceClaimStatusAc {
            allocation: crate::OptionableConvert::into_optioned(self.allocation),
            devices: crate::OptionableConvert::into_optioned(self.devices),
            reserved_for: crate::OptionableConvert::into_optioned(self.reserved_for),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allocation: crate::OptionableConvert::try_from_optioned(value.allocation)?,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            reserved_for: crate::OptionableConvert::try_from_optioned(
                value.reserved_for,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.allocation, other.allocation)?;
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        crate::OptionableConvert::merge(&mut self.reserved_for, other.reserved_for)?;
        Ok(())
    }
}
