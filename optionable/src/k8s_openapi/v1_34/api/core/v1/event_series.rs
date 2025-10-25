pub struct EventSeriesOpt {
    pub count: <Option<i32> as crate::Optionable>::Optioned,
    pub last_observed_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EventSeries {
    type Optioned = EventSeriesOpt;
}
#[automatically_derived]
impl crate::Optionable for EventSeriesOpt {
    type Optioned = EventSeriesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EventSeries {
    fn into_optioned(self) -> EventSeriesOpt {
        EventSeriesOpt {
            count: <Option<i32> as crate::OptionableConvert>::into_optioned(self.count),
            last_observed_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::into_optioned(self.last_observed_time),
        }
    }
    fn try_from_optioned(
        value: EventSeriesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            count: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.count)?,
            last_observed_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::try_from_optioned(value.last_observed_time)?,
        })
    }
    fn merge(&mut self, other: EventSeriesOpt) -> Result<(), crate::optionable::Error> {
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.count, other.count)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
        > as crate::OptionableConvert>::merge(
            &mut self.last_observed_time,
            other.last_observed_time,
        )?;
        Ok(())
    }
}
