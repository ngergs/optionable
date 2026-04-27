#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Event is a report of an event somewhere in the cluster. It generally denotes some state change in the system. Events have a limited retention time and triggers and messages may evolve with time.  Event consumers should not rely on the timing of an event with a given Reason reflecting a consistent underlying trigger, or the continued existence of events with that Reason.  Events should be treated as informative, best-effort, supplemental data.
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
    /// action is what action was taken/failed regarding to the regarding object. It is machine-readable. This field cannot be empty for new Events and it can have at most 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<std::string::String>,
    /// deprecatedCount is the deprecated field assuring backward compatibility with core.v1 Event type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_count: Option<i32>,
    /// deprecatedFirstTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_first_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// deprecatedLastTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_last_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// deprecatedSource is the deprecated field assuring backward compatibility with core.v1 Event type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_source: Option<
        <::k8s_openapi027::api::core::v1::EventSource as crate::Optionable>::Optioned,
    >,
    /// eventTime is the time when this Event was first observed. It is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// note is a human-readable description of the status of this operation. Maximal length of the note is 1kB, but libraries should be prepared to handle values up to 64kB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<std::string::String>,
    /// reason is why the action was taken. It is human-readable. This field cannot be empty for new Events and it can have at most 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// regarding contains the object this Event is about. In most cases it's an Object reporting controller implements, e.g. ReplicaSetController implements ReplicaSets and this event is emitted because it acts on some changes in a ReplicaSet object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regarding: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    /// related is the optional secondary object for more complex actions. E.g. when regarding object triggers a creation or deletion of related object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    /// reportingController is the name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`. This field cannot be empty for new Events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_controller: Option<std::string::String>,
    /// reportingInstance is the ID of the controller instance, e.g. `kubelet-xyzf`. This field cannot be empty for new Events and it can have at most 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_instance: Option<std::string::String>,
    /// series is data about the Event series this event represents or nil if it's a singleton Event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<
        <::k8s_openapi027::api::events::v1::EventSeries as crate::Optionable>::Optioned,
    >,
    /// type is the type of this event (Normal, Warning), new types could be added in the future. It is machine-readable. This field cannot be empty for new Events.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::events::v1::Event {
    type Optioned = EventAc;
}
#[automatically_derived]
impl crate::Optionable for EventAc {
    type Optioned = EventAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::events::v1::Event {
    fn into_optioned(self) -> EventAc {
        EventAc {
            api_version: Default::default(),
            kind: Default::default(),
            action: self.action,
            deprecated_count: self.deprecated_count,
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
            note: self.note,
            reason: self.reason,
            regarding: crate::OptionableConvert::into_optioned(self.regarding),
            related: crate::OptionableConvert::into_optioned(self.related),
            reporting_controller: self.reporting_controller,
            reporting_instance: self.reporting_instance,
            series: crate::OptionableConvert::into_optioned(self.series),
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: EventAc) -> Result<Self, crate::Error> {
        Ok(Self {
            action: value.action,
            deprecated_count: value.deprecated_count,
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
            note: value.note,
            reason: value.reason,
            regarding: crate::OptionableConvert::try_from_optioned(value.regarding)?,
            related: crate::OptionableConvert::try_from_optioned(value.related)?,
            reporting_controller: value.reporting_controller,
            reporting_instance: value.reporting_instance,
            series: crate::OptionableConvert::try_from_optioned(value.series)?,
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
        if self.deprecated_count.is_none() {
            self.deprecated_count = crate::OptionableConvert::try_from_optioned(
                other.deprecated_count,
            )?;
        } else if let Some(self_value) = self.deprecated_count.as_mut()
            && let Some(other_value) = other.deprecated_count
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.deprecated_first_timestamp.is_none() {
            self.deprecated_first_timestamp = crate::OptionableConvert::try_from_optioned(
                other.deprecated_first_timestamp,
            )?;
        } else if let Some(self_value) = self.deprecated_first_timestamp.as_mut()
            && let Some(other_value) = other.deprecated_first_timestamp
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.deprecated_last_timestamp.is_none() {
            self.deprecated_last_timestamp = crate::OptionableConvert::try_from_optioned(
                other.deprecated_last_timestamp,
            )?;
        } else if let Some(self_value) = self.deprecated_last_timestamp.as_mut()
            && let Some(other_value) = other.deprecated_last_timestamp
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.deprecated_source.is_none() {
            self.deprecated_source = crate::OptionableConvert::try_from_optioned(
                other.deprecated_source,
            )?;
        } else if let Some(self_value) = self.deprecated_source.as_mut()
            && let Some(other_value) = other.deprecated_source
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
        self.metadata = other.metadata;
        if self.note.is_none() {
            self.note = crate::OptionableConvert::try_from_optioned(other.note)?;
        } else if let Some(self_value) = self.note.as_mut()
            && let Some(other_value) = other.note
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = crate::OptionableConvert::try_from_optioned(other.reason)?;
        } else if let Some(self_value) = self.reason.as_mut()
            && let Some(other_value) = other.reason
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.regarding.is_none() {
            self.regarding = crate::OptionableConvert::try_from_optioned(
                other.regarding,
            )?;
        } else if let Some(self_value) = self.regarding.as_mut()
            && let Some(other_value) = other.regarding
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.related.is_none() {
            self.related = crate::OptionableConvert::try_from_optioned(other.related)?;
        } else if let Some(self_value) = self.related.as_mut()
            && let Some(other_value) = other.related
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.reporting_controller.is_none() {
            self.reporting_controller = crate::OptionableConvert::try_from_optioned(
                other.reporting_controller,
            )?;
        } else if let Some(self_value) = self.reporting_controller.as_mut()
            && let Some(other_value) = other.reporting_controller
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
impl crate::OptionedConvert<k8s_openapi027::api::events::v1::Event> for EventAc {
    fn from_optionable(value: k8s_openapi027::api::events::v1::Event) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::events::v1::Event, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::events::v1::Event,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for EventAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::events::v1::Event as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::events::v1::Event as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::events::v1::Event as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::events::v1::Event as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::events::v1::Event as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::events::v1::Event as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for EventAc {
    type Ty = <k8s_openapi027::api::events::v1::Event as k8s_openapi027::Metadata>::Ty;
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
    crate::testutil::roundtrip_test::<k8s_openapi027::api::events::v1::Event>();
}
impl k8s_openapi027::DeepMerge for EventAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.action, other.action);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.deprecated_count,
            other.deprecated_count,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.deprecated_first_timestamp,
            other.deprecated_first_timestamp,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.deprecated_last_timestamp,
            other.deprecated_last_timestamp,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.deprecated_source,
            other.deprecated_source,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.event_time, other.event_time);
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi027::DeepMerge::merge_from(&mut self.note, other.note);
        k8s_openapi027::DeepMerge::merge_from(&mut self.reason, other.reason);
        self.regarding = other.regarding;
        self.related = other.related;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.reporting_controller,
            other.reporting_controller,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.reporting_instance,
            other.reporting_instance,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.series, other.series);
        k8s_openapi027::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}
