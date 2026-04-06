#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HTTPGetActionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::HTTPHeader as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::HTTPGetAction {
    type Optioned = HTTPGetActionAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPGetActionAc {
    type Optioned = HTTPGetActionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::HTTPGetAction {
    fn into_optioned(self) -> HTTPGetActionAc {
        HTTPGetActionAc {
            host: self.host,
            http_headers: crate::OptionableConvert::into_optioned(self.http_headers),
            path: self.path,
            port: Some(crate::OptionableConvert::into_optioned(self.port)),
            scheme: self.scheme,
        }
    }
    fn try_from_optioned(value: HTTPGetActionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            host: value.host,
            http_headers: crate::OptionableConvert::try_from_optioned(
                value.http_headers,
            )?,
            path: value.path,
            port: crate::OptionableConvert::try_from_optioned(
                value
                    .port
                    .ok_or(crate::Error {
                        missing_field: "port",
                    })?,
            )?,
            scheme: value.scheme,
        })
    }
    fn merge(&mut self, other: HTTPGetActionAc) -> Result<(), crate::Error> {
        self.host = other.host;
        crate::OptionableConvert::merge(&mut self.http_headers, other.http_headers)?;
        self.path = other.path;
        if let Some(other_value) = other.port {
            crate::OptionableConvert::merge(&mut self.port, other_value)?;
        }
        self.scheme = other.scheme;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::HTTPGetAction>
for HTTPGetActionAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::HTTPGetAction) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::HTTPGetAction, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::HTTPGetAction,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
