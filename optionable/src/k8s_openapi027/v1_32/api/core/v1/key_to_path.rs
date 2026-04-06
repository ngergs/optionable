#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct KeyToPathAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::KeyToPath {
    type Optioned = KeyToPathAc;
}
#[automatically_derived]
impl crate::Optionable for KeyToPathAc {
    type Optioned = KeyToPathAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::KeyToPath {
    fn into_optioned(self) -> KeyToPathAc {
        KeyToPathAc {
            key: Some(self.key),
            mode: self.mode,
            path: Some(self.path),
        }
    }
    fn try_from_optioned(value: KeyToPathAc) -> Result<Self, crate::Error> {
        Ok(Self {
            key: value
                .key
                .ok_or(crate::Error {
                    missing_field: "key",
                })?,
            mode: value.mode,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
        })
    }
    fn merge(&mut self, other: KeyToPathAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = other_value;
        }
        self.mode = other.mode;
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::KeyToPath> for KeyToPathAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::KeyToPath) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::KeyToPath, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::KeyToPath,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
