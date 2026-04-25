#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CronJobStatus represents the current state of a cron job.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CronJobStatusAc {
    /// A list of pointers to currently running jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
        >,
    >,
    /// Information when was the last time the job was successfully scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// Information when was the last time the job successfully completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::batch::v1::CronJobStatus {
    type Optioned = CronJobStatusAc;
}
#[automatically_derived]
impl crate::Optionable for CronJobStatusAc {
    type Optioned = CronJobStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::batch::v1::CronJobStatus {
    fn into_optioned(self) -> CronJobStatusAc {
        CronJobStatusAc {
            active: crate::OptionableConvert::into_optioned(self.active),
            last_schedule_time: crate::OptionableConvert::into_optioned(
                self.last_schedule_time,
            ),
            last_successful_time: crate::OptionableConvert::into_optioned(
                self.last_successful_time,
            ),
        }
    }
    fn try_from_optioned(value: CronJobStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active: crate::OptionableConvert::try_from_optioned(value.active)?,
            last_schedule_time: crate::OptionableConvert::try_from_optioned(
                value.last_schedule_time,
            )?,
            last_successful_time: crate::OptionableConvert::try_from_optioned(
                value.last_successful_time,
            )?,
        })
    }
    fn merge(&mut self, other: CronJobStatusAc) -> Result<(), crate::Error> {
        if self.active.is_none() {
            self.active = crate::OptionableConvert::try_from_optioned(other.active)?;
        } else if let Some(self_value) = self.active.as_mut()
            && let Some(other_value) = other.active
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.last_schedule_time.is_none() {
            self.last_schedule_time = crate::OptionableConvert::try_from_optioned(
                other.last_schedule_time,
            )?;
        } else if let Some(self_value) = self.last_schedule_time.as_mut()
            && let Some(other_value) = other.last_schedule_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.last_successful_time.is_none() {
            self.last_successful_time = crate::OptionableConvert::try_from_optioned(
                other.last_successful_time,
            )?;
        } else if let Some(self_value) = self.last_successful_time.as_mut()
            && let Some(other_value) = other.last_successful_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::batch::v1::CronJobStatus>
for CronJobStatusAc {
    fn from_optionable(value: k8s_openapi027::api::batch::v1::CronJobStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::batch::v1::CronJobStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::batch::v1::CronJobStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for CronJobStatusAc {
    fn merge_from(&mut self, other: Self) {
        self.active = other.active;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.last_schedule_time,
            other.last_schedule_time,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.last_successful_time,
            other.last_successful_time,
        );
    }
}
