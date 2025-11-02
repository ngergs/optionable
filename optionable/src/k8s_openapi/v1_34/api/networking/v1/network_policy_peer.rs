#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyPeerAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_block: <Option<
        ::k8s_openapi::api::networking::v1::IPBlock,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicyPeer {
    type Optioned = NetworkPolicyPeerAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyPeerAc {
    type Optioned = NetworkPolicyPeerAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::NetworkPolicyPeer {
    fn into_optioned(self) -> NetworkPolicyPeerAc {
        NetworkPolicyPeerAc {
            ip_block: crate::OptionableConvert::into_optioned(self.ip_block),
            namespace_selector: crate::OptionableConvert::into_optioned(
                self.namespace_selector,
            ),
            pod_selector: crate::OptionableConvert::into_optioned(self.pod_selector),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicyPeerAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ip_block: crate::OptionableConvert::try_from_optioned(value.ip_block)?,
            namespace_selector: crate::OptionableConvert::try_from_optioned(
                value.namespace_selector,
            )?,
            pod_selector: crate::OptionableConvert::try_from_optioned(
                value.pod_selector,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NetworkPolicyPeerAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.ip_block, other.ip_block)?;
        crate::OptionableConvert::merge(
            &mut self.namespace_selector,
            other.namespace_selector,
        )?;
        crate::OptionableConvert::merge(&mut self.pod_selector, other.pod_selector)?;
        Ok(())
    }
}
