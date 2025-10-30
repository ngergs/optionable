pub struct SecurityContextOpt {
    pub allow_privilege_escalation: <Option<bool> as crate::Optionable>::Optioned,
    pub app_armor_profile: <Option<
        ::k8s_openapi::api::core::v1::AppArmorProfile,
    > as crate::Optionable>::Optioned,
    pub capabilities: <Option<
        ::k8s_openapi::api::core::v1::Capabilities,
    > as crate::Optionable>::Optioned,
    pub privileged: <Option<bool> as crate::Optionable>::Optioned,
    pub proc_mount: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub read_only_root_filesystem: <Option<bool> as crate::Optionable>::Optioned,
    pub run_as_group: <Option<i64> as crate::Optionable>::Optioned,
    pub run_as_non_root: <Option<bool> as crate::Optionable>::Optioned,
    pub run_as_user: <Option<i64> as crate::Optionable>::Optioned,
    pub se_linux_options: <Option<
        ::k8s_openapi::api::core::v1::SELinuxOptions,
    > as crate::Optionable>::Optioned,
    pub seccomp_profile: <Option<
        ::k8s_openapi::api::core::v1::SeccompProfile,
    > as crate::Optionable>::Optioned,
    pub windows_options: <Option<
        ::k8s_openapi::api::core::v1::WindowsSecurityContextOptions,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::SecurityContext {
    type Optioned = SecurityContextOpt;
}
#[automatically_derived]
impl crate::Optionable for SecurityContextOpt {
    type Optioned = SecurityContextOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SecurityContext {
    fn into_optioned(self) -> SecurityContextOpt {
        SecurityContextOpt {
            allow_privilege_escalation: crate::OptionableConvert::into_optioned(
                self.allow_privilege_escalation,
            ),
            app_armor_profile: crate::OptionableConvert::into_optioned(
                self.app_armor_profile,
            ),
            capabilities: crate::OptionableConvert::into_optioned(self.capabilities),
            privileged: crate::OptionableConvert::into_optioned(self.privileged),
            proc_mount: crate::OptionableConvert::into_optioned(self.proc_mount),
            read_only_root_filesystem: crate::OptionableConvert::into_optioned(
                self.read_only_root_filesystem,
            ),
            run_as_group: crate::OptionableConvert::into_optioned(self.run_as_group),
            run_as_non_root: crate::OptionableConvert::into_optioned(
                self.run_as_non_root,
            ),
            run_as_user: crate::OptionableConvert::into_optioned(self.run_as_user),
            se_linux_options: crate::OptionableConvert::into_optioned(
                self.se_linux_options,
            ),
            seccomp_profile: crate::OptionableConvert::into_optioned(
                self.seccomp_profile,
            ),
            windows_options: crate::OptionableConvert::into_optioned(
                self.windows_options,
            ),
        }
    }
    fn try_from_optioned(
        value: SecurityContextOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allow_privilege_escalation: crate::OptionableConvert::try_from_optioned(
                value.allow_privilege_escalation,
            )?,
            app_armor_profile: crate::OptionableConvert::try_from_optioned(
                value.app_armor_profile,
            )?,
            capabilities: crate::OptionableConvert::try_from_optioned(
                value.capabilities,
            )?,
            privileged: crate::OptionableConvert::try_from_optioned(value.privileged)?,
            proc_mount: crate::OptionableConvert::try_from_optioned(value.proc_mount)?,
            read_only_root_filesystem: crate::OptionableConvert::try_from_optioned(
                value.read_only_root_filesystem,
            )?,
            run_as_group: crate::OptionableConvert::try_from_optioned(
                value.run_as_group,
            )?,
            run_as_non_root: crate::OptionableConvert::try_from_optioned(
                value.run_as_non_root,
            )?,
            run_as_user: crate::OptionableConvert::try_from_optioned(value.run_as_user)?,
            se_linux_options: crate::OptionableConvert::try_from_optioned(
                value.se_linux_options,
            )?,
            seccomp_profile: crate::OptionableConvert::try_from_optioned(
                value.seccomp_profile,
            )?,
            windows_options: crate::OptionableConvert::try_from_optioned(
                value.windows_options,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: SecurityContextOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.allow_privilege_escalation,
            other.allow_privilege_escalation,
        )?;
        crate::OptionableConvert::merge(
            &mut self.app_armor_profile,
            other.app_armor_profile,
        )?;
        crate::OptionableConvert::merge(&mut self.capabilities, other.capabilities)?;
        crate::OptionableConvert::merge(&mut self.privileged, other.privileged)?;
        crate::OptionableConvert::merge(&mut self.proc_mount, other.proc_mount)?;
        crate::OptionableConvert::merge(
            &mut self.read_only_root_filesystem,
            other.read_only_root_filesystem,
        )?;
        crate::OptionableConvert::merge(&mut self.run_as_group, other.run_as_group)?;
        crate::OptionableConvert::merge(
            &mut self.run_as_non_root,
            other.run_as_non_root,
        )?;
        crate::OptionableConvert::merge(&mut self.run_as_user, other.run_as_user)?;
        crate::OptionableConvert::merge(
            &mut self.se_linux_options,
            other.se_linux_options,
        )?;
        crate::OptionableConvert::merge(
            &mut self.seccomp_profile,
            other.seccomp_profile,
        )?;
        crate::OptionableConvert::merge(
            &mut self.windows_options,
            other.windows_options,
        )?;
        Ok(())
    }
}
