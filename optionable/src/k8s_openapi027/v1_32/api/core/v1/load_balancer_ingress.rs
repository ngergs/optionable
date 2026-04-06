#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LoadBalancerIngressAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<std::string::String>,
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
        self.hostname = other.hostname;
        self.ip = other.ip;
        self.ip_mode = other.ip_mode;
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
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
