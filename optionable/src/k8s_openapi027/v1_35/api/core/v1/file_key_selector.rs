#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FileKeySelector selects a key of the env file.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FileKeySelectorAc {
    /// The key within the env file. An invalid key will prevent the pod from starting. The keys defined within a source may consist of any printable ASCII characters except '='. During Alpha stage of the EnvFiles feature gate, the key size is limited to 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    /// Specify whether the file or its key must be defined. If the file or key does not exist, then the env var is not published. If optional is set to true and the specified key does not exist, the environment variable will not be set in the Pod's containers.
    ///
    /// If optional is set to false and the specified key does not exist, an error will be returned during Pod creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// The path within the volume from which to select the file. Must be relative and may not contain the '..' path or start with '..'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// The name of the volume mount containing the env file.
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
            self.key = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.optional.is_none() {
            self.optional = crate::OptionableConvert::try_from_optioned(other.optional)?;
        } else {
            crate::OptionableConvert::merge(&mut self.optional, other.optional)?;
        }
        if let Some(other_value) = other.path {
            self.path = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.volume_name {
            self.volume_name = crate::OptionableConvert::try_from_optioned(other_value)?;
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
