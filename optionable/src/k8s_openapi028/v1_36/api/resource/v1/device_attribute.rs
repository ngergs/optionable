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
    /// BoolValues is a non-empty list of true/false values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bools: Option<std::vec::Vec<bool>>,
    /// IntValue is a number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    /// IntValues is a non-empty list of numbers.
    ///
    /// This is an alpha field and requires enabling the DRAListTypeAttributes feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ints: Option<std::vec::Vec<i64>>,
    /// StringValue is a string. Must not be longer than 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<std::string::String>,
    /// StringValues is a non-empty list of strings. Each string must not be longer than 64 characters.
    ///
    /// This is an alpha field and requires enabling the DRAListTypeAttributes feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strings: Option<std::vec::Vec<std::string::String>>,
    /// VersionValue is a semantic version according to semver.org spec 2.0.0. Must not be longer than 64 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<std::string::String>,
    /// VersionValues is a non-empty list of semantic versions according to semver.org spec 2.0.0. Each version string must not be longer than 64 characters.
    ///
    /// This is an alpha field and requires enabling the DRAListTypeAttributes feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::api::resource::v1::DeviceAttribute {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAttributeAc {
    type Optioned = DeviceAttributeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi028::api::resource::v1::DeviceAttribute {
    fn into_optioned(self) -> DeviceAttributeAc {
        DeviceAttributeAc {
            bool: self.bool,
            bools: self.bools,
            int: self.int,
            ints: self.ints,
            string: self.string,
            strings: self.strings,
            version: self.version,
            versions: self.versions,
        }
    }
    fn try_from_optioned(value: DeviceAttributeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            bool: value.bool,
            bools: value.bools,
            int: value.int,
            ints: value.ints,
            string: value.string,
            strings: value.strings,
            version: value.version,
            versions: value.versions,
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
        if self.bools.is_none() {
            self.bools = crate::OptionableConvert::try_from_optioned(other.bools)?;
        } else if let Some(self_value) = self.bools.as_mut()
            && let Some(other_value) = other.bools
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
        if self.ints.is_none() {
            self.ints = crate::OptionableConvert::try_from_optioned(other.ints)?;
        } else if let Some(self_value) = self.ints.as_mut()
            && let Some(other_value) = other.ints
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
        if self.strings.is_none() {
            self.strings = crate::OptionableConvert::try_from_optioned(other.strings)?;
        } else if let Some(self_value) = self.strings.as_mut()
            && let Some(other_value) = other.strings
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
        if self.versions.is_none() {
            self.versions = crate::OptionableConvert::try_from_optioned(other.versions)?;
        } else if let Some(self_value) = self.versions.as_mut()
            && let Some(other_value) = other.versions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::api::resource::v1::DeviceAttribute>
for DeviceAttributeAc {
    fn from_optionable(
        value: k8s_openapi028::api::resource::v1::DeviceAttribute,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi028::api::resource::v1::DeviceAttribute, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::resource::v1::DeviceAttribute,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for DeviceAttributeAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.bool, other.bool);
        k8s_openapi028::DeepMerge::merge_from(&mut self.bools, other.bools);
        k8s_openapi028::DeepMerge::merge_from(&mut self.int, other.int);
        k8s_openapi028::DeepMerge::merge_from(&mut self.ints, other.ints);
        k8s_openapi028::DeepMerge::merge_from(&mut self.string, other.string);
        k8s_openapi028::DeepMerge::merge_from(&mut self.strings, other.strings);
        k8s_openapi028::DeepMerge::merge_from(&mut self.version, other.version);
        k8s_openapi028::DeepMerge::merge_from(&mut self.versions, other.versions);
    }
}
