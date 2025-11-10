#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicySpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_types: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicySpec {
    type Optioned = NetworkPolicySpecAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicySpecAc {
    type Optioned = NetworkPolicySpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::NetworkPolicySpec {
    fn into_optioned(self) -> NetworkPolicySpecAc {
        NetworkPolicySpecAc {
            egress: crate::OptionableConvert::into_optioned(self.egress),
            ingress: crate::OptionableConvert::into_optioned(self.ingress),
            pod_selector: crate::OptionableConvert::into_optioned(self.pod_selector),
            policy_types: crate::OptionableConvert::into_optioned(self.policy_types),
        }
    }
    fn try_from_optioned(value: NetworkPolicySpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            egress: crate::OptionableConvert::try_from_optioned(value.egress)?,
            ingress: crate::OptionableConvert::try_from_optioned(value.ingress)?,
            pod_selector: crate::OptionableConvert::try_from_optioned(
                value.pod_selector,
            )?,
            policy_types: crate::OptionableConvert::try_from_optioned(
                value.policy_types,
            )?,
        })
    }
    fn merge(&mut self, other: NetworkPolicySpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.egress, other.egress)?;
        crate::OptionableConvert::merge(&mut self.ingress, other.ingress)?;
        crate::OptionableConvert::merge(&mut self.pod_selector, other.pod_selector)?;
        crate::OptionableConvert::merge(&mut self.policy_types, other.policy_types)?;
        Ok(())
    }
}
