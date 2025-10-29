pub struct EventSourceOpt {
    pub component: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EventSource {
    type Optioned = EventSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for EventSourceOpt {
    type Optioned = EventSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EventSource {
    fn into_optioned(self) -> EventSourceOpt {
        EventSourceOpt {
            component: crate::OptionableConvert::into_optioned(self.component),
            host: crate::OptionableConvert::into_optioned(self.host),
        }
    }
    fn try_from_optioned(
        value: EventSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            component: crate::OptionableConvert::try_from_optioned(value.component)?,
            host: crate::OptionableConvert::try_from_optioned(value.host)?,
        })
    }
    fn merge(&mut self, other: EventSourceOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.component, other.component)?;
        crate::OptionableConvert::merge(&mut self.host, other.host)?;
        Ok(())
    }
}
