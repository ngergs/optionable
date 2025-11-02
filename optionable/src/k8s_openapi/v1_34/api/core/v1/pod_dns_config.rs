#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodDNSConfigAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodDNSConfigOption>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searches: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodDNSConfig {
    type Optioned = PodDNSConfigAc;
}
#[automatically_derived]
impl crate::Optionable for PodDNSConfigAc {
    type Optioned = PodDNSConfigAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodDNSConfig {
    fn into_optioned(self) -> PodDNSConfigAc {
        PodDNSConfigAc {
            nameservers: crate::OptionableConvert::into_optioned(self.nameservers),
            options: crate::OptionableConvert::into_optioned(self.options),
            searches: crate::OptionableConvert::into_optioned(self.searches),
        }
    }
    fn try_from_optioned(
        value: PodDNSConfigAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            nameservers: crate::OptionableConvert::try_from_optioned(value.nameservers)?,
            options: crate::OptionableConvert::try_from_optioned(value.options)?,
            searches: crate::OptionableConvert::try_from_optioned(value.searches)?,
        })
    }
    fn merge(&mut self, other: PodDNSConfigAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.nameservers, other.nameservers)?;
        crate::OptionableConvert::merge(&mut self.options, other.options)?;
        crate::OptionableConvert::merge(&mut self.searches, other.searches)?;
        Ok(())
    }
}
