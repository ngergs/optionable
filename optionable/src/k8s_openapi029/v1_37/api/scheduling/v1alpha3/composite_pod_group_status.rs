#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CompositePodGroupStatus represents information about the status of a composite pod group.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompositePodGroupStatusAc {
    /// conditions represent the latest observations of the CompositePodGroup's state.
    ///
    /// Known condition types: - "CompositePodGroupInitiallyScheduled": Indicates whether the overall scheduling requirement
    ///   for the subtree under this CompositePodGroup has been satisfied. Once this condition
    ///   transitions to True, it serves as a terminal state and will never revert to False,
    ///   even if pods are subsequently deleted and group constraints are no longer met.
    /// - "DisruptionTarget": Indicates whether the CompositePodGroup is about to be terminated
    ///   due to disruption such as preemption.
    ///
    /// Known reasons for the CompositePodGroupInitiallyScheduled condition: - "Unschedulable": The CompositePodGroup's subtree could not be placed due to resource constraints,
    ///   affinity/anti-affinity, or topological constraints.
    /// - "SchedulerError": The CompositePodGroup cannot be scheduled due to some internal error
    ///   that occurred during scheduling.
    /// - "Invalid": Set to True when kube-scheduler detects an invalid group layout during
    ///   runtime validation. The `message` field details the specific layout violation (such as
    ///   a detected cycle, exceeding the maximum depth of 4, or referencing multiple distinct Workloads).
    ///
    /// Known reasons for the DisruptionTarget condition: - "PreemptionByScheduler": The CompositePodGroup was targeted by the scheduler's preemption loop
    ///   to free up capacity for higher-priority preemptors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi029::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupStatus {
    type Optioned = CompositePodGroupStatusAc;
}
#[automatically_derived]
impl crate::Optionable for CompositePodGroupStatusAc {
    type Optioned = CompositePodGroupStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupStatus {
    fn into_optioned(self) -> CompositePodGroupStatusAc {
        CompositePodGroupStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: CompositePodGroupStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(&mut self, other: CompositePodGroupStatusAc) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupStatus,
> for CompositePodGroupStatusAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::CompositePodGroupStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for CompositePodGroupStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.conditions,
            other.conditions,
        );
    }
}
