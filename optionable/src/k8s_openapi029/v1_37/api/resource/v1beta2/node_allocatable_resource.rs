#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAllocatableResource defines the translation between the DRA device/capacity units requested to the corresponding quantity of the node allocatable resource. At least one of Mapping or Overhead must be specified. Not specifying either is an invalid configuration.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAllocatableResourceAc {
    /// Mapping is used when the device directly models a node allocatable resource like standard CPU or memory (e.g., with a CPU DRA driver). The calculated quantity is accounted for exactly once per claim instance on the node. To prevent node cgroup isolation friction, the scheduler explicitly blocks sharing mapped device claims across multiple pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<
        <::k8s_openapi029::api::resource::v1beta2::NodeAllocatableMapping as crate::Optionable>::Optioned,
    >,
    /// Overhead contains fields for modeling auxiliary overhead incurred on node allocatable resources when allocating devices that are not themselves modeling a node allocatable resource (e.g., host memory overhead for GPUs). Sharing overhead-mapped claims across multiple pods is allowed. The node allocatable overhead is accounted for individually for each pod referencing the claim. Overhead is always subtracted from the node's allocatable capacity for the resource, even when mapping is specified for the same resource. Eg: If a device models memory capacity per socket as a consumable capacity pool via Mapping (with CapacityKey), any overhead specified for the same resource will be subtracted from the node's general allocatable capacity and not from the per-socket capacity pool in Mapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead: Option<
        <::k8s_openapi029::api::resource::v1beta2::NodeAllocatableOverhead as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::resource::v1beta2::NodeAllocatableResource {
    type Optioned = NodeAllocatableResourceAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAllocatableResourceAc {
    type Optioned = NodeAllocatableResourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1beta2::NodeAllocatableResource {
    fn into_optioned(self) -> NodeAllocatableResourceAc {
        NodeAllocatableResourceAc {
            mapping: crate::OptionableConvert::into_optioned(self.mapping),
            overhead: crate::OptionableConvert::into_optioned(self.overhead),
        }
    }
    fn try_from_optioned(
        value: NodeAllocatableResourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            mapping: crate::OptionableConvert::try_from_optioned(value.mapping)?,
            overhead: crate::OptionableConvert::try_from_optioned(value.overhead)?,
        })
    }
    fn merge(&mut self, other: NodeAllocatableResourceAc) -> Result<(), crate::Error> {
        if self.mapping.is_none() {
            self.mapping = crate::OptionableConvert::try_from_optioned(other.mapping)?;
        } else if let Some(self_value) = self.mapping.as_mut()
            && let Some(other_value) = other.mapping
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.overhead.is_none() {
            self.overhead = crate::OptionableConvert::try_from_optioned(other.overhead)?;
        } else if let Some(self_value) = self.overhead.as_mut()
            && let Some(other_value) = other.overhead
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::resource::v1beta2::NodeAllocatableResource,
> for NodeAllocatableResourceAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1beta2::NodeAllocatableResource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1beta2::NodeAllocatableResource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1beta2::NodeAllocatableResource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for NodeAllocatableResourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.mapping, other.mapping);
        k8s_openapi029::DeepMerge::merge_from(&mut self.overhead, other.overhead);
    }
}
