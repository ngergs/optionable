#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ObjectReference>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_schedule_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::CronJobStatus {
    type Optioned = CronJobStatusAc;
}
#[automatically_derived]
impl crate::Optionable for CronJobStatusAc {
    type Optioned = CronJobStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::CronJobStatus {
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
    fn try_from_optioned(
        value: CronJobStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
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
    fn merge(&mut self, other: CronJobStatusAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.active, other.active)?;
        crate::OptionableConvert::merge(
            &mut self.last_schedule_time,
            other.last_schedule_time,
        )?;
        crate::OptionableConvert::merge(
            &mut self.last_successful_time,
            other.last_successful_time,
        )?;
        Ok(())
    }
}
