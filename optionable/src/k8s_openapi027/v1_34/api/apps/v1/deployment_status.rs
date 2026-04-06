#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeploymentStatus is the most recently observed status of the Deployment.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeploymentStatusAc {
    /// Total number of available non-terminating pods (ready for at least minReadySeconds) targeted by this deployment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    /// Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    /// Represents the latest available observations of a deployment's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::apps::v1::DeploymentCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// The generation observed by the deployment controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// Total number of non-terminating pods targeted by this Deployment with a Ready Condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// Total number of non-terminating pods targeted by this deployment (their labels match the selector).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Total number of terminating pods targeted by this deployment. Terminating pods have a non-null .metadata.deletionTimestamp and have not yet reached the Failed or Succeeded .status.phase.
    ///
    /// This is an alpha field. Enable DeploymentReplicaSetTerminatingReplicas to be able to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_replicas: Option<i32>,
    /// Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,
    /// Total number of non-terminating pods targeted by this deployment that have the desired template spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::DeploymentStatus {
    type Optioned = DeploymentStatusAc;
}
#[automatically_derived]
impl crate::Optionable for DeploymentStatusAc {
    type Optioned = DeploymentStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::DeploymentStatus {
    fn into_optioned(self) -> DeploymentStatusAc {
        DeploymentStatusAc {
            available_replicas: self.available_replicas,
            collision_count: self.collision_count,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            observed_generation: self.observed_generation,
            ready_replicas: self.ready_replicas,
            replicas: self.replicas,
            terminating_replicas: self.terminating_replicas,
            unavailable_replicas: self.unavailable_replicas,
            updated_replicas: self.updated_replicas,
        }
    }
    fn try_from_optioned(value: DeploymentStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            available_replicas: value.available_replicas,
            collision_count: value.collision_count,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            observed_generation: value.observed_generation,
            ready_replicas: value.ready_replicas,
            replicas: value.replicas,
            terminating_replicas: value.terminating_replicas,
            unavailable_replicas: value.unavailable_replicas,
            updated_replicas: value.updated_replicas,
        })
    }
    fn merge(&mut self, other: DeploymentStatusAc) -> Result<(), crate::Error> {
        self.available_replicas = other.available_replicas;
        self.collision_count = other.collision_count;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        self.observed_generation = other.observed_generation;
        self.ready_replicas = other.ready_replicas;
        self.replicas = other.replicas;
        self.terminating_replicas = other.terminating_replicas;
        self.unavailable_replicas = other.unavailable_replicas;
        self.updated_replicas = other.updated_replicas;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::DeploymentStatus>
for DeploymentStatusAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::DeploymentStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::DeploymentStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::DeploymentStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
