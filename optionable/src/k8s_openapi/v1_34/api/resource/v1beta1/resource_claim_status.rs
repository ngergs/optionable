pub struct ResourceClaimStatusOpt {
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
    type Optioned = ResourceClaimStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimStatusOpt {
    type Optioned = ResourceClaimStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta1::ResourceClaimStatus {
    fn into_optioned(self) -> ResourceClaimStatusOpt {
        ResourceClaimStatusOpt {
            allocation: <Option<
                ::k8s_openapi::api::resource::v1beta1::AllocationResult,
            > as crate::OptionableConvert>::into_optioned(self.allocation),
            devices: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1beta1::AllocatedDeviceStatus,
                >,
            > as crate::OptionableConvert>::into_optioned(self.devices),
            reserved_for: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1beta1::ResourceClaimConsumerReference,
                >,
            > as crate::OptionableConvert>::into_optioned(self.reserved_for),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allocation: <Option<
                ::k8s_openapi::api::resource::v1beta1::AllocationResult,
            > as crate::OptionableConvert>::try_from_optioned(value.allocation)?,
            devices: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1beta1::AllocatedDeviceStatus,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.devices)?,
            reserved_for: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::resource::v1beta1::ResourceClaimConsumerReference,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.reserved_for)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::resource::v1beta1::AllocationResult,
        > as crate::OptionableConvert>::merge(&mut self.allocation, other.allocation)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1beta1::AllocatedDeviceStatus>,
        > as crate::OptionableConvert>::merge(&mut self.devices, other.devices)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::resource::v1beta1::ResourceClaimConsumerReference,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.reserved_for,
            other.reserved_for,
        )?;
        Ok(())
    }
}
