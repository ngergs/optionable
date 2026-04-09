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
    pub conditions: Option<
        std::vec::Vec<::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Condition>,
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
            conditions: self.conditions,
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
            conditions: value.conditions,
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
        self.conditions = other.conditions;
        if let Some(other_value) = other.current_healthy {
            self.current_healthy = other_value;
        }
        if let Some(other_value) = other.desired_healthy {
            self.desired_healthy = other_value;
        }
        crate::OptionableConvert::merge(&mut self.disrupted_pods, other.disrupted_pods)?;
        if let Some(other_value) = other.disruptions_allowed {
            self.disruptions_allowed = other_value;
        }
        if let Some(other_value) = other.expected_pods {
            self.expected_pods = other_value;
        }
        self.observed_generation = other.observed_generation;
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
