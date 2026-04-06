#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FileKeySelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::FileKeySelector {
    type Optioned = FileKeySelectorAc;
}
#[automatically_derived]
impl crate::Optionable for FileKeySelectorAc {
    type Optioned = FileKeySelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::FileKeySelector {
    fn into_optioned(self) -> FileKeySelectorAc {
        FileKeySelectorAc {
            key: Some(self.key),
            optional: self.optional,
            path: Some(self.path),
            volume_name: Some(self.volume_name),
        }
    }
    fn try_from_optioned(value: FileKeySelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            key: value
                .key
                .ok_or(crate::Error {
                    missing_field: "key",
                })?,
            optional: value.optional,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            volume_name: value
                .volume_name
                .ok_or(crate::Error {
                    missing_field: "volume_name",
                })?,
        })
    }
    fn merge(&mut self, other: FileKeySelectorAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = other_value;
        }
        self.optional = other.optional;
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        if let Some(other_value) = other.volume_name {
            self.volume_name = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::FileKeySelector>
for FileKeySelectorAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::FileKeySelector) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::FileKeySelector, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::FileKeySelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
