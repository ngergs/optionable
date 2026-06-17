#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodGroupStatus represents information about the status of a pod group.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodGroupStatusAc {
    /// Conditions represent the latest observations of the PodGroup's state.
    ///
    /// Known condition types: - "PodGroupScheduled": Indicates whether the scheduling requirement has been satisfied. - "DisruptionTarget": Indicates whether the PodGroup is about to be terminated
    ///   due to disruption such as preemption.
    ///
    /// Known reasons for the PodGroupScheduled condition: - "Unschedulable": The PodGroup cannot be scheduled due to resource constraints,
    ///   affinity/anti-affinity rules, or insufficient capacity for the gang.
    /// - "SchedulerError": The PodGroup cannot be scheduled due to some internal error
    ///   that happened during scheduling, for example due to nodeAffinity parsing errors.
    ///
    /// Known reasons for the DisruptionTarget condition: - "PreemptionByScheduler": The PodGroup was preempted by the scheduler to make room for
    ///   higher-priority PodGroups or Pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi028::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// Status of resource claims.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_statuses: Option<
        std::vec::Vec<
            <::k8s_openapi028::api::scheduling::v1alpha2::PodGroupResourceClaimStatus as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::api::scheduling::v1alpha2::PodGroupStatus {
    type Optioned = PodGroupStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodGroupStatusAc {
    type Optioned = PodGroupStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::scheduling::v1alpha2::PodGroupStatus {
    fn into_optioned(self) -> PodGroupStatusAc {
        PodGroupStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            resource_claim_statuses: crate::OptionableConvert::into_optioned(
                self.resource_claim_statuses,
            ),
        }
    }
    fn try_from_optioned(value: PodGroupStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            resource_claim_statuses: crate::OptionableConvert::try_from_optioned(
                value.resource_claim_statuses,
            )?,
        })
    }
    fn merge(&mut self, other: PodGroupStatusAc) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.resource_claim_statuses.is_none() {
            self.resource_claim_statuses = crate::OptionableConvert::try_from_optioned(
                other.resource_claim_statuses,
            )?;
        } else if let Some(self_value) = self.resource_claim_statuses.as_mut()
            && let Some(other_value) = other.resource_claim_statuses
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::api::scheduling::v1alpha2::PodGroupStatus>
for PodGroupStatusAc {
    fn from_optionable(
        value: k8s_openapi028::api::scheduling::v1alpha2::PodGroupStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::scheduling::v1alpha2::PodGroupStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::scheduling::v1alpha2::PodGroupStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for PodGroupStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.conditions,
            other.conditions,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.resource_claim_statuses,
            other.resource_claim_statuses,
        );
    }
}
