#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Adapts a ConfigMap into a volume.
///
/// The contents of the target ConfigMap's Data field will be presented in a volume as files using the keys in the Data field as the file names, unless the items element is populated with specific mappings of keys to paths. ConfigMap volumes support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigMapVolumeSourceAc {
    /// defaultMode is optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// items if unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::KeyToPath as crate::Optionable>::Optioned,
        >,
    >,
    /// Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// optional specify whether the ConfigMap or its keys must be defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ConfigMapVolumeSource {
    type Optioned = ConfigMapVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapVolumeSourceAc {
    type Optioned = ConfigMapVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ConfigMapVolumeSource {
    fn into_optioned(self) -> ConfigMapVolumeSourceAc {
        ConfigMapVolumeSourceAc {
            default_mode: self.default_mode,
            items: crate::OptionableConvert::into_optioned(self.items),
            name: Some(self.name),
            optional: self.optional,
        }
    }
    fn try_from_optioned(value: ConfigMapVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            default_mode: value.default_mode,
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            optional: value.optional,
        })
    }
    fn merge(&mut self, other: ConfigMapVolumeSourceAc) -> Result<(), crate::Error> {
        if other.default_mode.is_some() {
            self.default_mode = other.default_mode;
        }
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if other.optional.is_some() {
            self.optional = other.optional;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ConfigMapVolumeSource>
for ConfigMapVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ConfigMapVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ConfigMapVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ConfigMapVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
