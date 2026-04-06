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
    pub bool: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta2::DeviceAttribute {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAttributeAc {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta2::DeviceAttribute {
    fn into_optioned(self) -> DeviceAttributeAc {
        DeviceAttributeAc {
            bool: self.bool,
            int: self.int,
            string: self.string,
            version: self.version,
        }
    }
    fn try_from_optioned(value: DeviceAttributeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            bool: value.bool,
            int: value.int,
            string: value.string,
            version: value.version,
        })
    }
    fn merge(&mut self, other: DeviceAttributeAc) -> Result<(), crate::Error> {
        self.bool = other.bool;
        self.int = other.int;
        self.string = other.string;
        self.version = other.version;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta2::DeviceAttribute>
for DeviceAttributeAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::DeviceAttribute,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta2::DeviceAttribute, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::DeviceAttribute,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
