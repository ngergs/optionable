#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressPortStatus represents the error condition of a service port
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressPortStatusAc {
    /// error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use
    ///   CamelCase names
    /// - cloud provider specific error values must have names that comply with the
    ///   format foo.example.com/CamelCase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<std::string::String>,
    /// port is the port number of the ingress port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// protocol is the protocol of the ingress port. The supported values are: "TCP", "UDP", "SCTP"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressPortStatus {
    type Optioned = IngressPortStatusAc;
}
#[automatically_derived]
impl crate::Optionable for IngressPortStatusAc {
    type Optioned = IngressPortStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::IngressPortStatus {
    fn into_optioned(self) -> IngressPortStatusAc {
        IngressPortStatusAc {
            error: self.error,
            port: Some(self.port),
            protocol: Some(self.protocol),
        }
    }
    fn try_from_optioned(value: IngressPortStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            error: value.error,
            port: value
                .port
                .ok_or(crate::Error {
                    missing_field: "port",
                })?,
            protocol: value
                .protocol
                .ok_or(crate::Error {
                    missing_field: "protocol",
                })?,
        })
    }
    fn merge(&mut self, other: IngressPortStatusAc) -> Result<(), crate::Error> {
        if other.error.is_some() {
            self.error = other.error;
        }
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        if let Some(other_value) = other.protocol {
            self.protocol = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressPortStatus>
for IngressPortStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::IngressPortStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressPortStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressPortStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
