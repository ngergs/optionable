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
    pub conditions: Option<
        std::vec::Vec<::k8s_openapi027::api::apps::v1::StatefulSetCondition>,
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
            conditions: self.conditions,
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
            conditions: value.conditions,
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
        self.available_replicas = other.available_replicas;
        self.collision_count = other.collision_count;
        self.conditions = other.conditions;
        self.current_replicas = other.current_replicas;
        self.current_revision = other.current_revision;
        self.observed_generation = other.observed_generation;
        self.ready_replicas = other.ready_replicas;
        if let Some(other_value) = other.replicas {
            self.replicas = other_value;
        }
        self.update_revision = other.update_revision;
        self.updated_replicas = other.updated_replicas;
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
