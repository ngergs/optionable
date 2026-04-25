#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Selects a key from a ConfigMap.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigMapKeySelectorAc {
    /// The key to select.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    /// Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ConfigMapKeySelector {
    type Optioned = ConfigMapKeySelectorAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapKeySelectorAc {
    type Optioned = ConfigMapKeySelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ConfigMapKeySelector {
    fn into_optioned(self) -> ConfigMapKeySelectorAc {
        ConfigMapKeySelectorAc {
            key: Some(self.key),
            name: Some(self.name),
            optional: self.optional,
        }
    }
    fn try_from_optioned(value: ConfigMapKeySelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            key: value
                .key
                .ok_or(crate::Error {
                    missing_field: "key",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            optional: value.optional,
        })
    }
    fn merge(&mut self, other: ConfigMapKeySelectorAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.optional.is_none() {
            self.optional = crate::OptionableConvert::try_from_optioned(other.optional)?;
        } else if let Some(self_value) = self.optional.as_mut()
            && let Some(other_value) = other.optional
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ConfigMapKeySelector>
for ConfigMapKeySelectorAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ConfigMapKeySelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ConfigMapKeySelector, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ConfigMapKeySelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ConfigMapKeySelectorAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.key, other.key);
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.optional, other.optional);
    }
}
