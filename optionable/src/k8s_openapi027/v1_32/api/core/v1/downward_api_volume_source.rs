#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DownwardAPIVolumeSource represents a volume containing downward API info. Downward API volumes support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DownwardAPIVolumeSourceAc {
    /// Optional: mode bits to use on created files by default. Must be a Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<i32>,
    /// Items is a list of downward API volume file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::DownwardAPIVolumeFile as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::DownwardAPIVolumeSource {
    type Optioned = DownwardAPIVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIVolumeSourceAc {
    type Optioned = DownwardAPIVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::DownwardAPIVolumeSource {
    fn into_optioned(self) -> DownwardAPIVolumeSourceAc {
        DownwardAPIVolumeSourceAc {
            default_mode: self.default_mode,
            items: crate::OptionableConvert::into_optioned(self.items),
        }
    }
    fn try_from_optioned(
        value: DownwardAPIVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            default_mode: value.default_mode,
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
        })
    }
    fn merge(&mut self, other: DownwardAPIVolumeSourceAc) -> Result<(), crate::Error> {
        self.default_mode = other.default_mode;
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::DownwardAPIVolumeSource>
for DownwardAPIVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::DownwardAPIVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::DownwardAPIVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::DownwardAPIVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
