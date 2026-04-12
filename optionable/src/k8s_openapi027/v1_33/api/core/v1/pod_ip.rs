#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodIP represents a single IP address allocated to the pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodIPAc {
    /// IP is the IP address assigned to the pod
    pub ip: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodIP {
    type Optioned = PodIPAc;
}
#[automatically_derived]
impl crate::Optionable for PodIPAc {
    type Optioned = PodIPAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodIP {
    fn into_optioned(self) -> PodIPAc {
        PodIPAc { ip: self.ip }
    }
    fn try_from_optioned(value: PodIPAc) -> Result<Self, crate::Error> {
        Ok(Self { ip: value.ip })
    }
    fn merge(&mut self, other: PodIPAc) -> Result<(), crate::Error> {
        self.ip = other.ip;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::PodIP {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.ip == other.ip
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodIP> for PodIPAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodIP) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodIP, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodIP,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
