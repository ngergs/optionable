#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: <Option<
        ::k8s_openapi::api::resource::v1alpha2::AllocationResult,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deallocation_requested: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_for: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::resource::v1alpha2::ResourceClaimConsumerReference,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::ResourceClaimStatus {
    type Optioned = ResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimStatusAc {
    type Optioned = ResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::ResourceClaimStatus {
    fn into_optioned(self) -> ResourceClaimStatusAc {
        ResourceClaimStatusAc {
            allocation: crate::OptionableConvert::into_optioned(self.allocation),
            deallocation_requested: crate::OptionableConvert::into_optioned(
                self.deallocation_requested,
            ),
            driver_name: crate::OptionableConvert::into_optioned(self.driver_name),
            reserved_for: crate::OptionableConvert::into_optioned(self.reserved_for),
        }
    }
    fn try_from_optioned(value: ResourceClaimStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocation: crate::OptionableConvert::try_from_optioned(value.allocation)?,
            deallocation_requested: crate::OptionableConvert::try_from_optioned(
                value.deallocation_requested,
            )?,
            driver_name: crate::OptionableConvert::try_from_optioned(value.driver_name)?,
            reserved_for: crate::OptionableConvert::try_from_optioned(
                value.reserved_for,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceClaimStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.allocation, other.allocation)?;
        crate::OptionableConvert::merge(
            &mut self.deallocation_requested,
            other.deallocation_requested,
        )?;
        crate::OptionableConvert::merge(&mut self.driver_name, other.driver_name)?;
        crate::OptionableConvert::merge(&mut self.reserved_for, other.reserved_for)?;
        Ok(())
    }
}
