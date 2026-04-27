#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Event is a report of an event somewhere in the cluster.  Events have a limited retention time and triggers and messages may evolve with time.  Event consumers should not rely on the timing of an event with a given Reason reflecting a consistent underlying trigger, or the continued existence of events with that Reason.  Events should be treated as informative, best-effort, supplemental data.
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
    /// What action was taken/failed regarding to the Regarding object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<std::string::String>,
    /// The number of times this event has occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Time when this Event was first observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
    /// The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// The object that this event is about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub involved_object: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    /// The time at which the most recent occurrence of this event was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// A human-readable description of the status of this operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// This should be a short, machine understandable string that gives the reason for the transition into the object's current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Optional secondary object for more complex actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    /// Name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_component: Option<std::string::String>,
    /// ID of the controller instance, e.g. `kubelet-xyzf`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_instance: Option<std::string::String>,
    /// Data about the Event series this event represents or nil if it's a singleton Event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<
        <::k8s_openapi027::api::core::v1::EventSeries as crate::Optionable>::Optioned,
    >,
    /// The component reporting this event. Should be a short machine understandable string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<
        <::k8s_openapi027::api::core::v1::EventSource as crate::Optionable>::Optioned,
    >,
    /// Type of this event (Normal, Warning), new types could be added in the future
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Event {
    type Optioned = EventAc;
}
#[automatically_derived]
impl crate::Optionable for EventAc {
    type Optioned = EventAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Event {
    fn into_optioned(self) -> EventAc {
        EventAc {
            api_version: Default::default(),
            kind: Default::default(),
            action: self.action,
            count: self.count,
            event_time: crate::OptionableConvert::into_optioned(self.event_time),
            first_timestamp: crate::OptionableConvert::into_optioned(
                self.first_timestamp,
            ),
            involved_object: Some(
                crate::OptionableConvert::into_optioned(self.involved_object),
            ),
            last_timestamp: crate::OptionableConvert::into_optioned(self.last_timestamp),
            message: self.message,
            metadata: self.metadata,
            reason: self.reason,
            related: crate::OptionableConvert::into_optioned(self.related),
            reporting_component: self.reporting_component,
            reporting_instance: self.reporting_instance,
            series: crate::OptionableConvert::into_optioned(self.series),
            source: crate::OptionableConvert::into_optioned(self.source),
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: EventAc) -> Result<Self, crate::Error> {
        Ok(Self {
            action: value.action,
            count: value.count,
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
            message: value.message,
            metadata: value.metadata,
            reason: value.reason,
            related: crate::OptionableConvert::try_from_optioned(value.related)?,
            reporting_component: value.reporting_component,
            reporting_instance: value.reporting_instance,
            series: crate::OptionableConvert::try_from_optioned(value.series)?,
            source: crate::OptionableConvert::try_from_optioned(value.source)?,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: EventAc) -> Result<(), crate::Error> {
        if self.action.is_none() {
            self.action = crate::OptionableConvert::try_from_optioned(other.action)?;
        } else if let Some(self_value) = self.action.as_mut()
            && let Some(other_value) = other.action
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.count.is_none() {
            self.count = crate::OptionableConvert::try_from_optioned(other.count)?;
        } else if let Some(self_value) = self.count.as_mut()
            && let Some(other_value) = other.count
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.event_time.is_none() {
            self.event_time = crate::OptionableConvert::try_from_optioned(
                other.event_time,
            )?;
        } else if let Some(self_value) = self.event_time.as_mut()
            && let Some(other_value) = other.event_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.first_timestamp.is_none() {
            self.first_timestamp = crate::OptionableConvert::try_from_optioned(
                other.first_timestamp,
            )?;
        } else if let Some(self_value) = self.first_timestamp.as_mut()
            && let Some(other_value) = other.first_timestamp
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.involved_object {
            self.involved_object = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.last_timestamp.is_none() {
            self.last_timestamp = crate::OptionableConvert::try_from_optioned(
                other.last_timestamp,
            )?;
        } else if let Some(self_value) = self.last_timestamp.as_mut()
            && let Some(other_value) = other.last_timestamp
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else if let Some(self_value) = self.message.as_mut()
            && let Some(other_value) = other.message
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.metadata = other.metadata;
        if self.reason.is_none() {
            self.reason = crate::OptionableConvert::try_from_optioned(other.reason)?;
        } else if let Some(self_value) = self.reason.as_mut()
            && let Some(other_value) = other.reason
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.related.is_none() {
            self.related = crate::OptionableConvert::try_from_optioned(other.related)?;
        } else if let Some(self_value) = self.related.as_mut()
            && let Some(other_value) = other.related
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.reporting_component.is_none() {
            self.reporting_component = crate::OptionableConvert::try_from_optioned(
                other.reporting_component,
            )?;
        } else if let Some(self_value) = self.reporting_component.as_mut()
            && let Some(other_value) = other.reporting_component
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.reporting_instance.is_none() {
            self.reporting_instance = crate::OptionableConvert::try_from_optioned(
                other.reporting_instance,
            )?;
        } else if let Some(self_value) = self.reporting_instance.as_mut()
            && let Some(other_value) = other.reporting_instance
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.series.is_none() {
            self.series = crate::OptionableConvert::try_from_optioned(other.series)?;
        } else if let Some(self_value) = self.series.as_mut()
            && let Some(other_value) = other.series
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.source.is_none() {
            self.source = crate::OptionableConvert::try_from_optioned(other.source)?;
        } else if let Some(self_value) = self.source.as_mut()
            && let Some(other_value) = other.source
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.type_.is_none() {
            self.type_ = crate::OptionableConvert::try_from_optioned(other.type_)?;
        } else if let Some(self_value) = self.type_.as_mut()
            && let Some(other_value) = other.type_
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Event> for EventAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Event) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Event, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Event,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for EventAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::core::v1::Event as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::core::v1::Event as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::core::v1::Event as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::core::v1::Event as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::core::v1::Event as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::core::v1::Event as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for EventAc {
    type Ty = <k8s_openapi027::api::core::v1::Event as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_eventac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::core::v1::Event>();
}
impl k8s_openapi027::DeepMerge for EventAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.action, other.action);
        k8s_openapi027::DeepMerge::merge_from(&mut self.count, other.count);
        k8s_openapi027::DeepMerge::merge_from(&mut self.event_time, other.event_time);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.first_timestamp,
            other.first_timestamp,
        );
        self.involved_object = other.involved_object;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.last_timestamp,
            other.last_timestamp,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.message, other.message);
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi027::DeepMerge::merge_from(&mut self.reason, other.reason);
        self.related = other.related;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.reporting_component,
            other.reporting_component,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.reporting_instance,
            other.reporting_instance,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.series, other.series);
        k8s_openapi027::DeepMerge::merge_from(&mut self.source, other.source);
        k8s_openapi027::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}
