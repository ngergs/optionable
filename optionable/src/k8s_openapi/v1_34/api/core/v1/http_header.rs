#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HTTPHeaderAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::HTTPHeader {
    type Optioned = HTTPHeaderAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPHeaderAc {
    type Optioned = HTTPHeaderAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HTTPHeader {
    fn into_optioned(self) -> HTTPHeaderAc {
        HTTPHeaderAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            value: Some(crate::OptionableConvert::into_optioned(self.value)),
        }
    }
    fn try_from_optioned(value: HTTPHeaderAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            value: crate::OptionableConvert::try_from_optioned(
                value
                    .value
                    .ok_or(crate::optionable::Error {
                        missing_field: "value",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: HTTPHeaderAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.value {
            crate::OptionableConvert::merge(&mut self.value, other_value)?;
        }
        Ok(())
    }
}
