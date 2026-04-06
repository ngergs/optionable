#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EventSeries contain information on series of events, i.e. thing that was/is happening continuously for some time. How often to update the EventSeries is up to the event reporters. The default event reporter in "k8s.io/client-go/tools/events/event_broadcaster.go" shows how this struct is updated on heartbeats and can guide customized reporter implementations.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EventSeriesAc {
    /// count is the number of occurrences in this series up to the last heartbeat time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// lastObservedTime is the time when last Event from the series was seen before last heartbeat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::events::v1::EventSeries {
    type Optioned = EventSeriesAc;
}
#[automatically_derived]
impl crate::Optionable for EventSeriesAc {
    type Optioned = EventSeriesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::events::v1::EventSeries {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::events::v1::EventSeries>
for EventSeriesAc {
    fn from_optionable(value: k8s_openapi027::api::events::v1::EventSeries) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::events::v1::EventSeries, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::events::v1::EventSeries,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
