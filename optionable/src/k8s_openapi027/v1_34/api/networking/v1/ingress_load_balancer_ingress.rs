#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressLoadBalancerIngressAc {
    /// hostname is set for load-balancer ingress points that are DNS based.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    /// ip is set for load-balancer ingress points that are IP based.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<std::string::String>,
    /// ports provides information about the ports exposed by this LoadBalancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::IngressPortStatus as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::networking::v1::IngressLoadBalancerIngress {
    type Optioned = IngressLoadBalancerIngressAc;
}
#[automatically_derived]
impl crate::Optionable for IngressLoadBalancerIngressAc {
    type Optioned = IngressLoadBalancerIngressAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::IngressLoadBalancerIngress {
    fn into_optioned(self) -> IngressLoadBalancerIngressAc {
        IngressLoadBalancerIngressAc {
            hostname: self.hostname,
            ip: self.ip,
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: IngressLoadBalancerIngressAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            hostname: value.hostname,
            ip: value.ip,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressLoadBalancerIngressAc,
    ) -> Result<(), crate::Error> {
        if self.hostname.is_none() {
            self.hostname = crate::OptionableConvert::try_from_optioned(other.hostname)?;
        } else if let Some(self_value) = self.hostname.as_mut()
            && let Some(other_value) = other.hostname
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.ip.is_none() {
            self.ip = crate::OptionableConvert::try_from_optioned(other.ip)?;
        } else if let Some(self_value) = self.ip.as_mut()
            && let Some(other_value) = other.ip
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.ports.is_none() {
            self.ports = crate::OptionableConvert::try_from_optioned(other.ports)?;
        } else if let Some(self_value) = self.ports.as_mut()
            && let Some(other_value) = other.ports
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::networking::v1::IngressLoadBalancerIngress,
> for IngressLoadBalancerIngressAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::IngressLoadBalancerIngress,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::networking::v1::IngressLoadBalancerIngress,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressLoadBalancerIngress,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
