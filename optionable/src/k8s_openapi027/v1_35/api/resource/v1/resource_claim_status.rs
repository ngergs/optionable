#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: <Option<
        ::k8s_openapi027::api::resource::v1::AllocationResult,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: <Option<
        std::vec::Vec<::k8s_openapi027::api::resource::v1::AllocatedDeviceStatus>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_for: <Option<
        std::vec::Vec<
            ::k8s_openapi027::api::resource::v1::ResourceClaimConsumerReference,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1::ResourceClaimStatus {
    type Optioned = ResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimStatusAc {
    type Optioned = ResourceClaimStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1::ResourceClaimStatus {
    fn into_optioned(self) -> ResourceClaimStatusAc {
        ResourceClaimStatusAc {
            allocation: crate::OptionableConvert::into_optioned(self.allocation),
            devices: crate::OptionableConvert::into_optioned(self.devices),
            reserved_for: crate::OptionableConvert::into_optioned(self.reserved_for),
        }
    }
    fn try_from_optioned(value: ResourceClaimStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocation: crate::OptionableConvert::try_from_optioned(value.allocation)?,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            reserved_for: crate::OptionableConvert::try_from_optioned(
                value.reserved_for,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceClaimStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.allocation, other.allocation)?;
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        crate::OptionableConvert::merge(&mut self.reserved_for, other.reserved_for)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1::ResourceClaimStatus>
for ResourceClaimStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1::ResourceClaimStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1::ResourceClaimStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::ResourceClaimStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
