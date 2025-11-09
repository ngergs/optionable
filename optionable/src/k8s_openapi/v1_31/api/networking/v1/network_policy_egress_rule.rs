#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyEgressRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPort>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPeer>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule {
    type Optioned = NetworkPolicyEgressRuleAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyEgressRuleAc {
    type Optioned = NetworkPolicyEgressRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule {
    fn into_optioned(self) -> NetworkPolicyEgressRuleAc {
        NetworkPolicyEgressRuleAc {
            ports: crate::OptionableConvert::into_optioned(self.ports),
            to: crate::OptionableConvert::into_optioned(self.to),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicyEgressRuleAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
            to: crate::OptionableConvert::try_from_optioned(value.to)?,
        })
    }
    fn merge(&mut self, other: NetworkPolicyEgressRuleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        crate::OptionableConvert::merge(&mut self.to, other.to)?;
        Ok(())
    }
}
