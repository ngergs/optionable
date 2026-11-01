#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeAllocatableOverhead defines auxiliary resource overheads incurred when allocating a device. Overheads can be specified as a fixed cost per pod referencing the claim, a variable cost per container reference, or both. Kubelet accounts for this overhead by adding it to both the pod-level and container-level cgroups of referencing containers.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeAllocatableOverheadAc {
    /// PerContainer is applied per container reference to the claim. This models overhead scaling linearly with the number of containers actively using the device. When both PerPod and PerContainer are specified, the total overhead allocated for each pod referencing the claim is computed as: Quantity = PerPod + (PerContainer * NumReferences) Kubelet accounts for this overhead in cgroups: - Pod-level cgroup (requests and limits): Kubelet adds PerPod + (PerContainer * NumReferences). - Container-level cgroup (limits only): Kubelet adds PerPod + PerContainer for each referencing container. This allows any single container to access the pod-level overhead, while the parent cgroup caps the total usage to account for PerPod exactly once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_container: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
    /// PerPod is overhead applied once per pod referencing the claim on this node. This is a flat overhead incurred for every pod referencing the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_pod: Option<
        <::k8s_openapi029::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::resource::v1beta2::NodeAllocatableOverhead {
    type Optioned = NodeAllocatableOverheadAc;
}
#[automatically_derived]
impl crate::Optionable for NodeAllocatableOverheadAc {
    type Optioned = NodeAllocatableOverheadAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1beta2::NodeAllocatableOverhead {
    fn into_optioned(self) -> NodeAllocatableOverheadAc {
        NodeAllocatableOverheadAc {
            per_container: crate::OptionableConvert::into_optioned(self.per_container),
            per_pod: crate::OptionableConvert::into_optioned(self.per_pod),
        }
    }
    fn try_from_optioned(
        value: NodeAllocatableOverheadAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            per_container: crate::OptionableConvert::try_from_optioned(
                value.per_container,
            )?,
            per_pod: crate::OptionableConvert::try_from_optioned(value.per_pod)?,
        })
    }
    fn merge(&mut self, other: NodeAllocatableOverheadAc) -> Result<(), crate::Error> {
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::resource::v1beta2::NodeAllocatableOverhead,
> for NodeAllocatableOverheadAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1beta2::NodeAllocatableOverhead,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1beta2::NodeAllocatableOverhead,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1beta2::NodeAllocatableOverhead,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for NodeAllocatableOverheadAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.per_container,
            other.per_container,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.per_pod, other.per_pod);
    }
}
