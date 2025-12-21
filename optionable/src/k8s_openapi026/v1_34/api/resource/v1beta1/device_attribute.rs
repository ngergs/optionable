#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
impl crate::Optionable for k8s_openapi026::api::resource::v1beta1::DeviceAttribute {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAttributeAc {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1beta1::DeviceAttribute {
    fn into_optioned(self) -> DeviceAttributeAc {
        DeviceAttributeAc {
            bool: crate::OptionableConvert::into_optioned(self.bool),
            int: crate::OptionableConvert::into_optioned(self.int),
            string: crate::OptionableConvert::into_optioned(self.string),
            version: crate::OptionableConvert::into_optioned(self.version),
        }
    }
    fn try_from_optioned(value: DeviceAttributeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            bool: crate::OptionableConvert::try_from_optioned(value.bool)?,
            int: crate::OptionableConvert::try_from_optioned(value.int)?,
            string: crate::OptionableConvert::try_from_optioned(value.string)?,
            version: crate::OptionableConvert::try_from_optioned(value.version)?,
        })
    }
    fn merge(&mut self, other: DeviceAttributeAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.bool, other.bool)?;
        crate::OptionableConvert::merge(&mut self.int, other.int)?;
        crate::OptionableConvert::merge(&mut self.string, other.string)?;
        crate::OptionableConvert::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::resource::v1beta1::DeviceAttribute>
for DeviceAttributeAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1beta1::DeviceAttribute,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::resource::v1beta1::DeviceAttribute, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1beta1::DeviceAttribute,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
