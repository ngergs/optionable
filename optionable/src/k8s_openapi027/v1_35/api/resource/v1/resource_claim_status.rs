#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceClaimStatus tracks whether the resource has been allocated and what the result of that was.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimStatusAc {
    /// Allocation is set once the claim has been allocated successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<
        <::k8s_openapi027::api::resource::v1::AllocationResult as crate::Optionable>::Optioned,
    >,
    /// Devices contains the status of each device allocated for this claim, as reported by the driver. This can include driver-specific information. Entries are owned by their respective drivers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1::AllocatedDeviceStatus as crate::Optionable>::Optioned,
        >,
    >,
    /// ReservedFor indicates which entities are currently allowed to use the claim. A Pod which references a ResourceClaim which is not reserved for that Pod will not be started. A claim that is in use or might be in use because it has been reserved must not get deallocated.
    ///
    /// In a cluster with multiple scheduler instances, two pods might get scheduled concurrently by different schedulers. When they reference the same ResourceClaim which already has reached its maximum number of consumers, only one pod can be scheduled.
    ///
    /// Both schedulers try to add their pod to the claim.status.reservedFor field, but only the update that reaches the API server first gets stored. The other one fails with an error and the scheduler which issued it knows that it must put the pod back into the queue, waiting for the ResourceClaim to become usable again.
    ///
    /// There can be at most 256 such reservations. This may get increased in the future, but not reduced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_for: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1::ResourceClaimConsumerReference as crate::Optionable>::Optioned,
        >,
    >,
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
        if self.allocation.is_none() {
            self.allocation = crate::OptionableConvert::try_from_optioned(
                other.allocation,
            )?;
        } else if let Some(self_value) = self.allocation.as_mut()
            && let Some(other_value) = other.allocation
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.devices.is_none() {
            self.devices = crate::OptionableConvert::try_from_optioned(other.devices)?;
        } else if let Some(self_value) = self.devices.as_mut()
            && let Some(other_value) = other.devices
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.reserved_for.is_none() {
            self.reserved_for = crate::OptionableConvert::try_from_optioned(
                other.reserved_for,
            )?;
        } else if let Some(self_value) = self.reserved_for.as_mut()
            && let Some(other_value) = other.reserved_for
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
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
