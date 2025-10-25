pub struct EventOpt {
    pub action: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub deprecated_count: <Option<i32> as crate::Optionable>::Optioned,
    pub deprecated_first_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub deprecated_last_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub deprecated_source: <Option<
        ::k8s_openapi::api::core::v1::EventSource,
    > as crate::Optionable>::Optioned,
    pub event_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub note: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub regarding: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    pub related: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    pub reporting_controller: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub reporting_instance: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub series: <Option<
        ::k8s_openapi::api::events::v1::EventSeries,
    > as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::events::v1::event::Event {
    type Optioned = EventOpt;
}
#[automatically_derived]
impl crate::Optionable for EventOpt {
    type Optioned = EventOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::events::v1::event::Event {
    fn into_optioned(self) -> EventOpt {
        EventOpt {
            action: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.action),
            deprecated_count: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.deprecated_count),
            deprecated_first_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(
                self.deprecated_first_timestamp,
            ),
            deprecated_last_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(
                self.deprecated_last_timestamp,
            ),
            deprecated_source: <Option<
                ::k8s_openapi::api::core::v1::EventSource,
            > as crate::OptionableConvert>::into_optioned(self.deprecated_source),
            event_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::into_optioned(self.event_time),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            note: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.note),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
            regarding: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.regarding),
            related: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.related),
            reporting_controller: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reporting_controller),
            reporting_instance: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reporting_instance),
            series: <Option<
                ::k8s_openapi::api::events::v1::EventSeries,
            > as crate::OptionableConvert>::into_optioned(self.series),
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
            deprecated_count: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.deprecated_count)?,
            deprecated_first_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(
                value.deprecated_first_timestamp,
            )?,
            deprecated_last_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(
                value.deprecated_last_timestamp,
            )?,
            deprecated_source: <Option<
                ::k8s_openapi::api::core::v1::EventSource,
            > as crate::OptionableConvert>::try_from_optioned(value.deprecated_source)?,
            event_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::try_from_optioned(value.event_time)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            note: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.note)?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
            regarding: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.regarding)?,
            related: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.related)?,
            reporting_controller: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.reporting_controller,
            )?,
            reporting_instance: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reporting_instance)?,
            series: <Option<
                ::k8s_openapi::api::events::v1::EventSeries,
            > as crate::OptionableConvert>::try_from_optioned(value.series)?,
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: EventOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.action, other.action)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.deprecated_count,
            other.deprecated_count,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.deprecated_first_timestamp,
            other.deprecated_first_timestamp,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.deprecated_last_timestamp,
            other.deprecated_last_timestamp,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::EventSource,
        > as crate::OptionableConvert>::merge(
            &mut self.deprecated_source,
            other.deprecated_source,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
        > as crate::OptionableConvert>::merge(&mut self.event_time, other.event_time)?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.note, other.note)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        <Option<
            ::k8s_openapi::api::core::v1::ObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.regarding, other.regarding)?;
        <Option<
            ::k8s_openapi::api::core::v1::ObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.related, other.related)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.reporting_controller,
            other.reporting_controller,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.reporting_instance,
            other.reporting_instance,
        )?;
        <Option<
            ::k8s_openapi::api::events::v1::EventSeries,
        > as crate::OptionableConvert>::merge(&mut self.series, other.series)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
