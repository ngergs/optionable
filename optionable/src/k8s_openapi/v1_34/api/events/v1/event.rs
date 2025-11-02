#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = Event)]
pub struct EventAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_count: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_first_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_last_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_source: <Option<
        ::k8s_openapi::api::core::v1::EventSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regarding: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_controller: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_instance: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: <Option<
        ::k8s_openapi::api::events::v1::EventSeries,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::events::v1::Event {
    type Optioned = EventAc;
}
#[automatically_derived]
impl crate::Optionable for EventAc {
    type Optioned = EventAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::events::v1::Event {
    fn into_optioned(self) -> EventAc {
        EventAc {
            action: crate::OptionableConvert::into_optioned(self.action),
            deprecated_count: crate::OptionableConvert::into_optioned(
                self.deprecated_count,
            ),
            deprecated_first_timestamp: crate::OptionableConvert::into_optioned(
                self.deprecated_first_timestamp,
            ),
            deprecated_last_timestamp: crate::OptionableConvert::into_optioned(
                self.deprecated_last_timestamp,
            ),
            deprecated_source: crate::OptionableConvert::into_optioned(
                self.deprecated_source,
            ),
            event_time: crate::OptionableConvert::into_optioned(self.event_time),
            metadata: self.metadata,
            note: crate::OptionableConvert::into_optioned(self.note),
            reason: crate::OptionableConvert::into_optioned(self.reason),
            regarding: crate::OptionableConvert::into_optioned(self.regarding),
            related: crate::OptionableConvert::into_optioned(self.related),
            reporting_controller: crate::OptionableConvert::into_optioned(
                self.reporting_controller,
            ),
            reporting_instance: crate::OptionableConvert::into_optioned(
                self.reporting_instance,
            ),
            series: crate::OptionableConvert::into_optioned(self.series),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(value: EventAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            action: crate::OptionableConvert::try_from_optioned(value.action)?,
            deprecated_count: crate::OptionableConvert::try_from_optioned(
                value.deprecated_count,
            )?,
            deprecated_first_timestamp: crate::OptionableConvert::try_from_optioned(
                value.deprecated_first_timestamp,
            )?,
            deprecated_last_timestamp: crate::OptionableConvert::try_from_optioned(
                value.deprecated_last_timestamp,
            )?,
            deprecated_source: crate::OptionableConvert::try_from_optioned(
                value.deprecated_source,
            )?,
            event_time: crate::OptionableConvert::try_from_optioned(value.event_time)?,
            metadata: value.metadata,
            note: crate::OptionableConvert::try_from_optioned(value.note)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            regarding: crate::OptionableConvert::try_from_optioned(value.regarding)?,
            related: crate::OptionableConvert::try_from_optioned(value.related)?,
            reporting_controller: crate::OptionableConvert::try_from_optioned(
                value.reporting_controller,
            )?,
            reporting_instance: crate::OptionableConvert::try_from_optioned(
                value.reporting_instance,
            )?,
            series: crate::OptionableConvert::try_from_optioned(value.series)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: EventAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.action, other.action)?;
        crate::OptionableConvert::merge(
            &mut self.deprecated_count,
            other.deprecated_count,
        )?;
        crate::OptionableConvert::merge(
            &mut self.deprecated_first_timestamp,
            other.deprecated_first_timestamp,
        )?;
        crate::OptionableConvert::merge(
            &mut self.deprecated_last_timestamp,
            other.deprecated_last_timestamp,
        )?;
        crate::OptionableConvert::merge(
            &mut self.deprecated_source,
            other.deprecated_source,
        )?;
        crate::OptionableConvert::merge(&mut self.event_time, other.event_time)?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.note, other.note)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        crate::OptionableConvert::merge(&mut self.regarding, other.regarding)?;
        crate::OptionableConvert::merge(&mut self.related, other.related)?;
        crate::OptionableConvert::merge(
            &mut self.reporting_controller,
            other.reporting_controller,
        )?;
        crate::OptionableConvert::merge(
            &mut self.reporting_instance,
            other.reporting_instance,
        )?;
        crate::OptionableConvert::merge(&mut self.series, other.series)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::events::v1::Event;
