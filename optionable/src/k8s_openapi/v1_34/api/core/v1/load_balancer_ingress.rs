pub struct LoadBalancerIngressAc {
    pub hostname: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ip: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ip_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PortStatus>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LoadBalancerIngress {
    type Optioned = LoadBalancerIngressAc;
}
#[automatically_derived]
impl crate::Optionable for LoadBalancerIngressAc {
    type Optioned = LoadBalancerIngressAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LoadBalancerIngress {
    fn into_optioned(self) -> LoadBalancerIngressAc {
        LoadBalancerIngressAc {
            hostname: crate::OptionableConvert::into_optioned(self.hostname),
            ip: crate::OptionableConvert::into_optioned(self.ip),
            ip_mode: crate::OptionableConvert::into_optioned(self.ip_mode),
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: LoadBalancerIngressAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hostname: crate::OptionableConvert::try_from_optioned(value.hostname)?,
            ip: crate::OptionableConvert::try_from_optioned(value.ip)?,
            ip_mode: crate::OptionableConvert::try_from_optioned(value.ip_mode)?,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: LoadBalancerIngressAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.hostname, other.hostname)?;
        crate::OptionableConvert::merge(&mut self.ip, other.ip)?;
        crate::OptionableConvert::merge(&mut self.ip_mode, other.ip_mode)?;
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
