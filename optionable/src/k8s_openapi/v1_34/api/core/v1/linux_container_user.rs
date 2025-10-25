pub struct LinuxContainerUserOpt {
    pub gid: Option<i64>,
    pub supplemental_groups: <Option<std::vec::Vec<i64>> as crate::Optionable>::Optioned,
    pub uid: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::linux_container_user::LinuxContainerUser {
    type Optioned = LinuxContainerUserOpt;
}
#[automatically_derived]
impl crate::Optionable for LinuxContainerUserOpt {
    type Optioned = LinuxContainerUserOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::linux_container_user::LinuxContainerUser {
    fn into_optioned(self) -> LinuxContainerUserOpt {
        LinuxContainerUserOpt {
            gid: Some(self.gid),
            supplemental_groups: <Option<
                std::vec::Vec<i64>,
            > as crate::OptionableConvert>::into_optioned(self.supplemental_groups),
            uid: Some(self.uid),
        }
    }
    fn try_from_optioned(
        value: LinuxContainerUserOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            gid: value
                .gid
                .ok_or(crate::optionable::Error {
                    missing_field: "gid",
                })?,
            supplemental_groups: <Option<
                std::vec::Vec<i64>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.supplemental_groups,
            )?,
            uid: value
                .uid
                .ok_or(crate::optionable::Error {
                    missing_field: "uid",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: LinuxContainerUserOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.gid {
            self.gid = other_value;
        }
        <Option<
            std::vec::Vec<i64>,
        > as crate::OptionableConvert>::merge(
            &mut self.supplemental_groups,
            other.supplemental_groups,
        )?;
        if let Some(other_value) = other.uid {
            self.uid = other_value;
        }
        Ok(())
    }
}
