#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodSchedulingGate is associated to a Pod to guard its scheduling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodSchedulingGateAc {
    /// Name of the scheduling gate. Each scheduling gate must have a unique name field.
    pub name: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodSchedulingGate {
    type Optioned = PodSchedulingGateAc;
}
#[automatically_derived]
impl crate::Optionable for PodSchedulingGateAc {
    type Optioned = PodSchedulingGateAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodSchedulingGate {
    fn into_optioned(self) -> PodSchedulingGateAc {
        PodSchedulingGateAc {
            name: self.name,
        }
    }
    fn try_from_optioned(value: PodSchedulingGateAc) -> Result<Self, crate::Error> {
        Ok(Self { name: value.name })
    }
    fn merge(&mut self, other: PodSchedulingGateAc) -> Result<(), crate::Error> {
        self.name = other.name;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::core::v1::PodSchedulingGate {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodSchedulingGate>
for PodSchedulingGateAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodSchedulingGate) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodSchedulingGate, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodSchedulingGate,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
