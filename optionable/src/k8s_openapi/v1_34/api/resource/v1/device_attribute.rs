#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DeviceAttributeAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::DeviceAttribute {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAttributeAc {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::DeviceAttribute {
    fn into_optioned(self) -> DeviceAttributeAc {
        DeviceAttributeAc {
            bool: crate::OptionableConvert::into_optioned(self.bool),
            int: crate::OptionableConvert::into_optioned(self.int),
            string: crate::OptionableConvert::into_optioned(self.string),
            version: crate::OptionableConvert::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: DeviceAttributeAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            bool: crate::OptionableConvert::try_from_optioned(value.bool)?,
            int: crate::OptionableConvert::try_from_optioned(value.int)?,
            string: crate::OptionableConvert::try_from_optioned(value.string)?,
            version: crate::OptionableConvert::try_from_optioned(value.version)?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceAttributeAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.bool, other.bool)?;
        crate::OptionableConvert::merge(&mut self.int, other.int)?;
        crate::OptionableConvert::merge(&mut self.string, other.string)?;
        crate::OptionableConvert::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
