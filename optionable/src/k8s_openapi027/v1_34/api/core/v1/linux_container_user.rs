#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// LinuxContainerUser represents user identity information in Linux containers
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinuxContainerUserAc {
    /// GID is the primary gid initially attached to the first process in the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<i64>,
    /// SupplementalGroups are the supplemental groups initially attached to the first process in the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_groups: Option<std::vec::Vec<i64>>,
    /// UID is the primary uid initially attached to the first process in the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::LinuxContainerUser {
    type Optioned = LinuxContainerUserAc;
}
#[automatically_derived]
impl crate::Optionable for LinuxContainerUserAc {
    type Optioned = LinuxContainerUserAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::LinuxContainerUser {
    fn into_optioned(self) -> LinuxContainerUserAc {
        LinuxContainerUserAc {
            gid: Some(self.gid),
            supplemental_groups: self.supplemental_groups,
            uid: Some(self.uid),
        }
    }
    fn try_from_optioned(value: LinuxContainerUserAc) -> Result<Self, crate::Error> {
        Ok(Self {
            gid: value
                .gid
                .ok_or(crate::Error {
                    missing_field: "gid",
                })?,
            supplemental_groups: value.supplemental_groups,
            uid: value
                .uid
                .ok_or(crate::Error {
                    missing_field: "uid",
                })?,
        })
    }
    fn merge(&mut self, other: LinuxContainerUserAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.gid {
            self.gid = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.supplemental_groups.is_none() {
            self.supplemental_groups = crate::OptionableConvert::try_from_optioned(
                other.supplemental_groups,
            )?;
        } else if let Some(self_value) = self.supplemental_groups.as_mut()
            && let Some(other_value) = other.supplemental_groups
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.uid {
            self.uid = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::LinuxContainerUser>
for LinuxContainerUserAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::LinuxContainerUser,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::LinuxContainerUser, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::LinuxContainerUser,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
