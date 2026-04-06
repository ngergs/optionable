#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretEnvSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::SecretEnvSource {
    type Optioned = SecretEnvSourceAc;
}
#[automatically_derived]
impl crate::Optionable for SecretEnvSourceAc {
    type Optioned = SecretEnvSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::SecretEnvSource {
    fn into_optioned(self) -> SecretEnvSourceAc {
        SecretEnvSourceAc {
            name: Some(self.name),
            optional: self.optional,
        }
    }
    fn try_from_optioned(value: SecretEnvSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            optional: value.optional,
        })
    }
    fn merge(&mut self, other: SecretEnvSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.optional = other.optional;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::SecretEnvSource>
for SecretEnvSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::SecretEnvSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::SecretEnvSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::SecretEnvSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
