#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CronJobSpec describes how the job execution will look like and when it will actually run.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CronJobSpecAc {
    /// Specifies how to treat concurrent executions of a Job. Valid values are:
    ///
    /// - "Allow" (default): allows CronJobs to run concurrently; - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - "Replace": cancels currently running job and replaces it with a new one
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_policy: Option<std::string::String>,
    /// The number of failed finished jobs to retain. Value must be non-negative integer. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: Option<i32>,
    /// Specifies the job that will be created when executing a CronJob.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<
        <::k8s_openapi027::api::batch::v1::JobTemplateSpec as crate::Optionable>::Optioned,
    >,
    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<std::string::String>,
    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_deadline_seconds: Option<i64>,
    /// The number of successful finished jobs to retain. Value must be non-negative integer. Defaults to 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: Option<i32>,
    /// This flag tells the controller to suspend subsequent executions, it does not apply to already started executions.  Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// The time zone name for the given schedule, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. If not specified, this will default to the time zone of the kube-controller-manager process. The set of valid time zone names and the time zone offset is loaded from the system-wide time zone database by the API server during CronJob validation and the controller manager during execution. If no system-wide time zone database can be found a bundled version of the database is used instead. If the time zone name becomes invalid during the lifetime of a CronJob or due to a change in host configuration, the controller will stop creating new new Jobs and will create a system event with the reason UnknownTimeZone. More information can be found in https://kubernetes.io/docs/concepts/workloads/controllers/cron-jobs/#time-zones
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::CronJobSpec {
    type Optioned = CronJobSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CronJobSpecAc {
    type Optioned = CronJobSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::CronJobSpec {
    fn into_optioned(self) -> CronJobSpecAc {
        CronJobSpecAc {
            concurrency_policy: self.concurrency_policy,
            failed_jobs_history_limit: self.failed_jobs_history_limit,
            job_template: Some(
                crate::OptionableConvert::into_optioned(self.job_template),
            ),
            schedule: Some(self.schedule),
            starting_deadline_seconds: self.starting_deadline_seconds,
            successful_jobs_history_limit: self.successful_jobs_history_limit,
            suspend: self.suspend,
            time_zone: self.time_zone,
        }
    }
    fn try_from_optioned(value: CronJobSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            concurrency_policy: value.concurrency_policy,
            failed_jobs_history_limit: value.failed_jobs_history_limit,
            job_template: crate::OptionableConvert::try_from_optioned(
                value
                    .job_template
                    .ok_or(crate::Error {
                        missing_field: "job_template",
                    })?,
            )?,
            schedule: value
                .schedule
                .ok_or(crate::Error {
                    missing_field: "schedule",
                })?,
            starting_deadline_seconds: value.starting_deadline_seconds,
            successful_jobs_history_limit: value.successful_jobs_history_limit,
            suspend: value.suspend,
            time_zone: value.time_zone,
        })
    }
    fn merge(&mut self, other: CronJobSpecAc) -> Result<(), crate::Error> {
        if self.concurrency_policy.is_none() {
            self.concurrency_policy = crate::OptionableConvert::try_from_optioned(
                other.concurrency_policy,
            )?;
        } else if let Some(self_value) = self.concurrency_policy.as_mut()
            && let Some(other_value) = other.concurrency_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.failed_jobs_history_limit.is_none() {
            self.failed_jobs_history_limit = crate::OptionableConvert::try_from_optioned(
                other.failed_jobs_history_limit,
            )?;
        } else if let Some(self_value) = self.failed_jobs_history_limit.as_mut()
            && let Some(other_value) = other.failed_jobs_history_limit
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.job_template {
            self.job_template = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.schedule {
            self.schedule = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.starting_deadline_seconds.is_none() {
            self.starting_deadline_seconds = crate::OptionableConvert::try_from_optioned(
                other.starting_deadline_seconds,
            )?;
        } else if let Some(self_value) = self.starting_deadline_seconds.as_mut()
            && let Some(other_value) = other.starting_deadline_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.successful_jobs_history_limit.is_none() {
            self.successful_jobs_history_limit = crate::OptionableConvert::try_from_optioned(
                other.successful_jobs_history_limit,
            )?;
        } else if let Some(self_value) = self.successful_jobs_history_limit.as_mut()
            && let Some(other_value) = other.successful_jobs_history_limit
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
        if self.time_zone.is_none() {
            self.time_zone = crate::OptionableConvert::try_from_optioned(
                other.time_zone,
            )?;
        } else if let Some(self_value) = self.time_zone.as_mut()
            && let Some(other_value) = other.time_zone
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::CronJobSpec>
for CronJobSpecAc {
    fn from_optionable(value: k8s_openapi027::api::batch::v1::CronJobSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::CronJobSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::CronJobSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
