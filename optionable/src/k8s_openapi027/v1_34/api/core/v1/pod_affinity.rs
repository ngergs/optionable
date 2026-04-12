#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Pod affinity is a group of inter pod affinity scheduling rules.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodAffinityAc {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_during_scheduling_ignored_during_execution: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::WeightedPodAffinityTerm as crate::Optionable>::Optioned,
        >,
    >,
    /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_during_scheduling_ignored_during_execution: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodAffinityTerm as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodAffinity {
    type Optioned = PodAffinityAc;
}
#[automatically_derived]
impl crate::Optionable for PodAffinityAc {
    type Optioned = PodAffinityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodAffinity {
    fn into_optioned(self) -> PodAffinityAc {
        PodAffinityAc {
            preferred_during_scheduling_ignored_during_execution: crate::OptionableConvert::into_optioned(
                self.preferred_during_scheduling_ignored_during_execution,
            ),
            required_during_scheduling_ignored_during_execution: crate::OptionableConvert::into_optioned(
                self.required_during_scheduling_ignored_during_execution,
            ),
        }
    }
    fn try_from_optioned(value: PodAffinityAc) -> Result<Self, crate::Error> {
        Ok(Self {
            preferred_during_scheduling_ignored_during_execution: crate::OptionableConvert::try_from_optioned(
                value.preferred_during_scheduling_ignored_during_execution,
            )?,
            required_during_scheduling_ignored_during_execution: crate::OptionableConvert::try_from_optioned(
                value.required_during_scheduling_ignored_during_execution,
            )?,
        })
    }
    fn merge(&mut self, other: PodAffinityAc) -> Result<(), crate::Error> {
        if self.preferred_during_scheduling_ignored_during_execution.is_none() {
            self.preferred_during_scheduling_ignored_during_execution = other
                .preferred_during_scheduling_ignored_during_execution;
        }
        if let Some(other_value) = other
            .preferred_during_scheduling_ignored_during_execution
        {
            self.preferred_during_scheduling_ignored_during_execution = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.required_during_scheduling_ignored_during_execution.is_none() {
            self.required_during_scheduling_ignored_during_execution = other
                .required_during_scheduling_ignored_during_execution;
        }
        if let Some(other_value) = other
            .required_during_scheduling_ignored_during_execution
        {
            self.required_during_scheduling_ignored_during_execution = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodAffinity>
for PodAffinityAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodAffinity) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodAffinity, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodAffinity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
