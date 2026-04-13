#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// JobSpec describes how the job execution will look like.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JobSpecAc {
    /// Specifies the duration in seconds relative to the startTime that the job may be continuously active before the system tries to terminate it; value must be positive integer. If a Job is suspended (at creation or through an update), this timer will effectively be stopped and reset when the Job is resumed again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    /// Specifies the number of retries before marking this job failed. Defaults to 6
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_limit: Option<i32>,
    /// Specifies the limit for the number of retries within an index before marking this index as failed. When enabled the number of failures per index is kept in the pod's batch.kubernetes.io/job-index-failure-count annotation. It can only be set when Job's completionMode=Indexed, and the Pod's restart policy is Never. The field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_limit_per_index: Option<i32>,
    /// completionMode specifies how Pod completions are tracked. It can be `NonIndexed` (default) or `Indexed`.
    ///
    /// `NonIndexed` means that the Job is considered complete when there have been .spec.completions successfully completed Pods. Each Pod completion is homologous to each other.
    ///
    /// `Indexed` means that the Pods of a Job get an associated completion index from 0 to (.spec.completions - 1), available in the annotation batch.kubernetes.io/job-completion-index. The Job is considered complete when there is one successfully completed Pod for each index. When value is `Indexed`, .spec.completions must be specified and `.spec.parallelism` must be less than or equal to 10^5. In addition, The Pod name takes the form `$(job-name)-$(index)-$(random-string)`, the Pod hostname takes the form `$(job-name)-$(index)`.
    ///
    /// More completion modes can be added in the future. If the Job controller observes a mode that it doesn't recognize, which is possible during upgrades due to version skew, the controller skips updates for the Job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_mode: Option<std::string::String>,
    /// Specifies the desired number of successfully finished pods the job should be run with.  Setting to null means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions: Option<i32>,
    /// ManagedBy field indicates the controller that manages a Job. The k8s Job controller reconciles jobs which don't have this field at all or the field value is the reserved string `kubernetes.io/job-controller`, but skips reconciling Jobs with a custom value for this field. The value must be a valid domain-prefixed path (e.g. acme.io/foo) - all characters before the first "/" must be a valid subdomain as defined by RFC 1123. All characters trailing the first "/" must be valid HTTP Path characters as defined by RFC 3986. The value cannot exceed 63 characters. This field is immutable.
    ///
    /// This field is beta-level. The job controller accepts setting the field when the feature gate JobManagedBy is enabled (enabled by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<std::string::String>,
    /// manualSelector controls generation of pod labels and pod selectors. Leave `manualSelector` unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see `manualSelector=true` in jobs that were created with the old `extensions/v1beta1` API. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_selector: Option<bool>,
    /// Specifies the maximal number of failed indexes before marking the Job as failed, when backoffLimitPerIndex is set. Once the number of failed indexes exceeds this number the entire Job is marked as Failed and its execution is terminated. When left as null the job continues execution of all of its indexes and is marked with the `Complete` Job condition. It can only be specified when backoffLimitPerIndex is set. It can be null or up to completions. It is required and must be less than or equal to 10^4 when is completions greater than 10^5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failed_indexes: Option<i32>,
    /// Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) \< .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    /// Specifies the policy of handling failed pods. In particular, it allows to specify the set of actions and conditions which need to be satisfied to take the associated action. If empty, the default behaviour applies - the counter of failed pods, represented by the jobs's .status.failed field, is incremented and it is checked against the backoffLimit. This field cannot be used in combination with restartPolicy=OnFailure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_failure_policy: Option<
        <::k8s_openapi027::api::batch::v1::PodFailurePolicy as crate::Optionable>::Optioned,
    >,
    /// podReplacementPolicy specifies when to create replacement Pods. Possible values are: - TerminatingOrFailed means that we recreate pods
    ///   when they are terminating (has a metadata.deletionTimestamp) or failed.
    /// - Failed means to wait until a previously created Pod is fully terminated (has phase
    ///   Failed or Succeeded) before creating a replacement Pod.
    ///
    /// When using podFailurePolicy, Failed is the the only allowed value. TerminatingOrFailed and Failed are allowed values when podFailurePolicy is not in use. This is an beta field. To use this, enable the JobPodReplacementPolicy feature toggle. This is on by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_replacement_policy: Option<std::string::String>,
    /// A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// successPolicy specifies the policy when the Job can be declared as succeeded. If empty, the default behavior applies - the Job is declared as succeeded only when the number of succeeded pods equals to the completions. When the field is specified, it must be immutable and works only for the Indexed Jobs. Once the Job meets the SuccessPolicy, the lingering pods are terminated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_policy: Option<
        <::k8s_openapi027::api::batch::v1::SuccessPolicy as crate::Optionable>::Optioned,
    >,
    /// suspend specifies whether the Job controller should create Pods or not. If a Job is created with suspend set to true, no Pods are created by the Job controller. If a Job is suspended after creation (i.e. the flag goes from false to true), the Job controller will delete all active Pods associated with this Job. Users must design their workload to gracefully handle this. Suspending a Job will reset the StartTime field of the Job, effectively resetting the ActiveDeadlineSeconds timer too. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Describes the pod that will be created when executing a job. The only allowed template.spec.restartPolicy values are "Never" or "OnFailure". More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished execution (either Complete or Failed). If this field is set, ttlSecondsAfterFinished after the Job finishes, it is eligible to be automatically deleted. When the Job is being deleted, its lifecycle guarantees (e.g. finalizers) will be honored. If this field is unset, the Job won't be automatically deleted. If this field is set to zero, the Job becomes eligible to be deleted immediately after it finishes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds_after_finished: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::JobSpec {
    type Optioned = JobSpecAc;
}
#[automatically_derived]
impl crate::Optionable for JobSpecAc {
    type Optioned = JobSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::JobSpec {
    fn into_optioned(self) -> JobSpecAc {
        JobSpecAc {
            active_deadline_seconds: self.active_deadline_seconds,
            backoff_limit: self.backoff_limit,
            backoff_limit_per_index: self.backoff_limit_per_index,
            completion_mode: self.completion_mode,
            completions: self.completions,
            managed_by: self.managed_by,
            manual_selector: self.manual_selector,
            max_failed_indexes: self.max_failed_indexes,
            parallelism: self.parallelism,
            pod_failure_policy: crate::OptionableConvert::into_optioned(
                self.pod_failure_policy,
            ),
            pod_replacement_policy: self.pod_replacement_policy,
            selector: crate::OptionableConvert::into_optioned(self.selector),
            success_policy: crate::OptionableConvert::into_optioned(self.success_policy),
            suspend: self.suspend,
            template: Some(crate::OptionableConvert::into_optioned(self.template)),
            ttl_seconds_after_finished: self.ttl_seconds_after_finished,
        }
    }
    fn try_from_optioned(value: JobSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active_deadline_seconds: value.active_deadline_seconds,
            backoff_limit: value.backoff_limit,
            backoff_limit_per_index: value.backoff_limit_per_index,
            completion_mode: value.completion_mode,
            completions: value.completions,
            managed_by: value.managed_by,
            manual_selector: value.manual_selector,
            max_failed_indexes: value.max_failed_indexes,
            parallelism: value.parallelism,
            pod_failure_policy: crate::OptionableConvert::try_from_optioned(
                value.pod_failure_policy,
            )?,
            pod_replacement_policy: value.pod_replacement_policy,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
            success_policy: crate::OptionableConvert::try_from_optioned(
                value.success_policy,
            )?,
            suspend: value.suspend,
            template: crate::OptionableConvert::try_from_optioned(
                value
                    .template
                    .ok_or(crate::Error {
                        missing_field: "template",
                    })?,
            )?,
            ttl_seconds_after_finished: value.ttl_seconds_after_finished,
        })
    }
    fn merge(&mut self, other: JobSpecAc) -> Result<(), crate::Error> {
        if self.active_deadline_seconds.is_none() {
            self.active_deadline_seconds = crate::OptionableConvert::try_from_optioned(
                other.active_deadline_seconds,
            )?;
        } else if let Some(self_value) = self.active_deadline_seconds.as_mut()
            && let Some(other_value) = other.active_deadline_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.backoff_limit.is_none() {
            self.backoff_limit = crate::OptionableConvert::try_from_optioned(
                other.backoff_limit,
            )?;
        } else if let Some(self_value) = self.backoff_limit.as_mut()
            && let Some(other_value) = other.backoff_limit
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.backoff_limit_per_index.is_none() {
            self.backoff_limit_per_index = crate::OptionableConvert::try_from_optioned(
                other.backoff_limit_per_index,
            )?;
        } else if let Some(self_value) = self.backoff_limit_per_index.as_mut()
            && let Some(other_value) = other.backoff_limit_per_index
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.completion_mode.is_none() {
            self.completion_mode = crate::OptionableConvert::try_from_optioned(
                other.completion_mode,
            )?;
        } else if let Some(self_value) = self.completion_mode.as_mut()
            && let Some(other_value) = other.completion_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.completions.is_none() {
            self.completions = crate::OptionableConvert::try_from_optioned(
                other.completions,
            )?;
        } else if let Some(self_value) = self.completions.as_mut()
            && let Some(other_value) = other.completions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.managed_by.is_none() {
            self.managed_by = crate::OptionableConvert::try_from_optioned(
                other.managed_by,
            )?;
        } else if let Some(self_value) = self.managed_by.as_mut()
            && let Some(other_value) = other.managed_by
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.manual_selector.is_none() {
            self.manual_selector = crate::OptionableConvert::try_from_optioned(
                other.manual_selector,
            )?;
        } else if let Some(self_value) = self.manual_selector.as_mut()
            && let Some(other_value) = other.manual_selector
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.max_failed_indexes.is_none() {
            self.max_failed_indexes = crate::OptionableConvert::try_from_optioned(
                other.max_failed_indexes,
            )?;
        } else if let Some(self_value) = self.max_failed_indexes.as_mut()
            && let Some(other_value) = other.max_failed_indexes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.parallelism.is_none() {
            self.parallelism = crate::OptionableConvert::try_from_optioned(
                other.parallelism,
            )?;
        } else if let Some(self_value) = self.parallelism.as_mut()
            && let Some(other_value) = other.parallelism
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pod_failure_policy.is_none() {
            self.pod_failure_policy = crate::OptionableConvert::try_from_optioned(
                other.pod_failure_policy,
            )?;
        } else if let Some(self_value) = self.pod_failure_policy.as_mut()
            && let Some(other_value) = other.pod_failure_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pod_replacement_policy.is_none() {
            self.pod_replacement_policy = crate::OptionableConvert::try_from_optioned(
                other.pod_replacement_policy,
            )?;
        } else if let Some(self_value) = self.pod_replacement_policy.as_mut()
            && let Some(other_value) = other.pod_replacement_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.selector.is_none() {
            self.selector = crate::OptionableConvert::try_from_optioned(other.selector)?;
        } else if let Some(self_value) = self.selector.as_mut()
            && let Some(other_value) = other.selector
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.success_policy.is_none() {
            self.success_policy = crate::OptionableConvert::try_from_optioned(
                other.success_policy,
            )?;
        } else if let Some(self_value) = self.success_policy.as_mut()
            && let Some(other_value) = other.success_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.suspend.is_none() {
            self.suspend = crate::OptionableConvert::try_from_optioned(other.suspend)?;
        } else if let Some(self_value) = self.suspend.as_mut()
            && let Some(other_value) = other.suspend
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.template {
            crate::OptionableConvert::merge(&mut self.template, other_value)?;
        }
        if self.ttl_seconds_after_finished.is_none() {
            self.ttl_seconds_after_finished = crate::OptionableConvert::try_from_optioned(
                other.ttl_seconds_after_finished,
            )?;
        } else if let Some(self_value) = self.ttl_seconds_after_finished.as_mut()
            && let Some(other_value) = other.ttl_seconds_after_finished
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::JobSpec> for JobSpecAc {
    fn from_optionable(value: k8s_openapi027::api::batch::v1::JobSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::JobSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::JobSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
