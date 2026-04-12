#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerPort represents a network port in a single container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerPortAc {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 \< x \< 65536.
    pub container_port: i32,
    /// What host IP to bind the external port to.
    #[serde(rename = "hostIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<std::string::String>,
    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 \< x \< 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
    pub protocol: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerPort {
    type Optioned = ContainerPortAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerPortAc {
    type Optioned = ContainerPortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerPort {
    fn into_optioned(self) -> ContainerPortAc {
        ContainerPortAc {
            container_port: self.container_port,
            host_ip: self.host_ip,
            host_port: self.host_port,
            name: self.name,
            protocol: self.protocol,
        }
    }
    fn try_from_optioned(value: ContainerPortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            container_port: value.container_port,
            host_ip: value.host_ip,
            host_port: value.host_port,
            name: value.name,
            protocol: value.protocol,
        })
    }
    fn merge(&mut self, other: ContainerPortAc) -> Result<(), crate::Error> {
        self.container_port = other.container_port;
        if self.host_ip.is_none() {
            self.host_ip = crate::OptionableConvert::try_from_optioned(other.host_ip)?;
        } else {
            crate::OptionableConvert::merge(&mut self.host_ip, other.host_ip)?;
        }
        if self.host_port.is_none() {
            self.host_port = crate::OptionableConvert::try_from_optioned(
                other.host_port,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.host_port, other.host_port)?;
        }
        if self.name.is_none() {
            self.name = crate::OptionableConvert::try_from_optioned(other.name)?;
        } else {
            crate::OptionableConvert::merge(&mut self.name, other.name)?;
        }
        self.protocol = other.protocol;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::ContainerPort {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.container_port == other.container_port && self.protocol == other.protocol
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerPort>
for ContainerPortAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ContainerPort) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerPort, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerPort,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
