#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodDNSConfigOptionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodDNSConfigOption {
    type Optioned = PodDNSConfigOptionAc;
}
#[automatically_derived]
impl crate::Optionable for PodDNSConfigOptionAc {
    type Optioned = PodDNSConfigOptionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodDNSConfigOption {
    fn into_optioned(self) -> PodDNSConfigOptionAc {
        PodDNSConfigOptionAc {
            name: crate::OptionableConvert::into_optioned(self.name),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(value: PodDNSConfigOptionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: PodDNSConfigOptionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodDNSConfigOption>
for PodDNSConfigOptionAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PodDNSConfigOption,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodDNSConfigOption, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodDNSConfigOption,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
