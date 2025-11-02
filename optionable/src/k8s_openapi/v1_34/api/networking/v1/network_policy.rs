#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = NetworkPolicy)]
pub struct NetworkPolicyAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::networking::v1::NetworkPolicySpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicy {
    type Optioned = NetworkPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyAc {
    type Optioned = NetworkPolicyAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::NetworkPolicy {
    fn into_optioned(self) -> NetworkPolicyAc {
        NetworkPolicyAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: NetworkPolicyAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: NetworkPolicyAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::networking::v1::NetworkPolicy;
