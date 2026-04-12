#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EndpointPort represents a Port used by an EndpointSlice
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointPortAc {
    /// The application protocol for this port. This is used as a hint for implementations to offer richer behavior for protocols that they understand. This field follows standard Kubernetes label syntax. Valid values are either:
    ///
    /// * Un-prefixed protocol names - reserved for IANA standard service names (as per RFC-6335 and https://www.iana.org/assignments/service-names).
    ///
    /// * Kubernetes-defined prefixed names:
    ///   * 'kubernetes.io/h2c' - HTTP/2 prior knowledge over cleartext as described in https://www.rfc-editor.org/rfc/rfc9113.html#name-starting-http-2-with-prior-
    ///   * 'kubernetes.io/ws'  - WebSocket over cleartext as described in https://www.rfc-editor.org/rfc/rfc6455
    ///   * 'kubernetes.io/wss' - WebSocket over TLS as described in https://www.rfc-editor.org/rfc/rfc6455
    ///
    /// * Other protocols should use implementation-defined prefixed names such as mycompany.com/my-custom-protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_protocol: Option<std::string::String>,
    /// name represents the name of this port. All ports in an EndpointSlice must have a unique name. If the EndpointSlice is derived from a Kubernetes service, this corresponds to the Service.ports\[\].name. Name must either be an empty string or pass DNS_LABEL validation: * must be no more than 63 characters long. * must consist of lower case alphanumeric characters or '-'. * must start and end with an alphanumeric character. Default is empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// port represents the port number of the endpoint. If the EndpointSlice is derived from a Kubernetes service, this must be set to the service's target port. EndpointSlices used for other purposes may have a nil port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// protocol represents the IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP.
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
        if self.app_protocol.is_none() {
            self.app_protocol = crate::OptionableConvert::try_from_optioned(
                other.app_protocol,
            )?;
        } else if let Some(self_value) = self.app_protocol.as_mut()
            && let Some(other_value) = other.app_protocol
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.name.is_none() {
            self.name = crate::OptionableConvert::try_from_optioned(other.name)?;
        } else if let Some(self_value) = self.name.as_mut()
            && let Some(other_value) = other.name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.port.is_none() {
            self.port = crate::OptionableConvert::try_from_optioned(other.port)?;
        } else if let Some(self_value) = self.port.as_mut()
            && let Some(other_value) = other.port
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.protocol.is_none() {
            self.protocol = crate::OptionableConvert::try_from_optioned(other.protocol)?;
        } else if let Some(self_value) = self.protocol.as_mut()
            && let Some(other_value) = other.protocol
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
