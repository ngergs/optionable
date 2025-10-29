pub struct ContainerOpt {
    pub args: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub command: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub env: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EnvVar>,
    > as crate::Optionable>::Optioned,
    pub env_from: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EnvFromSource>,
    > as crate::Optionable>::Optioned,
    pub image: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub image_pull_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub lifecycle: <Option<
        ::k8s_openapi::api::core::v1::Lifecycle,
    > as crate::Optionable>::Optioned,
    pub liveness_probe: <Option<
        ::k8s_openapi::api::core::v1::Probe,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ContainerPort>,
    > as crate::Optionable>::Optioned,
    pub readiness_probe: <Option<
        ::k8s_openapi::api::core::v1::Probe,
    > as crate::Optionable>::Optioned,
    pub resize_policy: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ContainerResizePolicy>,
    > as crate::Optionable>::Optioned,
    pub resources: <Option<
        ::k8s_openapi::api::core::v1::ResourceRequirements,
    > as crate::Optionable>::Optioned,
    pub restart_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub restart_policy_rules: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ContainerRestartRule>,
    > as crate::Optionable>::Optioned,
    pub security_context: <Option<
        ::k8s_openapi::api::core::v1::SecurityContext,
    > as crate::Optionable>::Optioned,
    pub startup_probe: <Option<
        ::k8s_openapi::api::core::v1::Probe,
    > as crate::Optionable>::Optioned,
    pub stdin: <Option<bool> as crate::Optionable>::Optioned,
    pub stdin_once: <Option<bool> as crate::Optionable>::Optioned,
    pub termination_message_path: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub termination_message_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub tty: <Option<bool> as crate::Optionable>::Optioned,
    pub volume_devices: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::VolumeDevice>,
    > as crate::Optionable>::Optioned,
    pub volume_mounts: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMount>,
    > as crate::Optionable>::Optioned,
    pub working_dir: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Container {
    type Optioned = ContainerOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerOpt {
    type Optioned = ContainerOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Container {
    fn into_optioned(self) -> ContainerOpt {
        ContainerOpt {
            args: crate::OptionableConvert::into_optioned(self.args),
            command: crate::OptionableConvert::into_optioned(self.command),
            env: crate::OptionableConvert::into_optioned(self.env),
            env_from: crate::OptionableConvert::into_optioned(self.env_from),
            image: crate::OptionableConvert::into_optioned(self.image),
            image_pull_policy: crate::OptionableConvert::into_optioned(
                self.image_pull_policy,
            ),
            lifecycle: crate::OptionableConvert::into_optioned(self.lifecycle),
            liveness_probe: crate::OptionableConvert::into_optioned(self.liveness_probe),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            ports: crate::OptionableConvert::into_optioned(self.ports),
            readiness_probe: crate::OptionableConvert::into_optioned(
                self.readiness_probe,
            ),
            resize_policy: crate::OptionableConvert::into_optioned(self.resize_policy),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            restart_policy: crate::OptionableConvert::into_optioned(self.restart_policy),
            restart_policy_rules: crate::OptionableConvert::into_optioned(
                self.restart_policy_rules,
            ),
            security_context: crate::OptionableConvert::into_optioned(
                self.security_context,
            ),
            startup_probe: crate::OptionableConvert::into_optioned(self.startup_probe),
            stdin: crate::OptionableConvert::into_optioned(self.stdin),
            stdin_once: crate::OptionableConvert::into_optioned(self.stdin_once),
            termination_message_path: crate::OptionableConvert::into_optioned(
                self.termination_message_path,
            ),
            termination_message_policy: crate::OptionableConvert::into_optioned(
                self.termination_message_policy,
            ),
            tty: crate::OptionableConvert::into_optioned(self.tty),
            volume_devices: crate::OptionableConvert::into_optioned(self.volume_devices),
            volume_mounts: crate::OptionableConvert::into_optioned(self.volume_mounts),
            working_dir: crate::OptionableConvert::into_optioned(self.working_dir),
        }
    }
    fn try_from_optioned(value: ContainerOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            args: crate::OptionableConvert::try_from_optioned(value.args)?,
            command: crate::OptionableConvert::try_from_optioned(value.command)?,
            env: crate::OptionableConvert::try_from_optioned(value.env)?,
            env_from: crate::OptionableConvert::try_from_optioned(value.env_from)?,
            image: crate::OptionableConvert::try_from_optioned(value.image)?,
            image_pull_policy: crate::OptionableConvert::try_from_optioned(
                value.image_pull_policy,
            )?,
            lifecycle: crate::OptionableConvert::try_from_optioned(value.lifecycle)?,
            liveness_probe: crate::OptionableConvert::try_from_optioned(
                value.liveness_probe,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
            readiness_probe: crate::OptionableConvert::try_from_optioned(
                value.readiness_probe,
            )?,
            resize_policy: crate::OptionableConvert::try_from_optioned(
                value.resize_policy,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            restart_policy: crate::OptionableConvert::try_from_optioned(
                value.restart_policy,
            )?,
            restart_policy_rules: crate::OptionableConvert::try_from_optioned(
                value.restart_policy_rules,
            )?,
            security_context: crate::OptionableConvert::try_from_optioned(
                value.security_context,
            )?,
            startup_probe: crate::OptionableConvert::try_from_optioned(
                value.startup_probe,
            )?,
            stdin: crate::OptionableConvert::try_from_optioned(value.stdin)?,
            stdin_once: crate::OptionableConvert::try_from_optioned(value.stdin_once)?,
            termination_message_path: crate::OptionableConvert::try_from_optioned(
                value.termination_message_path,
            )?,
            termination_message_policy: crate::OptionableConvert::try_from_optioned(
                value.termination_message_policy,
            )?,
            tty: crate::OptionableConvert::try_from_optioned(value.tty)?,
            volume_devices: crate::OptionableConvert::try_from_optioned(
                value.volume_devices,
            )?,
            volume_mounts: crate::OptionableConvert::try_from_optioned(
                value.volume_mounts,
            )?,
            working_dir: crate::OptionableConvert::try_from_optioned(value.working_dir)?,
        })
    }
    fn merge(&mut self, other: ContainerOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.args, other.args)?;
        crate::OptionableConvert::merge(&mut self.command, other.command)?;
        crate::OptionableConvert::merge(&mut self.env, other.env)?;
        crate::OptionableConvert::merge(&mut self.env_from, other.env_from)?;
        crate::OptionableConvert::merge(&mut self.image, other.image)?;
        crate::OptionableConvert::merge(
            &mut self.image_pull_policy,
            other.image_pull_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.lifecycle, other.lifecycle)?;
        crate::OptionableConvert::merge(&mut self.liveness_probe, other.liveness_probe)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        crate::OptionableConvert::merge(
            &mut self.readiness_probe,
            other.readiness_probe,
        )?;
        crate::OptionableConvert::merge(&mut self.resize_policy, other.resize_policy)?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        crate::OptionableConvert::merge(&mut self.restart_policy, other.restart_policy)?;
        crate::OptionableConvert::merge(
            &mut self.restart_policy_rules,
            other.restart_policy_rules,
        )?;
        crate::OptionableConvert::merge(
            &mut self.security_context,
            other.security_context,
        )?;
        crate::OptionableConvert::merge(&mut self.startup_probe, other.startup_probe)?;
        crate::OptionableConvert::merge(&mut self.stdin, other.stdin)?;
        crate::OptionableConvert::merge(&mut self.stdin_once, other.stdin_once)?;
        crate::OptionableConvert::merge(
            &mut self.termination_message_path,
            other.termination_message_path,
        )?;
        crate::OptionableConvert::merge(
            &mut self.termination_message_policy,
            other.termination_message_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.tty, other.tty)?;
        crate::OptionableConvert::merge(&mut self.volume_devices, other.volume_devices)?;
        crate::OptionableConvert::merge(&mut self.volume_mounts, other.volume_mounts)?;
        crate::OptionableConvert::merge(&mut self.working_dir, other.working_dir)?;
        Ok(())
    }
}
