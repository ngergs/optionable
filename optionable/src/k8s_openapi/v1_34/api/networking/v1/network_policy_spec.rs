pub struct NetworkPolicySpecOpt {
    pub egress: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule>,
    > as crate::Optionable>::Optioned,
    pub ingress: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule>,
    > as crate::Optionable>::Optioned,
    pub pod_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub policy_types: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicySpec {
    type Optioned = NetworkPolicySpecOpt;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicySpecOpt {
    type Optioned = NetworkPolicySpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::NetworkPolicySpec {
    fn into_optioned(self) -> NetworkPolicySpecOpt {
        NetworkPolicySpecOpt {
            egress: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule,
                >,
            > as crate::OptionableConvert>::into_optioned(self.egress),
            ingress: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule,
                >,
            > as crate::OptionableConvert>::into_optioned(self.ingress),
            pod_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::into_optioned(self.pod_selector),
            policy_types: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.policy_types),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicySpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            egress: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.egress)?,
            ingress: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.ingress)?,
            pod_selector: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.pod_selector)?,
            policy_types: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.policy_types)?,
        })
    }
    fn merge(
        &mut self,
        other: NetworkPolicySpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyEgressRule>,
        > as crate::OptionableConvert>::merge(&mut self.egress, other.egress)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::networking::v1::NetworkPolicyIngressRule>,
        > as crate::OptionableConvert>::merge(&mut self.ingress, other.ingress)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.pod_selector,
            other.pod_selector,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.policy_types,
            other.policy_types,
        )?;
        Ok(())
    }
}
