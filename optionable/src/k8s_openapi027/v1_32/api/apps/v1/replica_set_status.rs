#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ReplicaSetStatus represents the current status of a ReplicaSet.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplicaSetStatusAc {
    /// The number of available replicas (ready for at least minReadySeconds) for this replica set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    /// Represents the latest available observations of a replica set's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::apps::v1::ReplicaSetCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// The number of pods that have labels matching the labels of the pod template of the replicaset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: Option<i32>,
    /// ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// readyReplicas is the number of pods targeted by this ReplicaSet with a Ready Condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// Replicas is the most recently observed number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::ReplicaSetStatus {
    type Optioned = ReplicaSetStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicaSetStatusAc {
    type Optioned = ReplicaSetStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::ReplicaSetStatus {
    fn into_optioned(self) -> ReplicaSetStatusAc {
        ReplicaSetStatusAc {
            available_replicas: self.available_replicas,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            fully_labeled_replicas: self.fully_labeled_replicas,
            observed_generation: self.observed_generation,
            ready_replicas: self.ready_replicas,
            replicas: Some(self.replicas),
        }
    }
    fn try_from_optioned(value: ReplicaSetStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            available_replicas: value.available_replicas,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            fully_labeled_replicas: value.fully_labeled_replicas,
            observed_generation: value.observed_generation,
            ready_replicas: value.ready_replicas,
            replicas: value
                .replicas
                .ok_or(crate::Error {
                    missing_field: "replicas",
                })?,
        })
    }
    fn merge(&mut self, other: ReplicaSetStatusAc) -> Result<(), crate::Error> {
        if self.available_replicas.is_none() {
            self.available_replicas = crate::OptionableConvert::try_from_optioned(
                other.available_replicas,
            )?;
        } else if let Some(self_value) = self.available_replicas.as_mut()
            && let Some(other_value) = other.available_replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.fully_labeled_replicas.is_none() {
            self.fully_labeled_replicas = crate::OptionableConvert::try_from_optioned(
                other.fully_labeled_replicas,
            )?;
        } else if let Some(self_value) = self.fully_labeled_replicas.as_mut()
            && let Some(other_value) = other.fully_labeled_replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
        if self.ready_replicas.is_none() {
            self.ready_replicas = crate::OptionableConvert::try_from_optioned(
                other.ready_replicas,
            )?;
        } else if let Some(self_value) = self.ready_replicas.as_mut()
            && let Some(other_value) = other.ready_replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.replicas {
            self.replicas = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::ReplicaSetStatus>
for ReplicaSetStatusAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::ReplicaSetStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::ReplicaSetStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::ReplicaSetStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
