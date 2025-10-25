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
            add: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.add),
            drop: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.drop),
        }
    }
    fn try_from_optioned(
        value: CapabilitiesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            add: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.add)?,
            drop: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.drop)?,
        })
    }
    fn merge(&mut self, other: CapabilitiesOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.add, other.add)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.drop, other.drop)?;
        Ok(())
    }
}
