#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct EventSeriesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_time: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::events::v1::EventSeries {
    type Optioned = EventSeriesAc;
}
#[automatically_derived]
impl crate::Optionable for EventSeriesAc {
    type Optioned = EventSeriesAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::events::v1::EventSeries {
    fn into_optioned(self) -> EventSeriesAc {
        EventSeriesAc {
            count: Some(self.count),
            last_observed_time: Some(
                crate::OptionableConvert::into_optioned(self.last_observed_time),
            ),
        }
    }
    fn try_from_optioned(value: EventSeriesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            count: value
                .count
                .ok_or(crate::Error {
                    missing_field: "count",
                })?,
            last_observed_time: crate::OptionableConvert::try_from_optioned(
                value
                    .last_observed_time
                    .ok_or(crate::Error {
                        missing_field: "last_observed_time",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: EventSeriesAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.count {
            self.count = other_value;
        }
        if let Some(other_value) = other.last_observed_time {
            crate::OptionableConvert::merge(&mut self.last_observed_time, other_value)?;
        }
        Ok(())
    }
}
