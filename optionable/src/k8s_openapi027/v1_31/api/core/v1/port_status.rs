#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PortStatusAc {
    /// Error is to record the problem with the service port The format of the error shall comply with the following rules: - built-in error values shall be specified in this file and those shall use
    ///   CamelCase names
    /// - cloud provider specific error values must have names that comply with the
    ///   format foo.example.com/CamelCase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<std::string::String>,
    /// Port is the port number of the service port of which status is recorded here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Protocol is the protocol of the service port of which status is recorded here The supported values are: "TCP", "UDP", "SCTP"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PortStatus {
    type Optioned = PortStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PortStatusAc {
    type Optioned = PortStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PortStatus {
    fn into_optioned(self) -> PortStatusAc {
        PortStatusAc {
            error: self.error,
            port: Some(self.port),
            protocol: Some(self.protocol),
        }
    }
    fn try_from_optioned(value: PortStatusAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: PortStatusAc) -> Result<(), crate::Error> {
        self.error = other.error;
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
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PortStatus> for PortStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PortStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PortStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PortStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
