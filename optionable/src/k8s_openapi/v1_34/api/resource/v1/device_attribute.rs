pub struct DeviceAttributeOpt {
    pub bool: <Option<bool> as crate::Optionable>::Optioned,
    pub int: <Option<i64> as crate::Optionable>::Optioned,
    pub string: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceAttribute {
    type Optioned = DeviceAttributeOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceAttributeOpt {
    type Optioned = DeviceAttributeOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::DeviceAttribute {
    fn into_optioned(self) -> DeviceAttributeOpt {
        DeviceAttributeOpt {
            bool: <Option<bool> as crate::OptionableConvert>::into_optioned(self.bool),
            int: <Option<i64> as crate::OptionableConvert>::into_optioned(self.int),
            string: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.string),
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: DeviceAttributeOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            bool: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.bool)?,
            int: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.int)?,
            string: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.string)?,
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.version)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAttributeOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<bool> as crate::OptionableConvert>::merge(&mut self.bool, other.bool)?;
        <Option<i64> as crate::OptionableConvert>::merge(&mut self.int, other.int)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.string, other.string)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
