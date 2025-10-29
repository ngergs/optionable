pub struct PodSecurityContextOpt {
    pub app_armor_profile: <Option<
        ::k8s_openapi::api::core::v1::AppArmorProfile,
    > as crate::Optionable>::Optioned,
    pub fs_group: <Option<i64> as crate::Optionable>::Optioned,
    pub fs_group_change_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub run_as_group: <Option<i64> as crate::Optionable>::Optioned,
    pub run_as_non_root: <Option<bool> as crate::Optionable>::Optioned,
    pub run_as_user: <Option<i64> as crate::Optionable>::Optioned,
    pub se_linux_change_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub se_linux_options: <Option<
        ::k8s_openapi::api::core::v1::SELinuxOptions,
    > as crate::Optionable>::Optioned,
    pub seccomp_profile: <Option<
        ::k8s_openapi::api::core::v1::SeccompProfile,
    > as crate::Optionable>::Optioned,
    pub supplemental_groups: <Option<std::vec::Vec<i64>> as crate::Optionable>::Optioned,
    pub supplemental_groups_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub sysctls: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::Sysctl>,
    > as crate::Optionable>::Optioned,
    pub windows_options: <Option<
        ::k8s_openapi::api::core::v1::WindowsSecurityContextOptions,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodSecurityContext {
    type Optioned = PodSecurityContextOpt;
}
#[automatically_derived]
impl crate::Optionable for PodSecurityContextOpt {
    type Optioned = PodSecurityContextOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodSecurityContext {
    fn into_optioned(self) -> PodSecurityContextOpt {
        PodSecurityContextOpt {
            app_armor_profile: crate::OptionableConvert::into_optioned(
                self.app_armor_profile,
            ),
            fs_group: crate::OptionableConvert::into_optioned(self.fs_group),
            fs_group_change_policy: crate::OptionableConvert::into_optioned(
                self.fs_group_change_policy,
            ),
            run_as_group: crate::OptionableConvert::into_optioned(self.run_as_group),
            run_as_non_root: crate::OptionableConvert::into_optioned(
                self.run_as_non_root,
            ),
            run_as_user: crate::OptionableConvert::into_optioned(self.run_as_user),
            se_linux_change_policy: crate::OptionableConvert::into_optioned(
                self.se_linux_change_policy,
            ),
            se_linux_options: crate::OptionableConvert::into_optioned(
                self.se_linux_options,
            ),
            seccomp_profile: crate::OptionableConvert::into_optioned(
                self.seccomp_profile,
            ),
            supplemental_groups: crate::OptionableConvert::into_optioned(
                self.supplemental_groups,
            ),
            supplemental_groups_policy: crate::OptionableConvert::into_optioned(
                self.supplemental_groups_policy,
            ),
            sysctls: crate::OptionableConvert::into_optioned(self.sysctls),
            windows_options: crate::OptionableConvert::into_optioned(
                self.windows_options,
            ),
        }
    }
    fn try_from_optioned(
        value: PodSecurityContextOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            app_armor_profile: crate::OptionableConvert::try_from_optioned(
                value.app_armor_profile,
            )?,
            fs_group: crate::OptionableConvert::try_from_optioned(value.fs_group)?,
            fs_group_change_policy: crate::OptionableConvert::try_from_optioned(
                value.fs_group_change_policy,
            )?,
            run_as_group: crate::OptionableConvert::try_from_optioned(
                value.run_as_group,
            )?,
            run_as_non_root: crate::OptionableConvert::try_from_optioned(
                value.run_as_non_root,
            )?,
            run_as_user: crate::OptionableConvert::try_from_optioned(value.run_as_user)?,
            se_linux_change_policy: crate::OptionableConvert::try_from_optioned(
                value.se_linux_change_policy,
            )?,
            se_linux_options: crate::OptionableConvert::try_from_optioned(
                value.se_linux_options,
            )?,
            seccomp_profile: crate::OptionableConvert::try_from_optioned(
                value.seccomp_profile,
            )?,
            supplemental_groups: crate::OptionableConvert::try_from_optioned(
                value.supplemental_groups,
            )?,
            supplemental_groups_policy: crate::OptionableConvert::try_from_optioned(
                value.supplemental_groups_policy,
            )?,
            sysctls: crate::OptionableConvert::try_from_optioned(value.sysctls)?,
            windows_options: crate::OptionableConvert::try_from_optioned(
                value.windows_options,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodSecurityContextOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.app_armor_profile,
            other.app_armor_profile,
        )?;
        crate::OptionableConvert::merge(&mut self.fs_group, other.fs_group)?;
        crate::OptionableConvert::merge(
            &mut self.fs_group_change_policy,
            other.fs_group_change_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.run_as_group, other.run_as_group)?;
        crate::OptionableConvert::merge(
            &mut self.run_as_non_root,
            other.run_as_non_root,
        )?;
        crate::OptionableConvert::merge(&mut self.run_as_user, other.run_as_user)?;
        crate::OptionableConvert::merge(
            &mut self.se_linux_change_policy,
            other.se_linux_change_policy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.se_linux_options,
            other.se_linux_options,
        )?;
        crate::OptionableConvert::merge(
            &mut self.seccomp_profile,
            other.seccomp_profile,
        )?;
        crate::OptionableConvert::merge(
            &mut self.supplemental_groups,
            other.supplemental_groups,
        )?;
        crate::OptionableConvert::merge(
            &mut self.supplemental_groups_policy,
            other.supplemental_groups_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.sysctls, other.sysctls)?;
        crate::OptionableConvert::merge(
            &mut self.windows_options,
            other.windows_options,
        )?;
        Ok(())
    }
}
