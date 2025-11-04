#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct IngressLoadBalancerIngressAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::IngressPortStatus>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1::IngressLoadBalancerIngress {
    type Optioned = IngressLoadBalancerIngressAc;
}
#[automatically_derived]
impl crate::Optionable for IngressLoadBalancerIngressAc {
    type Optioned = IngressLoadBalancerIngressAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::IngressLoadBalancerIngress {
    fn into_optioned(self) -> IngressLoadBalancerIngressAc {
        IngressLoadBalancerIngressAc {
            hostname: crate::OptionableConvert::into_optioned(self.hostname),
            ip: crate::OptionableConvert::into_optioned(self.ip),
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: IngressLoadBalancerIngressAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hostname: crate::OptionableConvert::try_from_optioned(value.hostname)?,
            ip: crate::OptionableConvert::try_from_optioned(value.ip)?,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressLoadBalancerIngressAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.hostname, other.hostname)?;
        crate::OptionableConvert::merge(&mut self.ip, other.ip)?;
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
