pub struct PodDNSConfigOptionOpt {
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodDNSConfigOption {
    type Optioned = PodDNSConfigOptionOpt;
}
#[automatically_derived]
impl crate::Optionable for PodDNSConfigOptionOpt {
    type Optioned = PodDNSConfigOptionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodDNSConfigOption {
    fn into_optioned(self) -> PodDNSConfigOptionOpt {
        PodDNSConfigOptionOpt {
            name: crate::OptionableConvert::into_optioned(self.name),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(
        value: PodDNSConfigOptionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(
        &mut self,
        other: PodDNSConfigOptionOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
