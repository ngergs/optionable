pub struct LoadBalancerIngressOpt {
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ip_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PortStatus>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::load_balancer_ingress::LoadBalancerIngress {
    type Optioned = LoadBalancerIngressOpt;
}
#[automatically_derived]
impl crate::Optionable for LoadBalancerIngressOpt {
    type Optioned = LoadBalancerIngressOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::load_balancer_ingress::LoadBalancerIngress {
    fn into_optioned(self) -> LoadBalancerIngressOpt {
        LoadBalancerIngressOpt {
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.hostname),
            ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.ip),
            ip_mode: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.ip_mode),
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PortStatus>,
            > as crate::OptionableConvert>::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: LoadBalancerIngressOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hostname: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.hostname)?,
            ip: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.ip)?,
            ip_mode: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.ip_mode)?,
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PortStatus>,
            > as crate::OptionableConvert>::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: LoadBalancerIngressOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.hostname, other.hostname)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.ip, other.ip)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.ip_mode, other.ip_mode)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::PortStatus>,
        > as crate::OptionableConvert>::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
