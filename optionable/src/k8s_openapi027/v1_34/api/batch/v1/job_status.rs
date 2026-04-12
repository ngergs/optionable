#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// JobStatus represents the current state of a Job.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JobStatusAc {
    /// The number of pending and running pods which are not terminating (without a deletionTimestamp). The value is zero for finished jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,
    /// completedIndexes holds the completed indexes when .spec.completionMode = "Indexed" in a text format. The indexes are represented as decimal integers separated by commas. The numbers are listed in increasing order. Three or more consecutive numbers are compressed and represented by the first and last element of the series, separated by a hyphen. For example, if the completed indexes are 1, 3, 4, 5 and 7, they are represented as "1,3-5,7".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_indexes: Option<std::string::String>,
    /// Represents time when the job was completed. It is not guaranteed to be set in happens-before order across separate operations. It is represented in RFC3339 form and is in UTC. The completion time is set when the job finishes successfully, and only then. The value cannot be updated or removed. The value indicates the same or later point in time as the startTime field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// The latest available observations of an object's current state. When a Job fails, one of the conditions will have type "Failed" and status true. When a Job is suspended, one of the conditions will have type "Suspended" and status true; when the Job is resumed, the status of this condition will become false. When a Job is completed, one of the conditions will have type "Complete" and status true.
    ///
    /// A job is considered finished when it is in a terminal condition, either "Complete" or "Failed". A Job cannot have both the "Complete" and "Failed" conditions. Additionally, it cannot be in the "Complete" and "FailureTarget" conditions. The "Complete", "Failed" and "FailureTarget" conditions cannot be disabled.
    ///
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::batch::v1::JobCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// The number of pods which reached phase Failed. The value increases monotonically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    /// FailedIndexes holds the failed indexes when spec.backoffLimitPerIndex is set. The indexes are represented in the text format analogous as for the `completedIndexes` field, ie. they are kept as decimal integers separated by commas. The numbers are listed in increasing order. Three or more consecutive numbers are compressed and represented by the first and last element of the series, separated by a hyphen. For example, if the failed indexes are 1, 3, 4, 5 and 7, they are represented as "1,3-5,7". The set of failed indexes cannot overlap with the set of completed indexes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_indexes: Option<std::string::String>,
    /// The number of active pods which have a Ready condition and are not terminating (without a deletionTimestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    /// Represents time when the job controller started processing a job. When a Job is created in the suspended state, this field is not set until the first time it is resumed. This field is reset every time a Job is resumed from suspension. It is represented in RFC3339 form and is in UTC.
    ///
    /// Once set, the field can only be removed when the job is suspended. The field cannot be modified while the job is unsuspended or finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// The number of pods which reached phase Succeeded. The value increases monotonically for a given spec. However, it may decrease in reaction to scale down of elastic indexed jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
    /// The number of pods which are terminating (in phase Pending or Running and have a deletionTimestamp).
    ///
    /// This field is beta-level. The job controller populates the field when the feature gate JobPodReplacementPolicy is enabled (enabled by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating: Option<i32>,
    /// uncountedTerminatedPods holds the UIDs of Pods that have terminated but the job controller hasn't yet accounted for in the status counters.
    ///
    /// The job controller creates pods with a finalizer. When a pod terminates (succeeded or failed), the controller does three steps to account for it in the job status:
    ///
    /// 1. Add the pod UID to the arrays in this field. 2. Remove the pod finalizer. 3. Remove the pod UID from the arrays while increasing the corresponding
    ///     counter.
    ///
    /// Old jobs might not be tracked using this field, in which case the field remains null. The structure is empty for finished jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncounted_terminated_pods: Option<
        <::k8s_openapi027::api::batch::v1::UncountedTerminatedPods as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::JobStatus {
    type Optioned = JobStatusAc;
}
#[automatically_derived]
impl crate::Optionable for JobStatusAc {
    type Optioned = JobStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::JobStatus {
    fn into_optioned(self) -> JobStatusAc {
        JobStatusAc {
            active: self.active,
            completed_indexes: self.completed_indexes,
            completion_time: crate::OptionableConvert::into_optioned(
                self.completion_time,
            ),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            failed: self.failed,
            failed_indexes: self.failed_indexes,
            ready: self.ready,
            start_time: crate::OptionableConvert::into_optioned(self.start_time),
            succeeded: self.succeeded,
            terminating: self.terminating,
            uncounted_terminated_pods: crate::OptionableConvert::into_optioned(
                self.uncounted_terminated_pods,
            ),
        }
    }
    fn try_from_optioned(value: JobStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active: value.active,
            completed_indexes: value.completed_indexes,
            completion_time: crate::OptionableConvert::try_from_optioned(
                value.completion_time,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            failed: value.failed,
            failed_indexes: value.failed_indexes,
            ready: value.ready,
            start_time: crate::OptionableConvert::try_from_optioned(value.start_time)?,
            succeeded: value.succeeded,
            terminating: value.terminating,
            uncounted_terminated_pods: crate::OptionableConvert::try_from_optioned(
                value.uncounted_terminated_pods,
            )?,
        })
    }
    fn merge(&mut self, other: JobStatusAc) -> Result<(), crate::Error> {
        if self.active.is_none() {
            self.active = crate::OptionableConvert::try_from_optioned(other.active)?;
        } else if let Some(self_value) = self.active.as_mut()
            && let Some(other_value) = other.active
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.completed_indexes.is_none() {
            self.completed_indexes = crate::OptionableConvert::try_from_optioned(
                other.completed_indexes,
            )?;
        } else if let Some(self_value) = self.completed_indexes.as_mut()
            && let Some(other_value) = other.completed_indexes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.completion_time.is_none() {
            self.completion_time = crate::OptionableConvert::try_from_optioned(
                other.completion_time,
            )?;
        } else if let Some(self_value) = self.completion_time.as_mut()
            && let Some(other_value) = other.completion_time
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
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.failed.is_none() {
            self.failed = crate::OptionableConvert::try_from_optioned(other.failed)?;
        } else if let Some(self_value) = self.failed.as_mut()
            && let Some(other_value) = other.failed
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.failed_indexes.is_none() {
            self.failed_indexes = crate::OptionableConvert::try_from_optioned(
                other.failed_indexes,
            )?;
        } else if let Some(self_value) = self.failed_indexes.as_mut()
            && let Some(other_value) = other.failed_indexes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.ready.is_none() {
            self.ready = crate::OptionableConvert::try_from_optioned(other.ready)?;
        } else if let Some(self_value) = self.ready.as_mut()
            && let Some(other_value) = other.ready
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.start_time.is_none() {
            self.start_time = crate::OptionableConvert::try_from_optioned(
                other.start_time,
            )?;
        } else if let Some(self_value) = self.start_time.as_mut()
            && let Some(other_value) = other.start_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.succeeded.is_none() {
            self.succeeded = crate::OptionableConvert::try_from_optioned(
                other.succeeded,
            )?;
        } else if let Some(self_value) = self.succeeded.as_mut()
            && let Some(other_value) = other.succeeded
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.terminating.is_none() {
            self.terminating = crate::OptionableConvert::try_from_optioned(
                other.terminating,
            )?;
        } else if let Some(self_value) = self.terminating.as_mut()
            && let Some(other_value) = other.terminating
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.uncounted_terminated_pods.is_none() {
            self.uncounted_terminated_pods = crate::OptionableConvert::try_from_optioned(
                other.uncounted_terminated_pods,
            )?;
        } else if let Some(self_value) = self.uncounted_terminated_pods.as_mut()
            && let Some(other_value) = other.uncounted_terminated_pods
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::JobStatus> for JobStatusAc {
    fn from_optionable(value: k8s_openapi027::api::batch::v1::JobStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::JobStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::JobStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
