#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyIngressRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPeer>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyPort>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule {
    type Optioned = NetworkPolicyIngressRuleAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyIngressRuleAc {
    type Optioned = NetworkPolicyIngressRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule {
    fn into_optioned(self) -> NetworkPolicyIngressRuleAc {
        NetworkPolicyIngressRuleAc {
            from: crate::OptionableConvert::into_optioned(self.from),
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicyIngressRuleAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            from: crate::OptionableConvert::try_from_optioned(value.from)?,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(&mut self, other: NetworkPolicyIngressRuleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.from, other.from)?;
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule>
for NetworkPolicyIngressRuleAc {
    fn from_optionable(
        value: ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
