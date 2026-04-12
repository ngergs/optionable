#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecurityContextAc {
    /// AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,
    /// appArmorProfile is the AppArmor options to use by this container. If set, this profile overrides the pod's appArmorProfile. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<
        <::k8s_openapi027::api::core::v1::AppArmorProfile as crate::Optionable>::Optioned,
    >,
    /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<
        <::k8s_openapi027::api::core::v1::Capabilities as crate::Optionable>::Optioned,
    >,
    /// Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// procMount denotes the type of proc mount to use for the containers. The default value is Default which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proc_mount: Option<std::string::String>,
    /// Whether this container has a read-only root filesystem. Default is false. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    /// The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<
        <::k8s_openapi027::api::core::v1::SELinuxOptions as crate::Optionable>::Optioned,
    >,
    /// The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<
        <::k8s_openapi027::api::core::v1::SeccompProfile as crate::Optionable>::Optioned,
    >,
    /// The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<
        <::k8s_openapi027::api::core::v1::WindowsSecurityContextOptions as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::SecurityContext {
    type Optioned = SecurityContextAc;
}
#[automatically_derived]
impl crate::Optionable for SecurityContextAc {
    type Optioned = SecurityContextAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::SecurityContext {
    fn into_optioned(self) -> SecurityContextAc {
        SecurityContextAc {
            allow_privilege_escalation: self.allow_privilege_escalation,
            app_armor_profile: crate::OptionableConvert::into_optioned(
                self.app_armor_profile,
            ),
            capabilities: crate::OptionableConvert::into_optioned(self.capabilities),
            privileged: self.privileged,
            proc_mount: self.proc_mount,
            read_only_root_filesystem: self.read_only_root_filesystem,
            run_as_group: self.run_as_group,
            run_as_non_root: self.run_as_non_root,
            run_as_user: self.run_as_user,
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
    fn try_from_optioned(value: SecurityContextAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allow_privilege_escalation: value.allow_privilege_escalation,
            app_armor_profile: crate::OptionableConvert::try_from_optioned(
                value.app_armor_profile,
            )?,
            capabilities: crate::OptionableConvert::try_from_optioned(
                value.capabilities,
            )?,
            privileged: value.privileged,
            proc_mount: value.proc_mount,
            read_only_root_filesystem: value.read_only_root_filesystem,
            run_as_group: value.run_as_group,
            run_as_non_root: value.run_as_non_root,
            run_as_user: value.run_as_user,
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
    fn merge(&mut self, other: SecurityContextAc) -> Result<(), crate::Error> {
        if other.allow_privilege_escalation.is_some() {
            self.allow_privilege_escalation = other.allow_privilege_escalation;
        }
        crate::OptionableConvert::merge(
            &mut self.app_armor_profile,
            other.app_armor_profile,
        )?;
        crate::OptionableConvert::merge(&mut self.capabilities, other.capabilities)?;
        if other.privileged.is_some() {
            self.privileged = other.privileged;
        }
        if other.proc_mount.is_some() {
            self.proc_mount = other.proc_mount;
        }
        if other.read_only_root_filesystem.is_some() {
            self.read_only_root_filesystem = other.read_only_root_filesystem;
        }
        if other.run_as_group.is_some() {
            self.run_as_group = other.run_as_group;
        }
        if other.run_as_non_root.is_some() {
            self.run_as_non_root = other.run_as_non_root;
        }
        if other.run_as_user.is_some() {
            self.run_as_user = other.run_as_user;
        }
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::SecurityContext>
for SecurityContextAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::SecurityContext) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::SecurityContext, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::SecurityContext,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
