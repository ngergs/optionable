pub struct NetworkPolicyIngressRuleOpt {
    pub from: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPeer>,
    > as crate::Optionable>::Optioned,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPort>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule {
    type Optioned = NetworkPolicyIngressRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyIngressRuleOpt {
    type Optioned = NetworkPolicyIngressRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule {
    fn into_optioned(self) -> NetworkPolicyIngressRuleOpt {
        NetworkPolicyIngressRuleOpt {
            from: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPeer>,
            > as crate::OptionableConvert>::into_optioned(self.from),
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPort>,
            > as crate::OptionableConvert>::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicyIngressRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            from: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPeer>,
            > as crate::OptionableConvert>::try_from_optioned(value.from)?,
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPort>,
            > as crate::OptionableConvert>::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: NetworkPolicyIngressRuleOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPeer>,
        > as crate::OptionableConvert>::merge(&mut self.from, other.from)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPort>,
        > as crate::OptionableConvert>::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
