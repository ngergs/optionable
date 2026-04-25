#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceAttribute must have exactly one field set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceAttributeAc {
    /// BoolValue is a true/false value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    /// IntValue is a number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    /// StringValue is a string. Must not be longer than 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<std::string::String>,
    /// VersionValue is a semantic version according to semver.org spec 2.0.0. Must not be longer than 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta1::DeviceAttribute {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAttributeAc {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::DeviceAttribute {
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
        if self.bool.is_none() {
            self.bool = crate::OptionableConvert::try_from_optioned(other.bool)?;
        } else if let Some(self_value) = self.bool.as_mut()
            && let Some(other_value) = other.bool
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.int.is_none() {
            self.int = crate::OptionableConvert::try_from_optioned(other.int)?;
        } else if let Some(self_value) = self.int.as_mut()
            && let Some(other_value) = other.int
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.string.is_none() {
            self.string = crate::OptionableConvert::try_from_optioned(other.string)?;
        } else if let Some(self_value) = self.string.as_mut()
            && let Some(other_value) = other.string
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.version.is_none() {
            self.version = crate::OptionableConvert::try_from_optioned(other.version)?;
        } else if let Some(self_value) = self.version.as_mut()
            && let Some(other_value) = other.version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta1::DeviceAttribute>
for DeviceAttributeAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::DeviceAttribute,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta1::DeviceAttribute, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::DeviceAttribute,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeviceAttributeAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.bool, other.bool);
        k8s_openapi027::DeepMerge::merge_from(&mut self.int, other.int);
        k8s_openapi027::DeepMerge::merge_from(&mut self.string, other.string);
        k8s_openapi027::DeepMerge::merge_from(&mut self.version, other.version);
    }
}
