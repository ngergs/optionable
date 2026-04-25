#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Adapts a Secret into a volume.
///
/// The contents of the target Secret's Data field will be presented in a volume as files using the keys in the Data field as the file names. Secret volumes support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretVolumeSourceAc {
    /// defaultMode is Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// items If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::KeyToPath as crate::Optionable>::Optioned,
        >,
    >,
    /// optional field specify whether the Secret or its keys must be defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// secretName is the name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::SecretVolumeSource {
    type Optioned = SecretVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for SecretVolumeSourceAc {
    type Optioned = SecretVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::SecretVolumeSource {
    fn into_optioned(self) -> SecretVolumeSourceAc {
        SecretVolumeSourceAc {
            default_mode: self.default_mode,
            items: crate::OptionableConvert::into_optioned(self.items),
            optional: self.optional,
            secret_name: self.secret_name,
        }
    }
    fn try_from_optioned(value: SecretVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            default_mode: value.default_mode,
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
            optional: value.optional,
            secret_name: value.secret_name,
        })
    }
    fn merge(&mut self, other: SecretVolumeSourceAc) -> Result<(), crate::Error> {
        if self.default_mode.is_none() {
            self.default_mode = crate::OptionableConvert::try_from_optioned(
                other.default_mode,
            )?;
        } else if let Some(self_value) = self.default_mode.as_mut()
            && let Some(other_value) = other.default_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.items.is_none() {
            self.items = crate::OptionableConvert::try_from_optioned(other.items)?;
        } else if let Some(self_value) = self.items.as_mut()
            && let Some(other_value) = other.items
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.optional.is_none() {
            self.optional = crate::OptionableConvert::try_from_optioned(other.optional)?;
        } else if let Some(self_value) = self.optional.as_mut()
            && let Some(other_value) = other.optional
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.secret_name.is_none() {
            self.secret_name = crate::OptionableConvert::try_from_optioned(
                other.secret_name,
            )?;
        } else if let Some(self_value) = self.secret_name.as_mut()
            && let Some(other_value) = other.secret_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::SecretVolumeSource>
for SecretVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::SecretVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::SecretVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::SecretVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for SecretVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.default_mode,
            other.default_mode,
        );
        self.items = other.items;
        k8s_openapi027::DeepMerge::merge_from(&mut self.optional, other.optional);
        k8s_openapi027::DeepMerge::merge_from(&mut self.secret_name, other.secret_name);
    }
}
