#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FlowDistinguisherMethodAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::FlowDistinguisherMethod {
    type Optioned = FlowDistinguisherMethodAc;
}
#[automatically_derived]
impl crate::Optionable for FlowDistinguisherMethodAc {
    type Optioned = FlowDistinguisherMethodAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::FlowDistinguisherMethod {
    fn into_optioned(self) -> FlowDistinguisherMethodAc {
        FlowDistinguisherMethodAc {
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(
        value: FlowDistinguisherMethodAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: FlowDistinguisherMethodAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
