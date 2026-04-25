#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HTTPGetAction describes an action based on HTTP Get requests.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HTTPGetActionAc {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<std::string::String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::HTTPHeader as crate::Optionable>::Optioned,
        >,
    >,
    /// Path to access on the HTTP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
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
        if self.host.is_none() {
            self.host = crate::OptionableConvert::try_from_optioned(other.host)?;
        } else if let Some(self_value) = self.host.as_mut()
            && let Some(other_value) = other.host
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.http_headers.is_none() {
            self.http_headers = crate::OptionableConvert::try_from_optioned(
                other.http_headers,
            )?;
        } else if let Some(self_value) = self.http_headers.as_mut()
            && let Some(other_value) = other.http_headers
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.path.is_none() {
            self.path = crate::OptionableConvert::try_from_optioned(other.path)?;
        } else if let Some(self_value) = self.path.as_mut()
            && let Some(other_value) = other.path
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.port {
            crate::OptionableConvert::merge(&mut self.port, other_value)?;
        }
        if self.scheme.is_none() {
            self.scheme = crate::OptionableConvert::try_from_optioned(other.scheme)?;
        } else if let Some(self_value) = self.scheme.as_mut()
            && let Some(other_value) = other.scheme
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for HTTPGetActionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.host, other.host);
        self.http_headers = other.http_headers;
        k8s_openapi027::DeepMerge::merge_from(&mut self.path, other.path);
        k8s_openapi027::DeepMerge::merge_from(&mut self.port, other.port);
        k8s_openapi027::DeepMerge::merge_from(&mut self.scheme, other.scheme);
    }
}
