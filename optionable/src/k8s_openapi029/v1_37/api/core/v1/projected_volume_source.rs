#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a projected volume source
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProjectedVolumeSourceAc {
    /// defaultMode are the mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// defaultUser is Optional: The owner UID of the created files by default. The defaultUser field is only used as a fallback when the item-level user field is unset. (Alpha) This field requires the AtomicWriteVolumeUserFields feature gate to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_user: Option<i64>,
    /// sources is the list of volume projections. Each entry in this list handles one source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::core::v1::VolumeProjection as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::core::v1::ProjectedVolumeSource {
    type Optioned = ProjectedVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ProjectedVolumeSourceAc {
    type Optioned = ProjectedVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::core::v1::ProjectedVolumeSource {
    fn into_optioned(self) -> ProjectedVolumeSourceAc {
        ProjectedVolumeSourceAc {
            default_mode: self.default_mode,
            default_user: self.default_user,
            sources: crate::OptionableConvert::into_optioned(self.sources),
        }
    }
    fn try_from_optioned(value: ProjectedVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            default_mode: value.default_mode,
            default_user: value.default_user,
            sources: crate::OptionableConvert::try_from_optioned(value.sources)?,
        })
    }
    fn merge(&mut self, other: ProjectedVolumeSourceAc) -> Result<(), crate::Error> {
        if self.default_mode.is_none() {
            self.default_mode = crate::OptionableConvert::try_from_optioned(
                other.default_mode,
            )?;
        } else if let Some(self_value) = self.default_mode.as_mut()
            && let Some(other_value) = other.default_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.default_user.is_none() {
            self.default_user = crate::OptionableConvert::try_from_optioned(
                other.default_user,
            )?;
        } else if let Some(self_value) = self.default_user.as_mut()
            && let Some(other_value) = other.default_user
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.sources.is_none() {
            self.sources = crate::OptionableConvert::try_from_optioned(other.sources)?;
        } else if let Some(self_value) = self.sources.as_mut()
            && let Some(other_value) = other.sources
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::core::v1::ProjectedVolumeSource>
for ProjectedVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi029::api::core::v1::ProjectedVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::core::v1::ProjectedVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::ProjectedVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for ProjectedVolumeSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.default_mode,
            other.default_mode,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.default_user,
            other.default_user,
        );
        self.sources = other.sources;
    }
}
