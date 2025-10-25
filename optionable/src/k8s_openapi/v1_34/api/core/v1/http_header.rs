pub struct HTTPHeaderOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub value: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::HTTPHeader {
    type Optioned = HTTPHeaderOpt;
}
#[automatically_derived]
impl crate::Optionable for HTTPHeaderOpt {
    type Optioned = HTTPHeaderOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HTTPHeader {
    fn into_optioned(self) -> HTTPHeaderOpt {
        HTTPHeaderOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            value: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.value,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: HTTPHeaderOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            value: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .value
                    .ok_or(crate::optionable::Error {
                        missing_field: "value",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: HTTPHeaderOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.value {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.value,
                other_value,
            )?;
        }
        Ok(())
    }
}
