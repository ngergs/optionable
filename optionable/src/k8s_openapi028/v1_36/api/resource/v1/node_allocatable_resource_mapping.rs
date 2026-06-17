#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAllocatableResourceMapping defines the translation between the DRA device/capacity units requested to the corresponding quantity of the node allocatable resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAllocatableResourceMappingAc {
    /// AllocationMultiplier is used as a multiplier for the allocated device count or the allocated capacity in the claim. It defaults to 1 if not specified. How the field is used also depends on whether `capacityKey` is set. 1.  If `capacityKey` is NOT set: `allocationMultiplier` multiplies the device count allocated to the claim.
    ///        a. A DRA driver representing each CPU core as a device would have
    ///        {ResourceName: "cpu", allocationMultiplier: "2"} in its
    ///        `nodeAllocatableResourceMappings`. If 4 devices are allocated to the claim,
    ///           4 * 2 CPUs would be considered as allocated and subtracted from the node's capacity.
    ///     b. A GPU device that needs additional node memory per GPU allocation would
    ///        have {ResourceName: "memory", allocationMultiplier: "2Gi"}.  Each allocated
    ///           GPU device instance of this type will account for 2Gi of memory.
    ///
    /// 2.  If `capacityKey` IS set: `allocationMultiplier` is multiplied by the amount of that capacity consumed.
    ///        The final node allocatable resource amount is `consumedCapacity\[capacityKey\]` * `allocationMultiplier`.
    ///     For example, if a Device's capacity "dra.example.com/cores" is consumed,
    ///     and each "core" provides 2 "cpu"s, the mapping would be:
    ///     {ResourceName: "cpu", capacityKey: "dra.example.com/cores", allocationMultiplier: "2"}.
    ///     If a claim consumes 8 "dra.example.com/cores", the CPU footprint is 8 * 2 = 16.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_multiplier: Option<
        <::k8s_openapi028::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// CapacityKey references a capacity name defined as a key in the `spec.devices\[*\].capacity` map. When this field is set, the value associated with this key in the `status.allocation.devices.results\[*\].consumedCapacity` map (for a specific claim allocation) determines the base quantity for the node allocatable resource. If `allocationMultiplier` is also set, it is multiplied with the base quantity. For example, if `spec.devices\[*\].capacity` has an entry "dra.example.com/memory": "128Gi", and this field is set to "dra.example.com/memory", then for a claim allocation that consumes { "dra.example.com/memory": "4Gi" } the base quantity for the node allocatable resource mapping will be "4Gi", and `allocationMultiplier` should be omitted or set to "1".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_key: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::resource::v1::NodeAllocatableResourceMapping {
    type Optioned = NodeAllocatableResourceMappingAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAllocatableResourceMappingAc {
    type Optioned = NodeAllocatableResourceMappingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::resource::v1::NodeAllocatableResourceMapping {
    fn into_optioned(self) -> NodeAllocatableResourceMappingAc {
        NodeAllocatableResourceMappingAc {
            allocation_multiplier: crate::OptionableConvert::into_optioned(
                self.allocation_multiplier,
            ),
            capacity_key: self.capacity_key,
        }
    }
    fn try_from_optioned(
        value: NodeAllocatableResourceMappingAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            allocation_multiplier: crate::OptionableConvert::try_from_optioned(
                value.allocation_multiplier,
            )?,
            capacity_key: value.capacity_key,
        })
    }
    fn merge(
        &mut self,
        other: NodeAllocatableResourceMappingAc,
    ) -> Result<(), crate::Error> {
        if self.allocation_multiplier.is_none() {
            self.allocation_multiplier = crate::OptionableConvert::try_from_optioned(
                other.allocation_multiplier,
            )?;
        } else if let Some(self_value) = self.allocation_multiplier.as_mut()
            && let Some(other_value) = other.allocation_multiplier
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.capacity_key.is_none() {
            self.capacity_key = crate::OptionableConvert::try_from_optioned(
                other.capacity_key,
            )?;
        } else if let Some(self_value) = self.capacity_key.as_mut()
            && let Some(other_value) = other.capacity_key
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::resource::v1::NodeAllocatableResourceMapping,
> for NodeAllocatableResourceMappingAc {
    fn from_optionable(
        value: k8s_openapi028::api::resource::v1::NodeAllocatableResourceMapping,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::resource::v1::NodeAllocatableResourceMapping,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::resource::v1::NodeAllocatableResourceMapping,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for NodeAllocatableResourceMappingAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.allocation_multiplier,
            other.allocation_multiplier,
        );
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.capacity_key,
            other.capacity_key,
        );
    }
}
