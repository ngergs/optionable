#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadPodGroupSchedulingPolicy defines the scheduling policy for a group of pods managed by a workload controller. Exactly one policy must be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadPodGroupSchedulingPolicyAc {
    /// basic specifies that standard, pod-by-pod Kubernetes scheduling behavior should be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupBasicSchedulingPolicy as crate::Optionable>::Optioned,
    >,
    /// gang specifies all-or-nothing scheduling semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gang: Option<
        <::k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupGangSchedulingPolicy as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy {
    type Optioned = WorkloadPodGroupSchedulingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadPodGroupSchedulingPolicyAc {
    type Optioned = WorkloadPodGroupSchedulingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy {
    fn into_optioned(self) -> WorkloadPodGroupSchedulingPolicyAc {
        WorkloadPodGroupSchedulingPolicyAc {
            basic: crate::OptionableConvert::into_optioned(self.basic),
            gang: crate::OptionableConvert::into_optioned(self.gang),
        }
    }
    fn try_from_optioned(
        value: WorkloadPodGroupSchedulingPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            basic: crate::OptionableConvert::try_from_optioned(value.basic)?,
            gang: crate::OptionableConvert::try_from_optioned(value.gang)?,
        })
    }
    fn merge(
        &mut self,
        other: WorkloadPodGroupSchedulingPolicyAc,
    ) -> Result<(), crate::Error> {
        if self.basic.is_none() {
            self.basic = crate::OptionableConvert::try_from_optioned(other.basic)?;
        } else if let Some(self_value) = self.basic.as_mut()
            && let Some(other_value) = other.basic
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.gang.is_none() {
            self.gang = crate::OptionableConvert::try_from_optioned(other.gang)?;
        } else if let Some(self_value) = self.gang.as_mut()
            && let Some(other_value) = other.gang
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy,
> for WorkloadPodGroupSchedulingPolicyAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupSchedulingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for WorkloadPodGroupSchedulingPolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.basic, other.basic);
        k8s_openapi029::DeepMerge::merge_from(&mut self.gang, other.gang);
    }
}
