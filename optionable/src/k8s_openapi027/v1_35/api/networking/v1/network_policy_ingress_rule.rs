#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NetworkPolicyIngressRule describes a particular set of traffic that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and from.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkPolicyIngressRuleAc {
    /// from is a list of sources which should be able to access the pods selected for this rule. Items in this list are combined using a logical OR operation. If this field is empty or missing, this rule matches all sources (traffic not restricted by source). If this field is present and contains at least one item, this rule allows traffic only if the traffic matches at least one item in the from list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::NetworkPolicyPeer as crate::Optionable>::Optioned,
        >,
    >,
    /// ports is a list of ports which should be made accessible on the pods selected for this rule. Each item in this list is combined using a logical OR. If this field is empty or missing, this rule matches all ports (traffic not restricted by port). If this field is present and contains at least one item, then this rule allows traffic only if the traffic matches at least one port in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::NetworkPolicyPort as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::networking::v1::NetworkPolicyIngressRule {
    type Optioned = NetworkPolicyIngressRuleAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyIngressRuleAc {
    type Optioned = NetworkPolicyIngressRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::NetworkPolicyIngressRule {
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
        if self.from.is_none() {
            self.from = crate::OptionableConvert::try_from_optioned(other.from)?;
        } else if let Some(self_value) = self.from.as_mut()
            && let Some(other_value) = other.from
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.ports.is_none() {
            self.ports = crate::OptionableConvert::try_from_optioned(other.ports)?;
        } else if let Some(self_value) = self.ports.as_mut()
            && let Some(other_value) = other.ports
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::networking::v1::NetworkPolicyIngressRule,
> for NetworkPolicyIngressRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::NetworkPolicyIngressRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::networking::v1::NetworkPolicyIngressRule,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::NetworkPolicyIngressRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for NetworkPolicyIngressRuleAc {
    fn merge_from(&mut self, other: Self) {
        self.from = other.from;
        self.ports = other.ports;
    }
}
