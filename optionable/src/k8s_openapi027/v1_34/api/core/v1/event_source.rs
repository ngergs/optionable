#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EventSource contains information for an event.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EventSourceAc {
    /// Component from which the event is generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<std::string::String>,
    /// Node name on which the event is generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EventSource {
    type Optioned = EventSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EventSourceAc {
    type Optioned = EventSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EventSource {
    fn into_optioned(self) -> EventSourceAc {
        EventSourceAc {
            component: self.component,
            host: self.host,
        }
    }
    fn try_from_optioned(value: EventSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            component: value.component,
            host: value.host,
        })
    }
    fn merge(&mut self, other: EventSourceAc) -> Result<(), crate::Error> {
        self.component = other.component;
        self.host = other.host;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EventSource>
for EventSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::EventSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EventSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EventSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
