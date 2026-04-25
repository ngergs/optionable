#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodSecurityContext holds pod-level security attributes and common container settings. Some fields are also present in container.securityContext.  Field values of container.securityContext take precedence over field values of PodSecurityContext.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodSecurityContextAc {
    /// appArmorProfile is the AppArmor options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<
        <::k8s_openapi027::api::core::v1::AppArmorProfile as crate::Optionable>::Optioned,
    >,
    /// A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:
    ///
    /// 1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----
    ///
    /// If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_group: Option<i64>,
    /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_group_change_policy: Option<std::string::String>,
    /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
    /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_linux_options: Option<
        <::k8s_openapi027::api::core::v1::SELinuxOptions as crate::Optionable>::Optioned,
    >,
    /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp_profile: Option<
        <::k8s_openapi027::api::core::v1::SeccompProfile as crate::Optionable>::Optioned,
    >,
    /// A list of groups applied to the first process run in each container, in addition to the container's primary GID and fsGroup (if specified).  If the SupplementalGroupsPolicy feature is enabled, the supplementalGroupsPolicy field determines whether these are in addition to or instead of any group memberships defined in the container image. If unspecified, no additional groups are added, though group memberships defined in the container image may still be used, depending on the supplementalGroupsPolicy field. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_groups: Option<std::vec::Vec<i64>>,
    /// Defines how supplemental groups of the first container processes are calculated. Valid values are "Merge" and "Strict". If not specified, "Merge" is used. (Alpha) Using the field requires the SupplementalGroupsPolicy feature gate to be enabled and the container runtime must implement support for this feature. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_groups_policy: Option<std::string::String>,
    /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Sysctl as crate::Optionable>::Optioned,
        >,
    >,
    /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_options: Option<
        <::k8s_openapi027::api::core::v1::WindowsSecurityContextOptions as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodSecurityContext {
    type Optioned = PodSecurityContextAc;
}
#[automatically_derived]
impl crate::Optionable for PodSecurityContextAc {
    type Optioned = PodSecurityContextAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodSecurityContext {
    fn into_optioned(self) -> PodSecurityContextAc {
        PodSecurityContextAc {
            app_armor_profile: crate::OptionableConvert::into_optioned(
                self.app_armor_profile,
            ),
            fs_group: self.fs_group,
            fs_group_change_policy: self.fs_group_change_policy,
            run_as_group: self.run_as_group,
            run_as_non_root: self.run_as_non_root,
            run_as_user: self.run_as_user,
            se_linux_options: crate::OptionableConvert::into_optioned(
                self.se_linux_options,
            ),
            seccomp_profile: crate::OptionableConvert::into_optioned(
                self.seccomp_profile,
            ),
            supplemental_groups: self.supplemental_groups,
            supplemental_groups_policy: self.supplemental_groups_policy,
            sysctls: crate::OptionableConvert::into_optioned(self.sysctls),
            windows_options: crate::OptionableConvert::into_optioned(
                self.windows_options,
            ),
        }
    }
    fn try_from_optioned(value: PodSecurityContextAc) -> Result<Self, crate::Error> {
        Ok(Self {
            app_armor_profile: crate::OptionableConvert::try_from_optioned(
                value.app_armor_profile,
            )?,
            fs_group: value.fs_group,
            fs_group_change_policy: value.fs_group_change_policy,
            run_as_group: value.run_as_group,
            run_as_non_root: value.run_as_non_root,
            run_as_user: value.run_as_user,
            se_linux_options: crate::OptionableConvert::try_from_optioned(
                value.se_linux_options,
            )?,
            seccomp_profile: crate::OptionableConvert::try_from_optioned(
                value.seccomp_profile,
            )?,
            supplemental_groups: value.supplemental_groups,
            supplemental_groups_policy: value.supplemental_groups_policy,
            sysctls: crate::OptionableConvert::try_from_optioned(value.sysctls)?,
            windows_options: crate::OptionableConvert::try_from_optioned(
                value.windows_options,
            )?,
        })
    }
    fn merge(&mut self, other: PodSecurityContextAc) -> Result<(), crate::Error> {
        if self.app_armor_profile.is_none() {
            self.app_armor_profile = crate::OptionableConvert::try_from_optioned(
                other.app_armor_profile,
            )?;
        } else if let Some(self_value) = self.app_armor_profile.as_mut()
            && let Some(other_value) = other.app_armor_profile
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.fs_group.is_none() {
            self.fs_group = crate::OptionableConvert::try_from_optioned(other.fs_group)?;
        } else if let Some(self_value) = self.fs_group.as_mut()
            && let Some(other_value) = other.fs_group
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.fs_group_change_policy.is_none() {
            self.fs_group_change_policy = crate::OptionableConvert::try_from_optioned(
                other.fs_group_change_policy,
            )?;
        } else if let Some(self_value) = self.fs_group_change_policy.as_mut()
            && let Some(other_value) = other.fs_group_change_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.run_as_group.is_none() {
            self.run_as_group = crate::OptionableConvert::try_from_optioned(
                other.run_as_group,
            )?;
        } else if let Some(self_value) = self.run_as_group.as_mut()
            && let Some(other_value) = other.run_as_group
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.run_as_non_root.is_none() {
            self.run_as_non_root = crate::OptionableConvert::try_from_optioned(
                other.run_as_non_root,
            )?;
        } else if let Some(self_value) = self.run_as_non_root.as_mut()
            && let Some(other_value) = other.run_as_non_root
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.run_as_user.is_none() {
            self.run_as_user = crate::OptionableConvert::try_from_optioned(
                other.run_as_user,
            )?;
        } else if let Some(self_value) = self.run_as_user.as_mut()
            && let Some(other_value) = other.run_as_user
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.se_linux_options.is_none() {
            self.se_linux_options = crate::OptionableConvert::try_from_optioned(
                other.se_linux_options,
            )?;
        } else if let Some(self_value) = self.se_linux_options.as_mut()
            && let Some(other_value) = other.se_linux_options
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.seccomp_profile.is_none() {
            self.seccomp_profile = crate::OptionableConvert::try_from_optioned(
                other.seccomp_profile,
            )?;
        } else if let Some(self_value) = self.seccomp_profile.as_mut()
            && let Some(other_value) = other.seccomp_profile
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
        if self.supplemental_groups_policy.is_none() {
            self.supplemental_groups_policy = crate::OptionableConvert::try_from_optioned(
                other.supplemental_groups_policy,
            )?;
        } else if let Some(self_value) = self.supplemental_groups_policy.as_mut()
            && let Some(other_value) = other.supplemental_groups_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.sysctls.is_none() {
            self.sysctls = crate::OptionableConvert::try_from_optioned(other.sysctls)?;
        } else if let Some(self_value) = self.sysctls.as_mut()
            && let Some(other_value) = other.sysctls
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.windows_options.is_none() {
            self.windows_options = crate::OptionableConvert::try_from_optioned(
                other.windows_options,
            )?;
        } else if let Some(self_value) = self.windows_options.as_mut()
            && let Some(other_value) = other.windows_options
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodSecurityContext>
for PodSecurityContextAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PodSecurityContext,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodSecurityContext, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodSecurityContext,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PodSecurityContextAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.app_armor_profile,
            other.app_armor_profile,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.fs_group, other.fs_group);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.fs_group_change_policy,
            other.fs_group_change_policy,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.run_as_group,
            other.run_as_group,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.run_as_non_root,
            other.run_as_non_root,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.run_as_user, other.run_as_user);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.se_linux_options,
            other.se_linux_options,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.seccomp_profile,
            other.seccomp_profile,
        );
        self.supplemental_groups = other.supplemental_groups;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.supplemental_groups_policy,
            other.supplemental_groups_policy,
        );
        self.sysctls = other.sysctls;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.windows_options,
            other.windows_options,
        );
    }
}
