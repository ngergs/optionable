#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAllocatableMapping defines how a DRA allocation directly translates into a node allocatable resource quantity. The mapping can be derived from either the count of allocated devices or the specific capacity consumed. These options are mutually exclusive. Kubelet adds this mapped resource quantity from claim to both requests and limits at the pod-level cgroup, and to limits at the container-level cgroup for each container referencing the claim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAllocatableMappingAc {
    /// CapacityKey references a capacity name defined as a key in the `spec.devices\[*\].capacity` map. When this field is set, the value associated with this key in the `status.allocation.devices.results\[*\].consumedCapacity` map (for a specific claim allocation) determines the base quantity for the node allocatable resource. `capacityMultiplier` must also be set and is multiplied with the base quantity. For example, if `spec.devices\[*\].capacity` has an entry "dra.example.com/memory": "128Gi", and this field is set to "dra.example.com/memory", then for a claim allocation that consumes { "dra.example.com/memory": "4Gi" } the base quantity for the node allocatable resource mapping will be "4Gi". The final node allocatable resource amount is `consumedCapacity\[capacityKey\]` * `capacityMultiplier`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_key: Option<std::string::String>,
    /// CapacityMultiplier is used as a multiplier for the allocated capacity consumed. It is only valid if `capacityKey` is set. The final node allocatable resource amount is `consumedCapacity\[capacityKey\]` * `capacityMultiplier`. For example, if a Device's capacity "dra.example.com/cores" is consumed, and each "core" provides 2 "cpu"s, the mapping would be: {ResourceName: "cpu", capacityKey: "dra.example.com/cores", capacityMultiplier: "2"}. If a claim consumes 8 "dra.example.com/cores", the CPU footprint is 8 * 2 = 16.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_multiplier: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// DeviceMultiplier is used as a multiplier for the allocated device count in the claim. The final node allocatable resource amount is `deviceCount` * `deviceMultiplier`. For example, a DRA driver representing each cache complex (CCX) as a device would have {ResourceName: "cpu", deviceMultiplier: "8"} in its `nodeAllocatableResources`. If 2 devices (CCX) are allocated to the claim, 2 * 8 = 16 CPUs would be considered as allocated. It is only valid when `capacityKey` and `capacityMultiplier` are not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_multiplier: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::resource::v1beta1::NodeAllocatableMapping {
    type Optioned = NodeAllocatableMappingAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAllocatableMappingAc {
    type Optioned = NodeAllocatableMappingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1beta1::NodeAllocatableMapping {
    fn into_optioned(self) -> NodeAllocatableMappingAc {
        NodeAllocatableMappingAc {
            capacity_key: self.capacity_key,
            capacity_multiplier: crate::OptionableConvert::into_optioned(
                self.capacity_multiplier,
            ),
            device_multiplier: crate::OptionableConvert::into_optioned(
                self.device_multiplier,
            ),
        }
    }
    fn try_from_optioned(value: NodeAllocatableMappingAc) -> Result<Self, crate::Error> {
        Ok(Self {
            capacity_key: value.capacity_key,
            capacity_multiplier: crate::OptionableConvert::try_from_optioned(
                value.capacity_multiplier,
            )?,
            device_multiplier: crate::OptionableConvert::try_from_optioned(
                value.device_multiplier,
            )?,
        })
    }
    fn merge(&mut self, other: NodeAllocatableMappingAc) -> Result<(), crate::Error> {
        if self.capacity_key.is_none() {
            self.capacity_key = crate::OptionableConvert::try_from_optioned(
                other.capacity_key,
            )?;
        } else if let Some(self_value) = self.capacity_key.as_mut()
            && let Some(other_value) = other.capacity_key
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.capacity_multiplier.is_none() {
            self.capacity_multiplier = crate::OptionableConvert::try_from_optioned(
                other.capacity_multiplier,
            )?;
        } else if let Some(self_value) = self.capacity_multiplier.as_mut()
            && let Some(other_value) = other.capacity_multiplier
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.device_multiplier.is_none() {
            self.device_multiplier = crate::OptionableConvert::try_from_optioned(
                other.device_multiplier,
            )?;
        } else if let Some(self_value) = self.device_multiplier.as_mut()
            && let Some(other_value) = other.device_multiplier
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::resource::v1beta1::NodeAllocatableMapping,
> for NodeAllocatableMappingAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1beta1::NodeAllocatableMapping,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1beta1::NodeAllocatableMapping,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1beta1::NodeAllocatableMapping,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for NodeAllocatableMappingAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.capacity_key,
            other.capacity_key,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.capacity_multiplier,
            other.capacity_multiplier,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.device_multiplier,
            other.device_multiplier,
        );
    }
}
