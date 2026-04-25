#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ReplicationControllerStatus represents the current status of a replication controller.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplicationControllerStatusAc {
    /// The number of available replicas (ready for at least minReadySeconds) for this replication controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    /// Represents the latest available observations of a replication controller's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ReplicationControllerCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// The number of pods that have labels matching the labels of the pod template of the replication controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: Option<i32>,
    /// ObservedGeneration reflects the generation of the most recently observed replication controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// The number of ready replicas for this replication controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// Replicas is the most recently observed number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ReplicationControllerStatus {
    type Optioned = ReplicationControllerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicationControllerStatusAc {
    type Optioned = ReplicationControllerStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ReplicationControllerStatus {
    fn into_optioned(self) -> ReplicationControllerStatusAc {
        ReplicationControllerStatusAc {
            available_replicas: self.available_replicas,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            fully_labeled_replicas: self.fully_labeled_replicas,
            observed_generation: self.observed_generation,
            ready_replicas: self.ready_replicas,
            replicas: Some(self.replicas),
        }
    }
    fn try_from_optioned(
        value: ReplicationControllerStatusAc,
    ) -> Result<Self, crate::Error> {
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
    fn merge(
        &mut self,
        other: ReplicationControllerStatusAc,
    ) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ReplicationControllerStatus>
for ReplicationControllerStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ReplicationControllerStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ReplicationControllerStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ReplicationControllerStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ReplicationControllerStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.available_replicas,
            other.available_replicas,
        );
        crate::k8s_openapi::merge::merge_map(&mut self.conditions, other.conditions);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.fully_labeled_replicas,
            other.fully_labeled_replicas,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.observed_generation,
            other.observed_generation,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.ready_replicas,
            other.ready_replicas,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.replicas, other.replicas);
    }
}
