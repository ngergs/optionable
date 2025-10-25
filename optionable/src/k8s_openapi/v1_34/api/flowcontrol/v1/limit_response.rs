pub struct LimitResponseOpt {
    pub queuing: <Option<
        ::k8s_openapi::api::flowcontrol::v1::QueuingConfiguration,
    > as crate::Optionable>::Optioned,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::limit_response::LimitResponse {
    type Optioned = LimitResponseOpt;
}
#[automatically_derived]
impl crate::Optionable for LimitResponseOpt {
    type Optioned = LimitResponseOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::limit_response::LimitResponse {
    fn into_optioned(self) -> LimitResponseOpt {
        LimitResponseOpt {
            queuing: <Option<
                ::k8s_openapi::api::flowcontrol::v1::QueuingConfiguration,
            > as crate::OptionableConvert>::into_optioned(self.queuing),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: LimitResponseOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            queuing: <Option<
                ::k8s_openapi::api::flowcontrol::v1::QueuingConfiguration,
            > as crate::OptionableConvert>::try_from_optioned(value.queuing)?,
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
        other: LimitResponseOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::flowcontrol::v1::QueuingConfiguration,
        > as crate::OptionableConvert>::merge(&mut self.queuing, other.queuing)?;
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
