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
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
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
impl crate::Optionable for ::k8s_openapi::api::core::v1::event::Event {
    type Optioned = EventOpt;
}
#[automatically_derived]
impl crate::Optionable for EventOpt {
    type Optioned = EventOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::event::Event {
    fn into_optioned(self) -> EventOpt {
        EventOpt {
            action: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.action),
            count: <Option<i32> as crate::OptionableConvert>::into_optioned(self.count),
            event_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::into_optioned(self.event_time),
            first_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.first_timestamp),
            involved_object: Some(
                <::k8s_openapi::api::core::v1::ObjectReference as crate::OptionableConvert>::into_optioned(
                    self.involved_object,
                ),
            ),
            last_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.last_timestamp),
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
            related: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.related),
            reporting_component: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reporting_component),
            reporting_instance: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reporting_instance),
            series: <Option<
                ::k8s_openapi::api::core::v1::EventSeries,
            > as crate::OptionableConvert>::into_optioned(self.series),
            source: <Option<
                ::k8s_openapi::api::core::v1::EventSource,
            > as crate::OptionableConvert>::into_optioned(self.source),
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(value: EventOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            action: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.action)?,
            count: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.count)?,
            event_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::try_from_optioned(value.event_time)?,
            first_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.first_timestamp)?,
            involved_object: <::k8s_openapi::api::core::v1::ObjectReference as crate::OptionableConvert>::try_from_optioned(
                value
                    .involved_object
                    .ok_or(crate::optionable::Error {
                        missing_field: "involved_object",
                    })?,
            )?,
            last_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.last_timestamp)?,
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
            related: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.related)?,
            reporting_component: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.reporting_component,
            )?,
            reporting_instance: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reporting_instance)?,
            series: <Option<
                ::k8s_openapi::api::core::v1::EventSeries,
            > as crate::OptionableConvert>::try_from_optioned(value.series)?,
            source: <Option<
                ::k8s_openapi::api::core::v1::EventSource,
            > as crate::OptionableConvert>::try_from_optioned(value.source)?,
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: EventOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.action, other.action)?;
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.count, other.count)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
        > as crate::OptionableConvert>::merge(&mut self.event_time, other.event_time)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.first_timestamp,
            other.first_timestamp,
        )?;
        if let Some(other_value) = other.involved_object {
            <::k8s_openapi::api::core::v1::ObjectReference as crate::OptionableConvert>::merge(
                &mut self.involved_object,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.last_timestamp,
            other.last_timestamp,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        <Option<
            ::k8s_openapi::api::core::v1::ObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.related, other.related)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.reporting_component,
            other.reporting_component,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.reporting_instance,
            other.reporting_instance,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::EventSeries,
        > as crate::OptionableConvert>::merge(&mut self.series, other.series)?;
        <Option<
            ::k8s_openapi::api::core::v1::EventSource,
        > as crate::OptionableConvert>::merge(&mut self.source, other.source)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
