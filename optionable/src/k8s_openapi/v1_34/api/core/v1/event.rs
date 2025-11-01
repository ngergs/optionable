pub struct EventOpt {
    pub action: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub count: <Option<i32> as crate::Optionable>::Optioned,
    pub event_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    pub first_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub involved_object: Option<
        <::k8s_openapi::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    pub last_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub related: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    pub reporting_component: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub reporting_instance: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub series: <Option<
        ::k8s_openapi::api::core::v1::EventSeries,
    > as crate::Optionable>::Optioned,
    pub source: <Option<
        ::k8s_openapi::api::core::v1::EventSource,
    > as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Event {
    type Optioned = EventOpt;
}
#[automatically_derived]
impl crate::Optionable for EventOpt {
    type Optioned = EventOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Event {
    fn into_optioned(self) -> EventOpt {
        EventOpt {
            action: crate::OptionableConvert::into_optioned(self.action),
            count: crate::OptionableConvert::into_optioned(self.count),
            event_time: crate::OptionableConvert::into_optioned(self.event_time),
            first_timestamp: crate::OptionableConvert::into_optioned(
                self.first_timestamp,
            ),
            involved_object: Some(
                crate::OptionableConvert::into_optioned(self.involved_object),
            ),
            last_timestamp: crate::OptionableConvert::into_optioned(self.last_timestamp),
            message: crate::OptionableConvert::into_optioned(self.message),
            metadata: self.metadata,
            reason: crate::OptionableConvert::into_optioned(self.reason),
            related: crate::OptionableConvert::into_optioned(self.related),
            reporting_component: crate::OptionableConvert::into_optioned(
                self.reporting_component,
            ),
            reporting_instance: crate::OptionableConvert::into_optioned(
                self.reporting_instance,
            ),
            series: crate::OptionableConvert::into_optioned(self.series),
            source: crate::OptionableConvert::into_optioned(self.source),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(value: EventOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            action: crate::OptionableConvert::try_from_optioned(value.action)?,
            count: crate::OptionableConvert::try_from_optioned(value.count)?,
            event_time: crate::OptionableConvert::try_from_optioned(value.event_time)?,
            first_timestamp: crate::OptionableConvert::try_from_optioned(
                value.first_timestamp,
            )?,
            involved_object: crate::OptionableConvert::try_from_optioned(
                value
                    .involved_object
                    .ok_or(crate::optionable::Error {
                        missing_field: "involved_object",
                    })?,
            )?,
            last_timestamp: crate::OptionableConvert::try_from_optioned(
                value.last_timestamp,
            )?,
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            metadata: value.metadata,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
            related: crate::OptionableConvert::try_from_optioned(value.related)?,
            reporting_component: crate::OptionableConvert::try_from_optioned(
                value.reporting_component,
            )?,
            reporting_instance: crate::OptionableConvert::try_from_optioned(
                value.reporting_instance,
            )?,
            series: crate::OptionableConvert::try_from_optioned(value.series)?,
            source: crate::OptionableConvert::try_from_optioned(value.source)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: EventOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.action, other.action)?;
        crate::OptionableConvert::merge(&mut self.count, other.count)?;
        crate::OptionableConvert::merge(&mut self.event_time, other.event_time)?;
        crate::OptionableConvert::merge(
            &mut self.first_timestamp,
            other.first_timestamp,
        )?;
        if let Some(other_value) = other.involved_object {
            crate::OptionableConvert::merge(&mut self.involved_object, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.last_timestamp, other.last_timestamp)?;
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        crate::OptionableConvert::merge(&mut self.related, other.related)?;
        crate::OptionableConvert::merge(
            &mut self.reporting_component,
            other.reporting_component,
        )?;
        crate::OptionableConvert::merge(
            &mut self.reporting_instance,
            other.reporting_instance,
        )?;
        crate::OptionableConvert::merge(&mut self.series, other.series)?;
        crate::OptionableConvert::merge(&mut self.source, other.source)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
