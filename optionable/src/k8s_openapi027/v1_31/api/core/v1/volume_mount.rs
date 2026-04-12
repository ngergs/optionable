#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeMount describes a mounting of a Volume within a container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeMountAc {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    pub mount_path: std::string::String,
    /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10. When RecursiveReadOnly is set to IfPossible or to Enabled, MountPropagation must be None or unspecified (which defaults to None).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<std::string::String>,
    /// This must match the Name of a Volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// RecursiveReadOnly specifies whether read-only mounts should be handled recursively.
    ///
    /// If ReadOnly is false, this field has no meaning and must be unspecified.
    ///
    /// If ReadOnly is true, and this field is set to Disabled, the mount is not made recursively read-only.  If this field is set to IfPossible, the mount is made recursively read-only, if it is supported by the container runtime.  If this field is set to Enabled, the mount is made recursively read-only if it is supported by the container runtime, otherwise the pod will not be started and an error will be generated to indicate the reason.
    ///
    /// If this field is set to IfPossible or Enabled, MountPropagation must be set to None (or be unspecified, which defaults to None).
    ///
    /// If this field is not specified, it is treated as an equivalent of Disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<std::string::String>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<std::string::String>,
    /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path_expr: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::VolumeMount {
    type Optioned = VolumeMountAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeMountAc {
    type Optioned = VolumeMountAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::VolumeMount {
    fn into_optioned(self) -> VolumeMountAc {
        VolumeMountAc {
            mount_path: self.mount_path,
            mount_propagation: self.mount_propagation,
            name: Some(self.name),
            read_only: self.read_only,
            recursive_read_only: self.recursive_read_only,
            sub_path: self.sub_path,
            sub_path_expr: self.sub_path_expr,
        }
    }
    fn try_from_optioned(value: VolumeMountAc) -> Result<Self, crate::Error> {
        Ok(Self {
            mount_path: value.mount_path,
            mount_propagation: value.mount_propagation,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            read_only: value.read_only,
            recursive_read_only: value.recursive_read_only,
            sub_path: value.sub_path,
            sub_path_expr: value.sub_path_expr,
        })
    }
    fn merge(&mut self, other: VolumeMountAc) -> Result<(), crate::Error> {
        self.mount_path = other.mount_path;
        if self.mount_propagation.is_none() {
            self.mount_propagation = crate::OptionableConvert::try_from_optioned(
                other.mount_propagation,
            )?;
        } else if let Some(self_value) = self.mount_propagation.as_mut()
            && let Some(other_value) = other.mount_propagation
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
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
        if self.recursive_read_only.is_none() {
            self.recursive_read_only = crate::OptionableConvert::try_from_optioned(
                other.recursive_read_only,
            )?;
        } else if let Some(self_value) = self.recursive_read_only.as_mut()
            && let Some(other_value) = other.recursive_read_only
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.sub_path.is_none() {
            self.sub_path = crate::OptionableConvert::try_from_optioned(other.sub_path)?;
        } else if let Some(self_value) = self.sub_path.as_mut()
            && let Some(other_value) = other.sub_path
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.sub_path_expr.is_none() {
            self.sub_path_expr = crate::OptionableConvert::try_from_optioned(
                other.sub_path_expr,
            )?;
        } else if let Some(self_value) = self.sub_path_expr.as_mut()
            && let Some(other_value) = other.sub_path_expr
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::VolumeMount {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.mount_path == other.mount_path
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::VolumeMount>
for VolumeMountAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::VolumeMount) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::VolumeMount, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::VolumeMount,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
