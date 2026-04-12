#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NetworkPolicyPeer describes a peer to allow traffic to/from. Only certain combinations of fields are allowed
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkPolicyPeerAc {
    /// ipBlock defines policy on a particular IPBlock. If this field is set then neither of the other fields can be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_block: Option<
        <::k8s_openapi027::api::networking::v1::IPBlock as crate::Optionable>::Optioned,
    >,
    /// namespaceSelector selects namespaces using cluster-scoped labels. This field follows standard label selector semantics; if present but empty, it selects all namespaces.
    ///
    /// If podSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the namespaces selected by namespaceSelector. Otherwise it selects all pods in the namespaces selected by namespaceSelector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// podSelector is a label selector which selects pods. This field follows standard label selector semantics; if present but empty, it selects all pods.
    ///
    /// If namespaceSelector is also set, then the NetworkPolicyPeer as a whole selects the pods matching podSelector in the Namespaces selected by NamespaceSelector. Otherwise it selects the pods matching podSelector in the policy's own namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::NetworkPolicyPeer {
    type Optioned = NetworkPolicyPeerAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyPeerAc {
    type Optioned = NetworkPolicyPeerAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::NetworkPolicyPeer {
    fn into_optioned(self) -> NetworkPolicyPeerAc {
        NetworkPolicyPeerAc {
            ip_block: crate::OptionableConvert::into_optioned(self.ip_block),
            namespace_selector: crate::OptionableConvert::into_optioned(
                self.namespace_selector,
            ),
            pod_selector: crate::OptionableConvert::into_optioned(self.pod_selector),
        }
    }
    fn try_from_optioned(value: NetworkPolicyPeerAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: NetworkPolicyPeerAc) -> Result<(), crate::Error> {
        if self.ip_block.is_none() {
            self.ip_block = crate::OptionableConvert::try_from_optioned(other.ip_block)?;
        } else if let Some(self_value) = self.ip_block.as_mut()
            && let Some(other_value) = other.ip_block
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.namespace_selector.is_none() {
            self.namespace_selector = crate::OptionableConvert::try_from_optioned(
                other.namespace_selector,
            )?;
        } else if let Some(self_value) = self.namespace_selector.as_mut()
            && let Some(other_value) = other.namespace_selector
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pod_selector.is_none() {
            self.pod_selector = crate::OptionableConvert::try_from_optioned(
                other.pod_selector,
            )?;
        } else if let Some(self_value) = self.pod_selector.as_mut()
            && let Some(other_value) = other.pod_selector
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::NetworkPolicyPeer>
for NetworkPolicyPeerAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::NetworkPolicyPeer,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::NetworkPolicyPeer, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::NetworkPolicyPeer,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
