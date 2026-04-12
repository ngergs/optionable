#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServicePort contains information on service's port.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServicePortAc {
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
    /// The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// The port on each node on which this service is exposed when type is NodePort or LoadBalancer.  Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail.  If not specified, a port will be allocated if this Service requires one.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIP). More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port: Option<i32>,
    /// The port that will be exposed by this service.
    pub port: i32,
    /// The IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.
    pub protocol: Option<std::string::String>,
    /// Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_port: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ServicePort {
    type Optioned = ServicePortAc;
}
#[automatically_derived]
impl crate::Optionable for ServicePortAc {
    type Optioned = ServicePortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ServicePort {
    fn into_optioned(self) -> ServicePortAc {
        ServicePortAc {
            app_protocol: self.app_protocol,
            name: self.name,
            node_port: self.node_port,
            port: self.port,
            protocol: self.protocol,
            target_port: crate::OptionableConvert::into_optioned(self.target_port),
        }
    }
    fn try_from_optioned(value: ServicePortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            app_protocol: value.app_protocol,
            name: value.name,
            node_port: value.node_port,
            port: value.port,
            protocol: value.protocol,
            target_port: crate::OptionableConvert::try_from_optioned(value.target_port)?,
        })
    }
    fn merge(&mut self, other: ServicePortAc) -> Result<(), crate::Error> {
        if self.app_protocol.is_none() {
            self.app_protocol = crate::OptionableConvert::try_from_optioned(
                other.app_protocol,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.app_protocol, other.app_protocol)?;
        }
        if self.name.is_none() {
            self.name = crate::OptionableConvert::try_from_optioned(other.name)?;
        } else {
            crate::OptionableConvert::merge(&mut self.name, other.name)?;
        }
        if self.node_port.is_none() {
            self.node_port = crate::OptionableConvert::try_from_optioned(
                other.node_port,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.node_port, other.node_port)?;
        }
        self.port = other.port;
        self.protocol = other.protocol;
        if self.target_port.is_none() {
            self.target_port = crate::OptionableConvert::try_from_optioned(
                other.target_port,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.target_port, other.target_port)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::ServicePort {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.port == other.port && self.protocol == other.protocol
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ServicePort>
for ServicePortAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ServicePort) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ServicePort, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ServicePort,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
