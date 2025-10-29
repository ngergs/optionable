pub struct NetworkPolicyEgressRuleOpt {
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPort>,
    > as crate::Optionable>::Optioned,
    pub to: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPeer>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule {
    type Optioned = NetworkPolicyEgressRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyEgressRuleOpt {
    type Optioned = NetworkPolicyEgressRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule {
    fn into_optioned(self) -> NetworkPolicyEgressRuleOpt {
        NetworkPolicyEgressRuleOpt {
            ports: crate::OptionableConvert::into_optioned(self.ports),
            to: crate::OptionableConvert::into_optioned(self.to),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicyEgressRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
            to: crate::OptionableConvert::try_from_optioned(value.to)?,
        })
    }
    fn merge(
        &mut self,
        other: NetworkPolicyEgressRuleOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        crate::OptionableConvert::merge(&mut self.to, other.to)?;
        Ok(())
    }
}
