#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinuxContainerUserAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_groups: <Option<std::vec::Vec<i64>> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LinuxContainerUser {
    type Optioned = LinuxContainerUserAc;
}
#[automatically_derived]
impl crate::Optionable for LinuxContainerUserAc {
    type Optioned = LinuxContainerUserAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LinuxContainerUser {
    fn into_optioned(self) -> LinuxContainerUserAc {
        LinuxContainerUserAc {
            gid: Some(self.gid),
            supplemental_groups: crate::OptionableConvert::into_optioned(
                self.supplemental_groups,
            ),
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
            supplemental_groups: crate::OptionableConvert::try_from_optioned(
                value.supplemental_groups,
            )?,
            uid: value
                .uid
                .ok_or(crate::Error {
                    missing_field: "uid",
                })?,
        })
    }
    fn merge(&mut self, other: LinuxContainerUserAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.gid {
            self.gid = other_value;
        }
        crate::OptionableConvert::merge(
            &mut self.supplemental_groups,
            other.supplemental_groups,
        )?;
        if let Some(other_value) = other.uid {
            self.uid = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::LinuxContainerUser>
for LinuxContainerUserAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::LinuxContainerUser) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::LinuxContainerUser, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::LinuxContainerUser,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
