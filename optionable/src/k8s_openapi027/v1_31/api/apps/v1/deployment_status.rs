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
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
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
    /// readyReplicas is the number of pods targeted by this Deployment with a Ready Condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Total number of unavailable pods targeted by this deployment. This is the total number of pods that are still required for the deployment to have 100% available capacity. They may either be pods that are running but not yet available or pods that still have not been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_replicas: Option<i32>,
    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
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
            unavailable_replicas: value.unavailable_replicas,
            updated_replicas: value.updated_replicas,
        })
    }
    fn merge(&mut self, other: DeploymentStatusAc) -> Result<(), crate::Error> {
        if self.available_replicas.is_none() {
            self.available_replicas = crate::OptionableConvert::try_from_optioned(
                other.available_replicas,
            )?;
        } else if let Some(self_value) = self.available_replicas.as_mut()
            && let Some(other_value) = other.available_replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.collision_count.is_none() {
            self.collision_count = crate::OptionableConvert::try_from_optioned(
                other.collision_count,
            )?;
        } else if let Some(self_value) = self.collision_count.as_mut()
            && let Some(other_value) = other.collision_count
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
        if self.replicas.is_none() {
            self.replicas = crate::OptionableConvert::try_from_optioned(other.replicas)?;
        } else if let Some(self_value) = self.replicas.as_mut()
            && let Some(other_value) = other.replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.unavailable_replicas.is_none() {
            self.unavailable_replicas = crate::OptionableConvert::try_from_optioned(
                other.unavailable_replicas,
            )?;
        } else if let Some(self_value) = self.unavailable_replicas.as_mut()
            && let Some(other_value) = other.unavailable_replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.updated_replicas.is_none() {
            self.updated_replicas = crate::OptionableConvert::try_from_optioned(
                other.updated_replicas,
            )?;
        } else if let Some(self_value) = self.updated_replicas.as_mut()
            && let Some(other_value) = other.updated_replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for DeploymentStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.available_replicas,
            other.available_replicas,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.collision_count,
            other.collision_count,
        );
        crate::k8s_openapi::merge::merge_map_option_wrapped(
            &mut self.conditions,
            other.conditions,
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
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.unavailable_replicas,
            other.unavailable_replicas,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.updated_replicas,
            other.updated_replicas,
        );
    }
}
