#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DaemonSetStatus represents the current status of a daemon set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DaemonSetStatusAc {
    /// Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_count: Option<i32>,
    /// Represents the latest available observations of a DaemonSet's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::apps::v1::DaemonSetCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_number_scheduled: Option<i32>,
    /// The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_number_scheduled: Option<i32>,
    /// The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_available: Option<i32>,
    /// The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_misscheduled: Option<i32>,
    /// numberReady is the number of nodes that should be running the daemon pod and have one or more of the daemon pod running with a Ready Condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_ready: Option<i32>,
    /// The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_unavailable: Option<i32>,
    /// The most recent generation observed by the daemon set controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// The total number of nodes that are running updated daemon pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_number_scheduled: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::DaemonSetStatus {
    type Optioned = DaemonSetStatusAc;
}
#[automatically_derived]
impl crate::Optionable for DaemonSetStatusAc {
    type Optioned = DaemonSetStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::DaemonSetStatus {
    fn into_optioned(self) -> DaemonSetStatusAc {
        DaemonSetStatusAc {
            collision_count: self.collision_count,
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_number_scheduled: Some(self.current_number_scheduled),
            desired_number_scheduled: Some(self.desired_number_scheduled),
            number_available: self.number_available,
            number_misscheduled: Some(self.number_misscheduled),
            number_ready: Some(self.number_ready),
            number_unavailable: self.number_unavailable,
            observed_generation: self.observed_generation,
            updated_number_scheduled: self.updated_number_scheduled,
        }
    }
    fn try_from_optioned(value: DaemonSetStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            collision_count: value.collision_count,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_number_scheduled: value
                .current_number_scheduled
                .ok_or(crate::Error {
                    missing_field: "current_number_scheduled",
                })?,
            desired_number_scheduled: value
                .desired_number_scheduled
                .ok_or(crate::Error {
                    missing_field: "desired_number_scheduled",
                })?,
            number_available: value.number_available,
            number_misscheduled: value
                .number_misscheduled
                .ok_or(crate::Error {
                    missing_field: "number_misscheduled",
                })?,
            number_ready: value
                .number_ready
                .ok_or(crate::Error {
                    missing_field: "number_ready",
                })?,
            number_unavailable: value.number_unavailable,
            observed_generation: value.observed_generation,
            updated_number_scheduled: value.updated_number_scheduled,
        })
    }
    fn merge(&mut self, other: DaemonSetStatusAc) -> Result<(), crate::Error> {
        if self.collision_count.is_none() {
            self.collision_count = crate::OptionableConvert::try_from_optioned(
                other.collision_count,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.collision_count,
                other.collision_count,
            )?;
        }
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else {
            crate::merge::try_merge_optioned_map(
                &mut self.conditions,
                other.conditions,
            )?;
        }
        if let Some(other_value) = other.current_number_scheduled {
            self.current_number_scheduled = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.desired_number_scheduled {
            self.desired_number_scheduled = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.number_available.is_none() {
            self.number_available = crate::OptionableConvert::try_from_optioned(
                other.number_available,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.number_available,
                other.number_available,
            )?;
        }
        if let Some(other_value) = other.number_misscheduled {
            self.number_misscheduled = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.number_ready {
            self.number_ready = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.number_unavailable.is_none() {
            self.number_unavailable = crate::OptionableConvert::try_from_optioned(
                other.number_unavailable,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.number_unavailable,
                other.number_unavailable,
            )?;
        }
        if self.observed_generation.is_none() {
            self.observed_generation = crate::OptionableConvert::try_from_optioned(
                other.observed_generation,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.observed_generation,
                other.observed_generation,
            )?;
        }
        if self.updated_number_scheduled.is_none() {
            self.updated_number_scheduled = crate::OptionableConvert::try_from_optioned(
                other.updated_number_scheduled,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.updated_number_scheduled,
                other.updated_number_scheduled,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::DaemonSetStatus>
for DaemonSetStatusAc {
    fn from_optionable(value: k8s_openapi027::api::apps::v1::DaemonSetStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::DaemonSetStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::DaemonSetStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
