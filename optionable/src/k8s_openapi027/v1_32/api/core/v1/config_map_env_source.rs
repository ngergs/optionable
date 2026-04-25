#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ConfigMapEnvSource selects a ConfigMap to populate the environment variables with.
///
/// The contents of the target ConfigMap's Data field will represent the key-value pairs as environment variables.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigMapEnvSourceAc {
    /// Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Specify whether the ConfigMap must be defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ConfigMapEnvSource {
    type Optioned = ConfigMapEnvSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapEnvSourceAc {
    type Optioned = ConfigMapEnvSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ConfigMapEnvSource {
    fn into_optioned(self) -> ConfigMapEnvSourceAc {
        ConfigMapEnvSourceAc {
            name: Some(self.name),
            optional: self.optional,
        }
    }
    fn try_from_optioned(value: ConfigMapEnvSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            optional: value.optional,
        })
    }
    fn merge(&mut self, other: ConfigMapEnvSourceAc) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ConfigMapEnvSource>
for ConfigMapEnvSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ConfigMapEnvSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ConfigMapEnvSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ConfigMapEnvSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ConfigMapEnvSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.optional, other.optional);
    }
}
