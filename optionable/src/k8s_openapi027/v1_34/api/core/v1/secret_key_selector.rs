#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretKeySelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::SecretKeySelector {
    type Optioned = SecretKeySelectorAc;
}
#[automatically_derived]
impl crate::Optionable for SecretKeySelectorAc {
    type Optioned = SecretKeySelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::SecretKeySelector {
    fn into_optioned(self) -> SecretKeySelectorAc {
        SecretKeySelectorAc {
            key: Some(self.key),
            name: Some(self.name),
            optional: self.optional,
        }
    }
    fn try_from_optioned(value: SecretKeySelectorAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: SecretKeySelectorAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.optional = other.optional;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::SecretKeySelector>
for SecretKeySelectorAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::SecretKeySelector) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::SecretKeySelector, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::SecretKeySelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
