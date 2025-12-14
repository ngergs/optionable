#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EventSeriesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EventSeries {
    type Optioned = EventSeriesAc;
}
#[automatically_derived]
impl crate::Optionable for EventSeriesAc {
    type Optioned = EventSeriesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EventSeries {
    fn into_optioned(self) -> EventSeriesAc {
        EventSeriesAc {
            count: crate::OptionableConvert::into_optioned(self.count),
            last_observed_time: crate::OptionableConvert::into_optioned(
                self.last_observed_time,
            ),
        }
    }
    fn try_from_optioned(value: EventSeriesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            count: crate::OptionableConvert::try_from_optioned(value.count)?,
            last_observed_time: crate::OptionableConvert::try_from_optioned(
                value.last_observed_time,
            )?,
        })
    }
    fn merge(&mut self, other: EventSeriesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.count, other.count)?;
        crate::OptionableConvert::merge(
            &mut self.last_observed_time,
            other.last_observed_time,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::EventSeries>
for EventSeriesAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::EventSeries) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::EventSeries, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::EventSeries,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
