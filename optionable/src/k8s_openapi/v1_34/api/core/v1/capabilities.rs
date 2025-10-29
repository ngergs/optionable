pub struct CapabilitiesOpt {
    pub add: <Option<std::vec::Vec<std::string::String>> as crate::Optionable>::Optioned,
    pub drop: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Capabilities {
    type Optioned = CapabilitiesOpt;
}
#[automatically_derived]
impl crate::Optionable for CapabilitiesOpt {
    type Optioned = CapabilitiesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Capabilities {
    fn into_optioned(self) -> CapabilitiesOpt {
        CapabilitiesOpt {
            add: crate::OptionableConvert::into_optioned(self.add),
            drop: crate::OptionableConvert::into_optioned(self.drop),
        }
    }
    fn try_from_optioned(
        value: CapabilitiesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            add: crate::OptionableConvert::try_from_optioned(value.add)?,
            drop: crate::OptionableConvert::try_from_optioned(value.drop)?,
        })
    }
    fn merge(&mut self, other: CapabilitiesOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.add, other.add)?;
        crate::OptionableConvert::merge(&mut self.drop, other.drop)?;
        Ok(())
    }
}
