pub struct FlowDistinguisherMethodOpt {
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::FlowDistinguisherMethod {
    type Optioned = FlowDistinguisherMethodOpt;
}
#[automatically_derived]
impl crate::Optionable for FlowDistinguisherMethodOpt {
    type Optioned = FlowDistinguisherMethodOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::FlowDistinguisherMethod {
    fn into_optioned(self) -> FlowDistinguisherMethodOpt {
        FlowDistinguisherMethodOpt {
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: FlowDistinguisherMethodOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
        other: FlowDistinguisherMethodOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
