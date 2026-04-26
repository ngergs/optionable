#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EndpointPort is a tuple that describes a single port.
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
    /// The name of this port.  This must match the 'name' field in the corresponding ServicePort. Must be a DNS_LABEL. Optional only if one port is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// The port number of the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// The IP protocol for this port. Must be UDP, TCP, or SCTP. Default is TCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EndpointPort {
    type Optioned = EndpointPortAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointPortAc {
    type Optioned = EndpointPortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EndpointPort {
    fn into_optioned(self) -> EndpointPortAc {
        EndpointPortAc {
            app_protocol: self.app_protocol,
            name: self.name,
            port: Some(self.port),
            protocol: self.protocol,
        }
    }
    fn try_from_optioned(value: EndpointPortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            app_protocol: value.app_protocol,
            name: value.name,
            port: value
                .port
                .ok_or(crate::Error {
                    missing_field: "port",
                })?,
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
        if let Some(other_value) = other.port {
            self.port = crate::OptionableConvert::try_from_optioned(other_value)?;
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
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EndpointPort>
for EndpointPortAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::EndpointPort) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EndpointPort, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EndpointPort,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for EndpointPortAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.app_protocol,
            other.app_protocol,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.port, other.port);
        k8s_openapi027::DeepMerge::merge_from(&mut self.protocol, other.protocol);
    }
}
