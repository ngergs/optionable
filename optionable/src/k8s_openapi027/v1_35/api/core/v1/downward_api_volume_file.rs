#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DownwardAPIVolumeFileAc {
    /// Required: Selects a field of the pod: only annotations, labels, name, namespace and uid are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<
        <::k8s_openapi027::api::core::v1::ObjectFieldSelector as crate::Optionable>::Optioned,
    >,
    /// Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<
        <::k8s_openapi027::api::core::v1::ResourceFieldSelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::DownwardAPIVolumeFile {
    type Optioned = DownwardAPIVolumeFileAc;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIVolumeFileAc {
    type Optioned = DownwardAPIVolumeFileAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::DownwardAPIVolumeFile {
    fn into_optioned(self) -> DownwardAPIVolumeFileAc {
        DownwardAPIVolumeFileAc {
            field_ref: crate::OptionableConvert::into_optioned(self.field_ref),
            mode: self.mode,
            path: Some(self.path),
            resource_field_ref: crate::OptionableConvert::into_optioned(
                self.resource_field_ref,
            ),
        }
    }
    fn try_from_optioned(value: DownwardAPIVolumeFileAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field_ref: crate::OptionableConvert::try_from_optioned(value.field_ref)?,
            mode: value.mode,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            resource_field_ref: crate::OptionableConvert::try_from_optioned(
                value.resource_field_ref,
            )?,
        })
    }
    fn merge(&mut self, other: DownwardAPIVolumeFileAc) -> Result<(), crate::Error> {
        if self.field_ref.is_none() {
            self.field_ref = other.field_ref;
        }
        if let Some(other_value) = other.field_ref {
            crate::OptionableConvert::merge(&mut self.field_ref, other_value)?;
        }
        if self.mode.is_none() {
            self.mode = other.mode;
        }
        if let Some(other_value) = other.mode {
            crate::OptionableConvert::merge(&mut self.mode, other_value)?;
        }
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        if self.resource_field_ref.is_none() {
            self.resource_field_ref = other.resource_field_ref;
        }
        if let Some(other_value) = other.resource_field_ref {
            crate::OptionableConvert::merge(&mut self.resource_field_ref, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::DownwardAPIVolumeFile>
for DownwardAPIVolumeFileAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::DownwardAPIVolumeFile,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::DownwardAPIVolumeFile, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::DownwardAPIVolumeFile,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
