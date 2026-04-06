#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NetworkPolicyPort describes a port to allow traffic on
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkPolicyPortAc {
    /// endPort indicates that the range of ports from port to endPort if set, inclusive, should be allowed by the policy. This field cannot be defined if the port field is not defined or if the port field is defined as a named (string) port. The endPort must be equal or greater than port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_port: Option<i32>,
    /// port represents the port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<
        <::k8s_openapi027::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    /// protocol represents the protocol (TCP, UDP, or SCTP) which traffic must match. If not specified, this field defaults to TCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::NetworkPolicyPort {
    type Optioned = NetworkPolicyPortAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyPortAc {
    type Optioned = NetworkPolicyPortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::NetworkPolicyPort {
    fn into_optioned(self) -> NetworkPolicyPortAc {
        NetworkPolicyPortAc {
            end_port: self.end_port,
            port: crate::OptionableConvert::into_optioned(self.port),
            protocol: self.protocol,
        }
    }
    fn try_from_optioned(value: NetworkPolicyPortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            end_port: value.end_port,
            port: crate::OptionableConvert::try_from_optioned(value.port)?,
            protocol: value.protocol,
        })
    }
    fn merge(&mut self, other: NetworkPolicyPortAc) -> Result<(), crate::Error> {
        self.end_port = other.end_port;
        crate::OptionableConvert::merge(&mut self.port, other.port)?;
        self.protocol = other.protocol;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::NetworkPolicyPort>
for NetworkPolicyPortAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::NetworkPolicyPort,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::NetworkPolicyPort, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::NetworkPolicyPort,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
