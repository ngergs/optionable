#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
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
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EventSource {
    fn into_optioned(self) -> EventSourceAc {
        EventSourceAc {
            component: crate::OptionableConvert::into_optioned(self.component),
            host: crate::OptionableConvert::into_optioned(self.host),
        }
    }
    fn try_from_optioned(
        value: EventSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            component: crate::OptionableConvert::try_from_optioned(value.component)?,
            host: crate::OptionableConvert::try_from_optioned(value.host)?,
        })
    }
    fn merge(&mut self, other: EventSourceAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.component, other.component)?;
        crate::OptionableConvert::merge(&mut self.host, other.host)?;
        Ok(())
    }
}
