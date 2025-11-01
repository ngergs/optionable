pub struct NetworkPolicyPortAc {
    pub end_port: <Option<i32> as crate::Optionable>::Optioned,
    pub port: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicyPort {
    type Optioned = NetworkPolicyPortAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyPortAc {
    type Optioned = NetworkPolicyPortAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::NetworkPolicyPort {
    fn into_optioned(self) -> NetworkPolicyPortAc {
        NetworkPolicyPortAc {
            end_port: crate::OptionableConvert::into_optioned(self.end_port),
            port: crate::OptionableConvert::into_optioned(self.port),
            protocol: crate::OptionableConvert::into_optioned(self.protocol),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicyPortAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            end_port: crate::OptionableConvert::try_from_optioned(value.end_port)?,
            port: crate::OptionableConvert::try_from_optioned(value.port)?,
            protocol: crate::OptionableConvert::try_from_optioned(value.protocol)?,
        })
    }
    fn merge(
        &mut self,
        other: NetworkPolicyPortAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.end_port, other.end_port)?;
        crate::OptionableConvert::merge(&mut self.port, other.port)?;
        crate::OptionableConvert::merge(&mut self.protocol, other.protocol)?;
        Ok(())
    }
}
