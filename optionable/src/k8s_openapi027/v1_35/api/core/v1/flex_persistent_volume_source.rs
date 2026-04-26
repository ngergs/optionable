#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FlexPersistentVolumeSource represents a generic persistent volume resource that is provisioned/attached using an exec based plugin.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlexPersistentVolumeSourceAc {
    /// driver is the name of the driver to use for this volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// fsType is the Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// options is Optional: this field holds extra command options if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// readOnly is Optional: defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretRef is Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::FlexPersistentVolumeSource {
    type Optioned = FlexPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for FlexPersistentVolumeSourceAc {
    type Optioned = FlexPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::FlexPersistentVolumeSource {
    fn into_optioned(self) -> FlexPersistentVolumeSourceAc {
        FlexPersistentVolumeSourceAc {
            driver: Some(self.driver),
            fs_type: self.fs_type,
            options: self.options,
            read_only: self.read_only,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
        }
    }
    fn try_from_optioned(
        value: FlexPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            fs_type: value.fs_type,
            options: value.options,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
        })
    }
    fn merge(
        &mut self,
        other: FlexPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver {
            self.driver = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.fs_type.is_none() {
            self.fs_type = crate::OptionableConvert::try_from_optioned(other.fs_type)?;
        } else if let Some(self_value) = self.fs_type.as_mut()
            && let Some(other_value) = other.fs_type
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.options.is_none() {
            self.options = crate::OptionableConvert::try_from_optioned(other.options)?;
        } else if let Some(self_value) = self.options.as_mut()
            && let Some(other_value) = other.options
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.read_only.is_none() {
            self.read_only = crate::OptionableConvert::try_from_optioned(
                other.read_only,
            )?;
        } else if let Some(self_value) = self.read_only.as_mut()
            && let Some(other_value) = other.read_only
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
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::FlexPersistentVolumeSource>
for FlexPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::FlexPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::FlexPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::FlexPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for FlexPersistentVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.driver, other.driver);
        k8s_openapi027::DeepMerge::merge_from(&mut self.fs_type, other.fs_type);
        k8s_openapi027::DeepMerge::merge_from(&mut self.options, other.options);
        k8s_openapi027::DeepMerge::merge_from(&mut self.read_only, other.read_only);
        k8s_openapi027::DeepMerge::merge_from(&mut self.secret_ref, other.secret_ref);
    }
}
