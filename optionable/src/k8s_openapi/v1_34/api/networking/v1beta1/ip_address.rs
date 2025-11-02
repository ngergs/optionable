#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = IPAddress)]
pub struct IPAddressAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::networking::v1beta1::IPAddressSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1beta1::IPAddress {
    type Optioned = IPAddressAc;
}
#[automatically_derived]
impl crate::Optionable for IPAddressAc {
    type Optioned = IPAddressAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1beta1::IPAddress {
    fn into_optioned(self) -> IPAddressAc {
        IPAddressAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(value: IPAddressAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: IPAddressAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::networking::v1beta1::IPAddress;
