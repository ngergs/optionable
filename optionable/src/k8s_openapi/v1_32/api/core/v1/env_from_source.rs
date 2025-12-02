#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct EnvFromSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map_ref: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapEnvSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretEnvSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EnvFromSource {
    type Optioned = EnvFromSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EnvFromSourceAc {
    type Optioned = EnvFromSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EnvFromSource {
    fn into_optioned(self) -> EnvFromSourceAc {
        EnvFromSourceAc {
            config_map_ref: crate::OptionableConvert::into_optioned(self.config_map_ref),
            prefix: crate::OptionableConvert::into_optioned(self.prefix),
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
        }
    }
    fn try_from_optioned(value: EnvFromSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config_map_ref: crate::OptionableConvert::try_from_optioned(
                value.config_map_ref,
            )?,
            prefix: crate::OptionableConvert::try_from_optioned(value.prefix)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
        })
    }
    fn merge(&mut self, other: EnvFromSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.config_map_ref, other.config_map_ref)?;
        crate::OptionableConvert::merge(&mut self.prefix, other.prefix)?;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::EnvFromSource>
for EnvFromSourceAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::EnvFromSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::EnvFromSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::EnvFromSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
