#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPortAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EndpointPort {
    type Optioned = EndpointPortAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointPortAc {
    type Optioned = EndpointPortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EndpointPort {
    fn into_optioned(self) -> EndpointPortAc {
        EndpointPortAc {
            app_protocol: crate::OptionableConvert::into_optioned(self.app_protocol),
            name: crate::OptionableConvert::into_optioned(self.name),
            port: Some(self.port),
            protocol: crate::OptionableConvert::into_optioned(self.protocol),
        }
    }
    fn try_from_optioned(value: EndpointPortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            app_protocol: crate::OptionableConvert::try_from_optioned(
                value.app_protocol,
            )?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            port: value
                .port
                .ok_or(crate::Error {
                    missing_field: "port",
                })?,
            protocol: crate::OptionableConvert::try_from_optioned(value.protocol)?,
        })
    }
    fn merge(&mut self, other: EndpointPortAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.app_protocol, other.app_protocol)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        crate::OptionableConvert::merge(&mut self.protocol, other.protocol)?;
        Ok(())
    }
}
