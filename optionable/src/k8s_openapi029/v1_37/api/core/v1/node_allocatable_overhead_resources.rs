#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAllocatableOverheadResources describes auxiliary overhead resource allocations.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAllocatableOverheadResourcesAc {
    /// Name is the name of the resource (e.g., cpu, memory).
    pub name: std::string::String,
    /// PerContainer is the variable overhead quantity applied for each container referencing the claim. The container references are recorded in `nodeAllocatableResourceClaimStatuses.containers`. The total overhead quantity allocated for the claim is computed as: Quantity = PerPod + (PerContainer * NumReferences) Kubelet accounts for this overhead in cgroups: - Pod-level cgroup (requests and limits): Kubelet adds PerPod + (PerContainer * NumReferences). - Container-level cgroup (limits only): Kubelet adds PerPod + PerContainer for each referencing container. This allows any single container to access the pod-level overhead, while the parent cgroup caps the total usage to account for PerPod exactly once. At least one of PerPod or PerContainer must be specified. Specifying neither is an invalid configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_container: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// PerPod is the flat overhead quantity allocated per pod. Adding to each container limit allows individual containers to utilize the overhead, while the parent pod-level cgroup limit caps the total usage at the pod boundary where the overhead is accounted for exactly once. At least one of PerPod or PerContainer must be specified. Specifying neither is an invalid configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_pod: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::core::v1::NodeAllocatableOverheadResources {
    type Optioned = NodeAllocatableOverheadResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAllocatableOverheadResourcesAc {
    type Optioned = NodeAllocatableOverheadResourcesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::core::v1::NodeAllocatableOverheadResources {
    fn into_optioned(self) -> NodeAllocatableOverheadResourcesAc {
        NodeAllocatableOverheadResourcesAc {
            name: self.name,
            per_container: crate::OptionableConvert::into_optioned(self.per_container),
            per_pod: crate::OptionableConvert::into_optioned(self.per_pod),
        }
    }
    fn try_from_optioned(
        value: NodeAllocatableOverheadResourcesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            per_container: crate::OptionableConvert::try_from_optioned(
                value.per_container,
            )?,
            per_pod: crate::OptionableConvert::try_from_optioned(value.per_pod)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeAllocatableOverheadResourcesAc,
    ) -> Result<(), crate::Error> {
        self.name = other.name;
        if self.per_container.is_none() {
            self.per_container = crate::OptionableConvert::try_from_optioned(
                other.per_container,
            )?;
        } else if let Some(self_value) = self.per_container.as_mut()
            && let Some(other_value) = other.per_container
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.per_pod.is_none() {
            self.per_pod = crate::OptionableConvert::try_from_optioned(other.per_pod)?;
        } else if let Some(self_value) = self.per_pod.as_mut()
            && let Some(other_value) = other.per_pod
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::core::v1::NodeAllocatableOverheadResources {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::core::v1::NodeAllocatableOverheadResources,
> for NodeAllocatableOverheadResourcesAc {
    fn from_optionable(
        value: k8s_openapi029::api::core::v1::NodeAllocatableOverheadResources,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::core::v1::NodeAllocatableOverheadResources,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::NodeAllocatableOverheadResources,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for NodeAllocatableOverheadResourcesAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.per_container,
            other.per_container,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.per_pod, other.per_pod);
    }
}
impl crate::merge::MapKeysEq for NodeAllocatableOverheadResourcesAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
