#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// LoadBalancerIngress represents the status of a load-balancer ingress point: traffic intended for the service should be sent to an ingress point.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoadBalancerIngressAc {
    /// Hostname is set for load-balancer ingress points that are DNS based (typically AWS load-balancers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    /// IP is set for load-balancer ingress points that are IP based (typically GCE or OpenStack load-balancers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<std::string::String>,
    /// IPMode specifies how the load-balancer IP behaves, and may only be specified when the ip field is specified. Setting this to "VIP" indicates that traffic is delivered to the node with the destination set to the load-balancer's IP and port. Setting this to "Proxy" indicates that traffic is delivered to the node or pod with the destination set to the node's IP and node port or the pod's IP and port. Service implementations may use this information to adjust traffic routing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<std::string::String>,
    /// Ports is a list of records of service ports If used, every port defined in the service should have an entry in it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PortStatus as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::LoadBalancerIngress {
    type Optioned = LoadBalancerIngressAc;
}
#[automatically_derived]
impl crate::Optionable for LoadBalancerIngressAc {
    type Optioned = LoadBalancerIngressAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::LoadBalancerIngress {
    fn into_optioned(self) -> LoadBalancerIngressAc {
        LoadBalancerIngressAc {
            hostname: self.hostname,
            ip: self.ip,
            ip_mode: self.ip_mode,
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(value: LoadBalancerIngressAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hostname: value.hostname,
            ip: value.ip,
            ip_mode: value.ip_mode,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(&mut self, other: LoadBalancerIngressAc) -> Result<(), crate::Error> {
        if self.hostname.is_none() {
            self.hostname = other.hostname;
        }
        if let Some(other_value) = other.hostname {
            crate::OptionableConvert::merge(&mut self.hostname, other_value)?;
        }
        if self.ip.is_none() {
            self.ip = other.ip;
        }
        if let Some(other_value) = other.ip {
            crate::OptionableConvert::merge(&mut self.ip, other_value)?;
        }
        if self.ip_mode.is_none() {
            self.ip_mode = other.ip_mode;
        }
        if let Some(other_value) = other.ip_mode {
            crate::OptionableConvert::merge(&mut self.ip_mode, other_value)?;
        }
        if self.ports.is_none() {
            self.ports = other.ports;
        }
        if let Some(other_value) = other.ports {
            self.ports = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::LoadBalancerIngress>
for LoadBalancerIngressAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::LoadBalancerIngress,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::LoadBalancerIngress, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::LoadBalancerIngress,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
