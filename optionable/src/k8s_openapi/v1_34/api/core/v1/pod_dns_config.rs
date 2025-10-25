pub struct PodDNSConfigOpt {
    pub nameservers: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub options: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PodDNSConfigOption>,
    > as crate::Optionable>::Optioned,
    pub searches: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodDNSConfig {
    type Optioned = PodDNSConfigOpt;
}
#[automatically_derived]
impl crate::Optionable for PodDNSConfigOpt {
    type Optioned = PodDNSConfigOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodDNSConfig {
    fn into_optioned(self) -> PodDNSConfigOpt {
        PodDNSConfigOpt {
            nameservers: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.nameservers),
            options: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodDNSConfigOption>,
            > as crate::OptionableConvert>::into_optioned(self.options),
            searches: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.searches),
        }
    }
    fn try_from_optioned(
        value: PodDNSConfigOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            nameservers: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.nameservers)?,
            options: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::PodDNSConfigOption>,
            > as crate::OptionableConvert>::try_from_optioned(value.options)?,
            searches: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.searches)?,
        })
    }
    fn merge(&mut self, other: PodDNSConfigOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.nameservers, other.nameservers)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::PodDNSConfigOption>,
        > as crate::OptionableConvert>::merge(&mut self.options, other.options)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.searches, other.searches)?;
        Ok(())
    }
}
