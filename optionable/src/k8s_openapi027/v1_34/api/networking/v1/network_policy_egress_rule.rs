#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and to. This type is beta-level in 1.8
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkPolicyEgressRuleAc {
    /// ports is a list of destination ports for outgoing traffic. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::NetworkPolicyPort as crate::Optionable>::Optioned,
        >,
    >,
    /// to is a list of destinations for outgoing traffic of pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all destinations (traffic not restricted by destination). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the to list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::NetworkPolicyPeer as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::NetworkPolicyEgressRule {
    type Optioned = NetworkPolicyEgressRuleAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyEgressRuleAc {
    type Optioned = NetworkPolicyEgressRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::NetworkPolicyEgressRule {
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
        if self.ports.is_none() {
            self.ports = crate::OptionableConvert::try_from_optioned(other.ports)?;
        } else if let Some(self_value) = self.ports.as_mut()
            && let Some(other_value) = other.ports
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.to.is_none() {
            self.to = crate::OptionableConvert::try_from_optioned(other.to)?;
        } else if let Some(self_value) = self.to.as_mut()
            && let Some(other_value) = other.to
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::NetworkPolicyEgressRule>
for NetworkPolicyEgressRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::NetworkPolicyEgressRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::networking::v1::NetworkPolicyEgressRule,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::NetworkPolicyEgressRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
