#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct EventSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EventSource {
    type Optioned = EventSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EventSourceAc {
    type Optioned = EventSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EventSource {
    fn into_optioned(self) -> EventSourceAc {
        EventSourceAc {
            component: crate::OptionableConvert::into_optioned(self.component),
            host: crate::OptionableConvert::into_optioned(self.host),
        }
    }
    fn try_from_optioned(value: EventSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            component: crate::OptionableConvert::try_from_optioned(value.component)?,
            host: crate::OptionableConvert::try_from_optioned(value.host)?,
        })
    }
    fn merge(&mut self, other: EventSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.component, other.component)?;
        crate::OptionableConvert::merge(&mut self.host, other.host)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::EventSource>
for EventSourceAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::EventSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::EventSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::EventSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
