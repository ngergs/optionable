#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StatefulSetStatus represents the current state of a StatefulSet.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetStatusAc {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,
    /// collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    /// Represents the latest available observations of a statefulset's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::apps::v1::StatefulSetCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by currentRevision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i32>,
    /// currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence \[0,currentReplicas).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_revision: Option<std::string::String>,
    /// observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the StatefulSet's generation, which is updated on mutation by the API Server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// readyReplicas is the number of pods created for this StatefulSet with a Ready Condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,
    /// replicas is the number of Pods created by the StatefulSet controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence \[replicas-updatedReplicas,replicas)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_revision: Option<std::string::String>,
    /// updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version indicated by updateRevision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::StatefulSetStatus {
    type Optioned = StatefulSetStatusAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetStatusAc {
    type Optioned = StatefulSetStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::StatefulSetStatus {
    fn into_optioned(self) -> StatefulSetStatusAc {
        StatefulSetStatusAc {
            available_replicas: self.available_replicas,
            collision_count: self.collision_count,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_replicas: self.current_replicas,
            current_revision: self.current_revision,
            observed_generation: self.observed_generation,
            ready_replicas: self.ready_replicas,
            replicas: Some(self.replicas),
            update_revision: self.update_revision,
            updated_replicas: self.updated_replicas,
        }
    }
    fn try_from_optioned(value: StatefulSetStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            available_replicas: value.available_replicas,
            collision_count: value.collision_count,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_replicas: value.current_replicas,
            current_revision: value.current_revision,
            observed_generation: value.observed_generation,
            ready_replicas: value.ready_replicas,
            replicas: value
                .replicas
                .ok_or(crate::Error {
                    missing_field: "replicas",
                })?,
            update_revision: value.update_revision,
            updated_replicas: value.updated_replicas,
        })
    }
    fn merge(&mut self, other: StatefulSetStatusAc) -> Result<(), crate::Error> {
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
        if self.current_replicas.is_none() {
            self.current_replicas = crate::OptionableConvert::try_from_optioned(
                other.current_replicas,
            )?;
        } else if let Some(self_value) = self.current_replicas.as_mut()
            && let Some(other_value) = other.current_replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.current_revision.is_none() {
            self.current_revision = crate::OptionableConvert::try_from_optioned(
                other.current_revision,
            )?;
        } else if let Some(self_value) = self.current_revision.as_mut()
            && let Some(other_value) = other.current_revision
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
        if self.update_revision.is_none() {
            self.update_revision = crate::OptionableConvert::try_from_optioned(
                other.update_revision,
            )?;
        } else if let Some(self_value) = self.update_revision.as_mut()
            && let Some(other_value) = other.update_revision
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
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::StatefulSetStatus>
for StatefulSetStatusAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::StatefulSetStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::StatefulSetStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::StatefulSetStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for StatefulSetStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.available_replicas,
            other.available_replicas,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.collision_count,
            other.collision_count,
        );
        crate::k8s_openapi::merge::merge_map(&mut self.conditions, other.conditions);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.current_replicas,
            other.current_replicas,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.current_revision,
            other.current_revision,
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
            &mut self.update_revision,
            other.update_revision,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.updated_replicas,
            other.updated_replicas,
        );
    }
}
