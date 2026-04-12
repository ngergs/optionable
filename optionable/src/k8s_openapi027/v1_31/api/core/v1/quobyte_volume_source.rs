#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not support ownership management or SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QuobyteVolumeSourceAc {
    /// group to map volume access to Default is no group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    /// readOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<std::string::String>,
    /// tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<std::string::String>,
    /// user to map volume access to Defaults to serivceaccount user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
    /// volume is a string that references an already created Quobyte volume by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::QuobyteVolumeSource {
    type Optioned = QuobyteVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for QuobyteVolumeSourceAc {
    type Optioned = QuobyteVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::QuobyteVolumeSource {
    fn into_optioned(self) -> QuobyteVolumeSourceAc {
        QuobyteVolumeSourceAc {
            group: self.group,
            read_only: self.read_only,
            registry: Some(self.registry),
            tenant: self.tenant,
            user: self.user,
            volume: Some(self.volume),
        }
    }
    fn try_from_optioned(value: QuobyteVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: value.group,
            read_only: value.read_only,
            registry: value
                .registry
                .ok_or(crate::Error {
                    missing_field: "registry",
                })?,
            tenant: value.tenant,
            user: value.user,
            volume: value
                .volume
                .ok_or(crate::Error {
                    missing_field: "volume",
                })?,
        })
    }
    fn merge(&mut self, other: QuobyteVolumeSourceAc) -> Result<(), crate::Error> {
        if self.group.is_none() {
            self.group = crate::OptionableConvert::try_from_optioned(other.group)?;
        } else {
            crate::OptionableConvert::merge(&mut self.group, other.group)?;
        }
        if self.read_only.is_none() {
            self.read_only = crate::OptionableConvert::try_from_optioned(
                other.read_only,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        }
        if let Some(other_value) = other.registry {
            self.registry = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.tenant.is_none() {
            self.tenant = crate::OptionableConvert::try_from_optioned(other.tenant)?;
        } else {
            crate::OptionableConvert::merge(&mut self.tenant, other.tenant)?;
        }
        if self.user.is_none() {
            self.user = crate::OptionableConvert::try_from_optioned(other.user)?;
        } else {
            crate::OptionableConvert::merge(&mut self.user, other.user)?;
        }
        if let Some(other_value) = other.volume {
            self.volume = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::QuobyteVolumeSource>
for QuobyteVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::QuobyteVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::QuobyteVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::QuobyteVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
