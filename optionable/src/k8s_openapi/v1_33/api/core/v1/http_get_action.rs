#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct HTTPGetActionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_headers: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::HTTPHeader>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<
        <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::HTTPGetAction {
    type Optioned = HTTPGetActionAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPGetActionAc {
    type Optioned = HTTPGetActionAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HTTPGetAction {
    fn into_optioned(self) -> HTTPGetActionAc {
        HTTPGetActionAc {
            host: crate::OptionableConvert::into_optioned(self.host),
            http_headers: crate::OptionableConvert::into_optioned(self.http_headers),
            path: crate::OptionableConvert::into_optioned(self.path),
            port: Some(crate::OptionableConvert::into_optioned(self.port)),
            scheme: crate::OptionableConvert::into_optioned(self.scheme),
        }
    }
    fn try_from_optioned(value: HTTPGetActionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            host: crate::OptionableConvert::try_from_optioned(value.host)?,
            http_headers: crate::OptionableConvert::try_from_optioned(
                value.http_headers,
            )?,
            path: crate::OptionableConvert::try_from_optioned(value.path)?,
            port: crate::OptionableConvert::try_from_optioned(
                value
                    .port
                    .ok_or(crate::Error {
                        missing_field: "port",
                    })?,
            )?,
            scheme: crate::OptionableConvert::try_from_optioned(value.scheme)?,
        })
    }
    fn merge(&mut self, other: HTTPGetActionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.host, other.host)?;
        crate::OptionableConvert::merge(&mut self.http_headers, other.http_headers)?;
        crate::OptionableConvert::merge(&mut self.path, other.path)?;
        if let Some(other_value) = other.port {
            crate::OptionableConvert::merge(&mut self.port, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.scheme, other.scheme)?;
        Ok(())
    }
}
