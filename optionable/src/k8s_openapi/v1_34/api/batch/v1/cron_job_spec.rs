pub struct CronJobSpecOpt {
    pub concurrency_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub failed_jobs_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    pub job_template: Option<
        <::k8s_openapi::api::batch::v1::JobTemplateSpec as crate::Optionable>::Optioned,
    >,
    pub schedule: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub starting_deadline_seconds: <Option<i64> as crate::Optionable>::Optioned,
    pub successful_jobs_history_limit: <Option<i32> as crate::Optionable>::Optioned,
    pub suspend: <Option<bool> as crate::Optionable>::Optioned,
    pub time_zone: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::cron_job_spec::CronJobSpec {
    type Optioned = CronJobSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for CronJobSpecOpt {
    type Optioned = CronJobSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::batch::v1::cron_job_spec::CronJobSpec {
    fn into_optioned(self) -> CronJobSpecOpt {
        CronJobSpecOpt {
            concurrency_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.concurrency_policy),
            failed_jobs_history_limit: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(
                self.failed_jobs_history_limit,
            ),
            job_template: Some(
                <::k8s_openapi::api::batch::v1::JobTemplateSpec as crate::OptionableConvert>::into_optioned(
                    self.job_template,
                ),
            ),
            schedule: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.schedule,
                ),
            ),
            starting_deadline_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(
                self.starting_deadline_seconds,
            ),
            successful_jobs_history_limit: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(
                self.successful_jobs_history_limit,
            ),
            suspend: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.suspend),
            time_zone: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.time_zone),
        }
    }
    fn try_from_optioned(
        value: CronJobSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            concurrency_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.concurrency_policy)?,
            failed_jobs_history_limit: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.failed_jobs_history_limit,
            )?,
            job_template: <::k8s_openapi::api::batch::v1::JobTemplateSpec as crate::OptionableConvert>::try_from_optioned(
                value
                    .job_template
                    .ok_or(crate::optionable::Error {
                        missing_field: "job_template",
                    })?,
            )?,
            schedule: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .schedule
                    .ok_or(crate::optionable::Error {
                        missing_field: "schedule",
                    })?,
            )?,
            starting_deadline_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.starting_deadline_seconds,
            )?,
            successful_jobs_history_limit: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.successful_jobs_history_limit,
            )?,
            suspend: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.suspend)?,
            time_zone: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.time_zone)?,
        })
    }
    fn merge(&mut self, other: CronJobSpecOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.concurrency_policy,
            other.concurrency_policy,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.failed_jobs_history_limit,
            other.failed_jobs_history_limit,
        )?;
        if let Some(other_value) = other.job_template {
            <::k8s_openapi::api::batch::v1::JobTemplateSpec as crate::OptionableConvert>::merge(
                &mut self.job_template,
                other_value,
            )?;
        }
        if let Some(other_value) = other.schedule {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.schedule,
                other_value,
            )?;
        }
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.starting_deadline_seconds,
            other.starting_deadline_seconds,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.successful_jobs_history_limit,
            other.successful_jobs_history_limit,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.suspend, other.suspend)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.time_zone, other.time_zone)?;
        Ok(())
    }
}
