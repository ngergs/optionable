#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NetworkPolicySpec provides the specification of a NetworkPolicy
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkPolicySpecAc {
    /// egress is a list of egress rules to be applied to the selected pods. Outgoing traffic is allowed if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic matches at least one egress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy limits all outgoing traffic (and serves solely to ensure that the pods it selects are isolated by default). This field is beta-level in 1.8
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::NetworkPolicyEgressRule as crate::Optionable>::Optioned,
        >,
    >,
    /// ingress is a list of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::networking::v1::NetworkPolicyIngressRule as crate::Optionable>::Optioned,
        >,
    >,
    /// podSelector selects the pods to which this NetworkPolicy object applies. The array of rules is applied to any pods selected by this field. An empty selector matches all pods in the policy's namespace. Multiple network policies can select the same set of pods. In this case, the ingress rules for each are combined additively. This field is optional. If it is not specified, it defaults to an empty selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// policyTypes is a list of rule types that the NetworkPolicy relates to. Valid options are \["Ingress"\], \["Egress"\], or \["Ingress", "Egress"\]. If this field is not specified, it will default based on the existence of ingress or egress rules; policies that contain an egress section are assumed to affect egress, and all policies (whether or not they contain an ingress section) are assumed to affect ingress. If you want to write an egress-only policy, you must explicitly specify policyTypes \[ "Egress" \]. Likewise, if you want to write a policy that specifies that no egress is allowed, you must specify a policyTypes value that include "Egress" (since such a policy would not include an egress section and would otherwise default to just \[ "Ingress" \]). This field is beta-level in 1.8
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_types: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::NetworkPolicySpec {
    type Optioned = NetworkPolicySpecAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicySpecAc {
    type Optioned = NetworkPolicySpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::NetworkPolicySpec {
    fn into_optioned(self) -> NetworkPolicySpecAc {
        NetworkPolicySpecAc {
            egress: crate::OptionableConvert::into_optioned(self.egress),
            ingress: crate::OptionableConvert::into_optioned(self.ingress),
            pod_selector: crate::OptionableConvert::into_optioned(self.pod_selector),
            policy_types: self.policy_types,
        }
    }
    fn try_from_optioned(value: NetworkPolicySpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            egress: crate::OptionableConvert::try_from_optioned(value.egress)?,
            ingress: crate::OptionableConvert::try_from_optioned(value.ingress)?,
            pod_selector: crate::OptionableConvert::try_from_optioned(
                value.pod_selector,
            )?,
            policy_types: value.policy_types,
        })
    }
    fn merge(&mut self, other: NetworkPolicySpecAc) -> Result<(), crate::Error> {
        if self.egress.is_none() {
            self.egress = crate::OptionableConvert::try_from_optioned(other.egress)?;
        } else if let Some(self_value) = self.egress.as_mut()
            && let Some(other_value) = other.egress
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.ingress.is_none() {
            self.ingress = crate::OptionableConvert::try_from_optioned(other.ingress)?;
        } else if let Some(self_value) = self.ingress.as_mut()
            && let Some(other_value) = other.ingress
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
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
        if self.policy_types.is_none() {
            self.policy_types = crate::OptionableConvert::try_from_optioned(
                other.policy_types,
            )?;
        } else if let Some(self_value) = self.policy_types.as_mut()
            && let Some(other_value) = other.policy_types
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::NetworkPolicySpec>
for NetworkPolicySpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::NetworkPolicySpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::NetworkPolicySpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::NetworkPolicySpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
