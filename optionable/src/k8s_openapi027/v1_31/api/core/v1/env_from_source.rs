#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EnvFromSource represents the source of a set of ConfigMaps
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvFromSourceAc {
    /// The ConfigMap to select from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map_ref: Option<
        <::k8s_openapi027::api::core::v1::ConfigMapEnvSource as crate::Optionable>::Optioned,
    >,
    /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<std::string::String>,
    /// The Secret to select from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretEnvSource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EnvFromSource {
    type Optioned = EnvFromSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EnvFromSourceAc {
    type Optioned = EnvFromSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EnvFromSource {
    fn into_optioned(self) -> EnvFromSourceAc {
        EnvFromSourceAc {
            config_map_ref: crate::OptionableConvert::into_optioned(self.config_map_ref),
            prefix: self.prefix,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
        }
    }
    fn try_from_optioned(value: EnvFromSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config_map_ref: crate::OptionableConvert::try_from_optioned(
                value.config_map_ref,
            )?,
            prefix: value.prefix,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
        })
    }
    fn merge(&mut self, other: EnvFromSourceAc) -> Result<(), crate::Error> {
        if self.config_map_ref.is_none() {
            self.config_map_ref = crate::OptionableConvert::try_from_optioned(
                other.config_map_ref,
            )?;
        } else if let Some(self_value) = self.config_map_ref.as_mut()
            && let Some(other_value) = other.config_map_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.prefix.is_none() {
            self.prefix = crate::OptionableConvert::try_from_optioned(other.prefix)?;
        } else if let Some(self_value) = self.prefix.as_mut()
            && let Some(other_value) = other.prefix
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.secret_ref.is_none() {
            self.secret_ref = crate::OptionableConvert::try_from_optioned(
                other.secret_ref,
            )?;
        } else if let Some(self_value) = self.secret_ref.as_mut()
            && let Some(other_value) = other.secret_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EnvFromSource>
for EnvFromSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::EnvFromSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EnvFromSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EnvFromSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
