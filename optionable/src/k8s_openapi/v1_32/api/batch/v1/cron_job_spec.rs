#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CronJobSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_jobs_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<
        <::k8s_openapi::api::batch::v1::JobTemplateSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_deadline_seconds: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_jobs_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::CronJobSpec {
    type Optioned = CronJobSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CronJobSpecAc {
    type Optioned = CronJobSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::CronJobSpec {
    fn into_optioned(self) -> CronJobSpecAc {
        CronJobSpecAc {
            concurrency_policy: crate::OptionableConvert::into_optioned(
                self.concurrency_policy,
            ),
            failed_jobs_history_limit: crate::OptionableConvert::into_optioned(
                self.failed_jobs_history_limit,
            ),
            job_template: Some(
                crate::OptionableConvert::into_optioned(self.job_template),
            ),
            schedule: Some(crate::OptionableConvert::into_optioned(self.schedule)),
            starting_deadline_seconds: crate::OptionableConvert::into_optioned(
                self.starting_deadline_seconds,
            ),
            successful_jobs_history_limit: crate::OptionableConvert::into_optioned(
                self.successful_jobs_history_limit,
            ),
            suspend: crate::OptionableConvert::into_optioned(self.suspend),
            time_zone: crate::OptionableConvert::into_optioned(self.time_zone),
        }
    }
    fn try_from_optioned(value: CronJobSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            concurrency_policy: crate::OptionableConvert::try_from_optioned(
                value.concurrency_policy,
            )?,
            failed_jobs_history_limit: crate::OptionableConvert::try_from_optioned(
                value.failed_jobs_history_limit,
            )?,
            job_template: crate::OptionableConvert::try_from_optioned(
                value
                    .job_template
                    .ok_or(crate::Error {
                        missing_field: "job_template",
                    })?,
            )?,
            schedule: crate::OptionableConvert::try_from_optioned(
                value
                    .schedule
                    .ok_or(crate::Error {
                        missing_field: "schedule",
                    })?,
            )?,
            starting_deadline_seconds: crate::OptionableConvert::try_from_optioned(
                value.starting_deadline_seconds,
            )?,
            successful_jobs_history_limit: crate::OptionableConvert::try_from_optioned(
                value.successful_jobs_history_limit,
            )?,
            suspend: crate::OptionableConvert::try_from_optioned(value.suspend)?,
            time_zone: crate::OptionableConvert::try_from_optioned(value.time_zone)?,
        })
    }
    fn merge(&mut self, other: CronJobSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.concurrency_policy,
            other.concurrency_policy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.failed_jobs_history_limit,
            other.failed_jobs_history_limit,
        )?;
        if let Some(other_value) = other.job_template {
            crate::OptionableConvert::merge(&mut self.job_template, other_value)?;
        }
        if let Some(other_value) = other.schedule {
            crate::OptionableConvert::merge(&mut self.schedule, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.starting_deadline_seconds,
            other.starting_deadline_seconds,
        )?;
        crate::OptionableConvert::merge(
            &mut self.successful_jobs_history_limit,
            other.successful_jobs_history_limit,
        )?;
        crate::OptionableConvert::merge(&mut self.suspend, other.suspend)?;
        crate::OptionableConvert::merge(&mut self.time_zone, other.time_zone)?;
        Ok(())
    }
}
