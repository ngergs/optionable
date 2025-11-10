#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct FlowDistinguisherMethodAc {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1beta3::FlowDistinguisherMethod {
    type Optioned = FlowDistinguisherMethodAc;
}
#[automatically_derived]
impl crate::Optionable for FlowDistinguisherMethodAc {
    type Optioned = FlowDistinguisherMethodAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1beta3::FlowDistinguisherMethod {
    fn into_optioned(self) -> FlowDistinguisherMethodAc {
        FlowDistinguisherMethodAc {
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(
        value: FlowDistinguisherMethodAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: FlowDistinguisherMethodAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
