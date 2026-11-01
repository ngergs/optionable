#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadPodGroupGangSchedulingPolicy defines the parameters for gang (all-or-nothing) scheduling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadPodGroupGangSchedulingPolicyAc {
    /// minCount is the minimum number of pods that must be scheduled at the same time for the scheduler to admit the entire group. This field is optional. If it is not specified, the controller should inject a context-specific sane default (e.g., parallelism for a Job). If set, it must be a positive integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupGangSchedulingPolicy {
    type Optioned = WorkloadPodGroupGangSchedulingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadPodGroupGangSchedulingPolicyAc {
    type Optioned = WorkloadPodGroupGangSchedulingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupGangSchedulingPolicy {
    fn into_optioned(self) -> WorkloadPodGroupGangSchedulingPolicyAc {
        WorkloadPodGroupGangSchedulingPolicyAc {
            min_count: self.min_count,
        }
    }
    fn try_from_optioned(
        value: WorkloadPodGroupGangSchedulingPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self { min_count: value.min_count })
    }
    fn merge(
        &mut self,
        other: WorkloadPodGroupGangSchedulingPolicyAc,
    ) -> Result<(), crate::Error> {
        if self.min_count.is_none() {
            self.min_count = crate::OptionableConvert::try_from_optioned(
                other.min_count,
            )?;
        } else if let Some(self_value) = self.min_count.as_mut()
            && let Some(other_value) = other.min_count
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupGangSchedulingPolicy,
> for WorkloadPodGroupGangSchedulingPolicyAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupGangSchedulingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupGangSchedulingPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupGangSchedulingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for WorkloadPodGroupGangSchedulingPolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.min_count, other.min_count);
    }
}
