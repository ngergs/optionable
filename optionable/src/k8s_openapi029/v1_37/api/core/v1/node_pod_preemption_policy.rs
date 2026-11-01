#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodePodPreemptionPolicy defines the node-level policies governing preemption for pods on this node.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodePodPreemptionPolicyAc {
    /// DisableResizePreemption lists the owners (e.g., autoscalers, operators, administrators) that have requested to disable scheduler and Kubelet preemption for in-place pod resize on this node. If this list is non-empty, resize-induced preemption is disabled on this node. This is an alpha field and requires enabling the InPlacePodVerticalScalingSchedulerPreemption feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_resize_preemption: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::core::v1::NodePodPreemptionPolicy {
    type Optioned = NodePodPreemptionPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for NodePodPreemptionPolicyAc {
    type Optioned = NodePodPreemptionPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::core::v1::NodePodPreemptionPolicy {
    fn into_optioned(self) -> NodePodPreemptionPolicyAc {
        NodePodPreemptionPolicyAc {
            disable_resize_preemption: self.disable_resize_preemption,
        }
    }
    fn try_from_optioned(
        value: NodePodPreemptionPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            disable_resize_preemption: value.disable_resize_preemption,
        })
    }
    fn merge(&mut self, other: NodePodPreemptionPolicyAc) -> Result<(), crate::Error> {
        if self.disable_resize_preemption.is_none() {
            self.disable_resize_preemption = crate::OptionableConvert::try_from_optioned(
                other.disable_resize_preemption,
            )?;
        } else if let Some(self_value) = self.disable_resize_preemption.as_mut()
            && let Some(other_value) = other.disable_resize_preemption
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::core::v1::NodePodPreemptionPolicy>
for NodePodPreemptionPolicyAc {
    fn from_optionable(
        value: k8s_openapi029::api::core::v1::NodePodPreemptionPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::core::v1::NodePodPreemptionPolicy, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::NodePodPreemptionPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for NodePodPreemptionPolicyAc {
    fn merge_from(&mut self, other: Self) {
        crate::merge::merge_append_not_present_option_wrapped(
            &mut self.disable_resize_preemption,
            other.disable_resize_preemption,
        );
    }
}
