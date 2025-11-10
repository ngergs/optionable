#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct EndpointConditionsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::EndpointConditions {
    type Optioned = EndpointConditionsAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointConditionsAc {
    type Optioned = EndpointConditionsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::EndpointConditions {
    fn into_optioned(self) -> EndpointConditionsAc {
        EndpointConditionsAc {
            ready: crate::OptionableConvert::into_optioned(self.ready),
            serving: crate::OptionableConvert::into_optioned(self.serving),
            terminating: crate::OptionableConvert::into_optioned(self.terminating),
        }
    }
    fn try_from_optioned(value: EndpointConditionsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ready: crate::OptionableConvert::try_from_optioned(value.ready)?,
            serving: crate::OptionableConvert::try_from_optioned(value.serving)?,
            terminating: crate::OptionableConvert::try_from_optioned(value.terminating)?,
        })
    }
    fn merge(&mut self, other: EndpointConditionsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.ready, other.ready)?;
        crate::OptionableConvert::merge(&mut self.serving, other.serving)?;
        crate::OptionableConvert::merge(&mut self.terminating, other.terminating)?;
        Ok(())
    }
}
