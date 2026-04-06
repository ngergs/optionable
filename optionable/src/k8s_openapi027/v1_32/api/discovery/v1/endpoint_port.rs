#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointPortAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::discovery::v1::EndpointPort {
    type Optioned = EndpointPortAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointPortAc {
    type Optioned = EndpointPortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::discovery::v1::EndpointPort {
    fn into_optioned(self) -> EndpointPortAc {
        EndpointPortAc {
            app_protocol: self.app_protocol,
            name: self.name,
            port: self.port,
            protocol: self.protocol,
        }
    }
    fn try_from_optioned(value: EndpointPortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            app_protocol: value.app_protocol,
            name: value.name,
            port: value.port,
            protocol: value.protocol,
        })
    }
    fn merge(&mut self, other: EndpointPortAc) -> Result<(), crate::Error> {
        self.app_protocol = other.app_protocol;
        self.name = other.name;
        self.port = other.port;
        self.protocol = other.protocol;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::discovery::v1::EndpointPort>
for EndpointPortAc {
    fn from_optionable(value: k8s_openapi027::api::discovery::v1::EndpointPort) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::discovery::v1::EndpointPort, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::discovery::v1::EndpointPort,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
