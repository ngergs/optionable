#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAllocatableResourceClaimStatus describes the status of node allocatable resources allocated via DRA.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAllocatableResourceClaimStatusAc {
    /// Containers lists the names of all containers in this pod that reference the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<std::vec::Vec<std::string::String>>,
    /// Mapping contains allocations through devices mapped in the device spec's `nodeAllocatableResources\[...\].mapping` field. This is used by kubelet for pod level and container-level cgroup enforcement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::core::v1::NodeAllocatableMappedResources as crate::Optionable>::Optioned,
        >,
    >,
    /// Overhead contains allocations through devices mapped in the device spec's `nodeAllocatableResources\[...\].overhead` field. This is used by kubelet for pod level and container-level cgroup enforcement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::core::v1::NodeAllocatableOverheadResources as crate::Optionable>::Optioned,
        >,
    >,
    /// ResourceClaimName is the resource claim referenced by the pod that resulted in this node allocatable resource allocation.
    pub resource_claim_name: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::core::v1::NodeAllocatableResourceClaimStatus {
    type Optioned = NodeAllocatableResourceClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAllocatableResourceClaimStatusAc {
    type Optioned = NodeAllocatableResourceClaimStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::core::v1::NodeAllocatableResourceClaimStatus {
    fn into_optioned(self) -> NodeAllocatableResourceClaimStatusAc {
        NodeAllocatableResourceClaimStatusAc {
            containers: self.containers,
            mapping: crate::OptionableConvert::into_optioned(self.mapping),
            overhead: crate::OptionableConvert::into_optioned(self.overhead),
            resource_claim_name: self.resource_claim_name,
        }
    }
    fn try_from_optioned(
        value: NodeAllocatableResourceClaimStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            containers: value.containers,
            mapping: crate::OptionableConvert::try_from_optioned(value.mapping)?,
            overhead: crate::OptionableConvert::try_from_optioned(value.overhead)?,
            resource_claim_name: value.resource_claim_name,
        })
    }
    fn merge(
        &mut self,
        other: NodeAllocatableResourceClaimStatusAc,
    ) -> Result<(), crate::Error> {
        if self.containers.is_none() {
            self.containers = crate::OptionableConvert::try_from_optioned(
                other.containers,
            )?;
        } else if let Some(self_value) = self.containers.as_mut()
            && let Some(other_value) = other.containers
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        if self.mapping.is_none() {
            self.mapping = crate::OptionableConvert::try_from_optioned(other.mapping)?;
        } else if let Some(self_value) = self.mapping.as_mut()
            && let Some(other_value) = other.mapping
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.overhead.is_none() {
            self.overhead = crate::OptionableConvert::try_from_optioned(other.overhead)?;
        } else if let Some(self_value) = self.overhead.as_mut()
            && let Some(other_value) = other.overhead
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        self.resource_claim_name = other.resource_claim_name;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::core::v1::NodeAllocatableResourceClaimStatus {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.resource_claim_name == other.resource_claim_name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::core::v1::NodeAllocatableResourceClaimStatus,
> for NodeAllocatableResourceClaimStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::core::v1::NodeAllocatableResourceClaimStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::core::v1::NodeAllocatableResourceClaimStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::NodeAllocatableResourceClaimStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for NodeAllocatableResourceClaimStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::merge::merge_append_not_present_option_wrapped(
            &mut self.containers,
            other.containers,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.mapping,
            other.mapping,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.overhead,
            other.overhead,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.resource_claim_name,
            other.resource_claim_name,
        );
    }
}
impl crate::merge::MapKeysEq for NodeAllocatableResourceClaimStatusAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.resource_claim_name == other.resource_claim_name
    }
}
