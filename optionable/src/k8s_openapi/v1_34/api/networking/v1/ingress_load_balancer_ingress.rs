pub struct IngressLoadBalancerIngressOpt {
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::IngressPortStatus>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1::ingress_load_balancer_ingress::IngressLoadBalancerIngress {
    type Optioned = IngressLoadBalancerIngressOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressLoadBalancerIngressOpt {
    type Optioned = IngressLoadBalancerIngressOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::ingress_load_balancer_ingress::IngressLoadBalancerIngress {
    fn into_optioned(self) -> IngressLoadBalancerIngressOpt {
        IngressLoadBalancerIngressOpt {
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.hostname),
            ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.ip),
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::IngressPortStatus>,
            > as crate::OptionableConvert>::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: IngressLoadBalancerIngressOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.hostname)?,
            ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.ip)?,
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::IngressPortStatus>,
            > as crate::OptionableConvert>::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressLoadBalancerIngressOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.hostname, other.hostname)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.ip, other.ip)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::networking::v1::IngressPortStatus>,
        > as crate::OptionableConvert>::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
