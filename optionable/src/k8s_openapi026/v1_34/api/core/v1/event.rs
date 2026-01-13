#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EventAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_timestamp: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub involved_object: Option<
        <::k8s_openapi026::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_timestamp: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: <Option<
        ::k8s_openapi026::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_component: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_instance: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: <Option<
        ::k8s_openapi026::api::core::v1::EventSeries,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: <Option<
        ::k8s_openapi026::api::core::v1::EventSource,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::Event {
    type Optioned = EventAc;
}
#[automatically_derived]
impl crate::Optionable for EventAc {
    type Optioned = EventAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::Event {
    fn into_optioned(self) -> EventAc {
        EventAc {
            api_version: Default::default(),
            kind: Default::default(),
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
    fn try_from_optioned(value: EventAc) -> Result<Self, crate::Error> {
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
                    .ok_or(crate::Error {
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
    fn merge(&mut self, other: EventAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::Event> for EventAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::Event) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::Event, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::Event,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for EventAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::core::v1::Event as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::core::v1::Event as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::core::v1::Event as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::core::v1::Event as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::core::v1::Event as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::core::v1::Event as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for EventAc {
    type Ty = <k8s_openapi026::api::core::v1::Event as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_eventac() {
    crate::testutil::roundtrip_test::<k8s_openapi026::api::core::v1::Event>();
}
