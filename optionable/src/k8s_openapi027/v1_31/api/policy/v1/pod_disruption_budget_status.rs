#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodDisruptionBudgetStatus represents information about the status of a PodDisruptionBudget. Status may trail the actual state of a system.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodDisruptionBudgetStatusAc {
    /// Conditions contain conditions for PDB. The disruption controller sets the DisruptionAllowed condition. The following are known values for the reason field (additional reasons could be added in the future): - SyncFailed: The controller encountered an error and wasn't able to compute
    ///               the number of allowed disruptions. Therefore no disruptions are
    ///               allowed and the status of the condition will be False.
    /// - InsufficientPods: The number of pods are either at or below the number
    ///                     required by the PodDisruptionBudget. No disruptions are
    ///                     allowed and the status of the condition will be False.
    /// - SufficientPods: There are more pods than required by the PodDisruptionBudget.
    ///                   The condition will be True, and the number of allowed
    ///                   disruptions are provided by the disruptionsAllowed property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// current number of healthy pods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_healthy: Option<i32>,
    /// minimum desired number of healthy pods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_healthy: Option<i32>,
    /// DisruptedPods contains information about pods whose eviction was processed by the API server eviction subresource handler but has not yet been observed by the PodDisruptionBudget controller. A pod will be in this map from the time when the API server processed the eviction request to the time when the pod is seen by PDB controller as having been marked for deletion (or after a timeout). The key in the map is the name of the pod and the value is the time when the API server processed the eviction request. If the deletion didn't occur and a pod is still there it will be removed from the list automatically by PodDisruptionBudget controller after some time. If everything goes smooth this map should be empty for the most of the time. Large number of entries in the map may indicate problems with pod deletions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disrupted_pods: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
        >,
    >,
    /// Number of pod disruptions that are currently allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruptions_allowed: Option<i32>,
    /// total number of pods counted by this disruption budget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_pods: Option<i32>,
    /// Most recent generation observed when updating this PDB status. DisruptionsAllowed and other status information is valid only if observedGeneration equals to PDB's object generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::policy::v1::PodDisruptionBudgetStatus {
    type Optioned = PodDisruptionBudgetStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodDisruptionBudgetStatusAc {
    type Optioned = PodDisruptionBudgetStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::policy::v1::PodDisruptionBudgetStatus {
    fn into_optioned(self) -> PodDisruptionBudgetStatusAc {
        PodDisruptionBudgetStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_healthy: Some(self.current_healthy),
            desired_healthy: Some(self.desired_healthy),
            disrupted_pods: crate::OptionableConvert::into_optioned(self.disrupted_pods),
            disruptions_allowed: Some(self.disruptions_allowed),
            expected_pods: Some(self.expected_pods),
            observed_generation: self.observed_generation,
        }
    }
    fn try_from_optioned(
        value: PodDisruptionBudgetStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_healthy: value
                .current_healthy
                .ok_or(crate::Error {
                    missing_field: "current_healthy",
                })?,
            desired_healthy: value
                .desired_healthy
                .ok_or(crate::Error {
                    missing_field: "desired_healthy",
                })?,
            disrupted_pods: crate::OptionableConvert::try_from_optioned(
                value.disrupted_pods,
            )?,
            disruptions_allowed: value
                .disruptions_allowed
                .ok_or(crate::Error {
                    missing_field: "disruptions_allowed",
                })?,
            expected_pods: value
                .expected_pods
                .ok_or(crate::Error {
                    missing_field: "expected_pods",
                })?,
            observed_generation: value.observed_generation,
        })
    }
    fn merge(&mut self, other: PodDisruptionBudgetStatusAc) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if let Some(other_value) = other.current_healthy {
            self.current_healthy = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.desired_healthy {
            self.desired_healthy = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.disrupted_pods.is_none() {
            self.disrupted_pods = crate::OptionableConvert::try_from_optioned(
                other.disrupted_pods,
            )?;
        } else if let Some(self_value) = self.disrupted_pods.as_mut()
            && let Some(other_value) = other.disrupted_pods
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.disruptions_allowed {
            self.disruptions_allowed = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.expected_pods {
            self.expected_pods = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.observed_generation.is_none() {
            self.observed_generation = crate::OptionableConvert::try_from_optioned(
                other.observed_generation,
            )?;
        } else if let Some(self_value) = self.observed_generation.as_mut()
            && let Some(other_value) = other.observed_generation
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::policy::v1::PodDisruptionBudgetStatus>
for PodDisruptionBudgetStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::policy::v1::PodDisruptionBudgetStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::policy::v1::PodDisruptionBudgetStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::policy::v1::PodDisruptionBudgetStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PodDisruptionBudgetStatusAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_map(&mut self.conditions, other.conditions);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.current_healthy,
            other.current_healthy,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.desired_healthy,
            other.desired_healthy,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.disrupted_pods,
            other.disrupted_pods,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.disruptions_allowed,
            other.disruptions_allowed,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.expected_pods,
            other.expected_pods,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.observed_generation,
            other.observed_generation,
        );
    }
}
