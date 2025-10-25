pub struct EphemeralContainerOpt {
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
    pub target_container_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
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
impl crate::Optionable
for ::k8s_openapi::api::core::v1::ephemeral_container::EphemeralContainer {
    type Optioned = EphemeralContainerOpt;
}
#[automatically_derived]
impl crate::Optionable for EphemeralContainerOpt {
    type Optioned = EphemeralContainerOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ephemeral_container::EphemeralContainer {
    fn into_optioned(self) -> EphemeralContainerOpt {
        EphemeralContainerOpt {
            args: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.args),
            command: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.command),
            env: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EnvVar>,
            > as crate::OptionableConvert>::into_optioned(self.env),
            env_from: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EnvFromSource>,
            > as crate::OptionableConvert>::into_optioned(self.env_from),
            image: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.image),
            image_pull_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.image_pull_policy),
            lifecycle: <Option<
                ::k8s_openapi::api::core::v1::Lifecycle,
            > as crate::OptionableConvert>::into_optioned(self.lifecycle),
            liveness_probe: <Option<
                ::k8s_openapi::api::core::v1::Probe,
            > as crate::OptionableConvert>::into_optioned(self.liveness_probe),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ContainerPort>,
            > as crate::OptionableConvert>::into_optioned(self.ports),
            readiness_probe: <Option<
                ::k8s_openapi::api::core::v1::Probe,
            > as crate::OptionableConvert>::into_optioned(self.readiness_probe),
            resize_policy: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ContainerResizePolicy>,
            > as crate::OptionableConvert>::into_optioned(self.resize_policy),
            resources: <Option<
                ::k8s_openapi::api::core::v1::ResourceRequirements,
            > as crate::OptionableConvert>::into_optioned(self.resources),
            restart_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.restart_policy),
            restart_policy_rules: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ContainerRestartRule>,
            > as crate::OptionableConvert>::into_optioned(self.restart_policy_rules),
            security_context: <Option<
                ::k8s_openapi::api::core::v1::SecurityContext,
            > as crate::OptionableConvert>::into_optioned(self.security_context),
            startup_probe: <Option<
                ::k8s_openapi::api::core::v1::Probe,
            > as crate::OptionableConvert>::into_optioned(self.startup_probe),
            stdin: <Option<bool> as crate::OptionableConvert>::into_optioned(self.stdin),
            stdin_once: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.stdin_once),
            target_container_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.target_container_name),
            termination_message_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.termination_message_path),
            termination_message_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(
                self.termination_message_policy,
            ),
            tty: <Option<bool> as crate::OptionableConvert>::into_optioned(self.tty),
            volume_devices: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::VolumeDevice>,
            > as crate::OptionableConvert>::into_optioned(self.volume_devices),
            volume_mounts: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMount>,
            > as crate::OptionableConvert>::into_optioned(self.volume_mounts),
            working_dir: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.working_dir),
        }
    }
    fn try_from_optioned(
        value: EphemeralContainerOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            args: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.args)?,
            command: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.command)?,
            env: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EnvVar>,
            > as crate::OptionableConvert>::try_from_optioned(value.env)?,
            env_from: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EnvFromSource>,
            > as crate::OptionableConvert>::try_from_optioned(value.env_from)?,
            image: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.image)?,
            image_pull_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.image_pull_policy)?,
            lifecycle: <Option<
                ::k8s_openapi::api::core::v1::Lifecycle,
            > as crate::OptionableConvert>::try_from_optioned(value.lifecycle)?,
            liveness_probe: <Option<
                ::k8s_openapi::api::core::v1::Probe,
            > as crate::OptionableConvert>::try_from_optioned(value.liveness_probe)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ContainerPort>,
            > as crate::OptionableConvert>::try_from_optioned(value.ports)?,
            readiness_probe: <Option<
                ::k8s_openapi::api::core::v1::Probe,
            > as crate::OptionableConvert>::try_from_optioned(value.readiness_probe)?,
            resize_policy: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ContainerResizePolicy>,
            > as crate::OptionableConvert>::try_from_optioned(value.resize_policy)?,
            resources: <Option<
                ::k8s_openapi::api::core::v1::ResourceRequirements,
            > as crate::OptionableConvert>::try_from_optioned(value.resources)?,
            restart_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.restart_policy)?,
            restart_policy_rules: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ContainerRestartRule>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.restart_policy_rules,
            )?,
            security_context: <Option<
                ::k8s_openapi::api::core::v1::SecurityContext,
            > as crate::OptionableConvert>::try_from_optioned(value.security_context)?,
            startup_probe: <Option<
                ::k8s_openapi::api::core::v1::Probe,
            > as crate::OptionableConvert>::try_from_optioned(value.startup_probe)?,
            stdin: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.stdin)?,
            stdin_once: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.stdin_once)?,
            target_container_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.target_container_name,
            )?,
            termination_message_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.termination_message_path,
            )?,
            termination_message_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.termination_message_policy,
            )?,
            tty: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.tty)?,
            volume_devices: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::VolumeDevice>,
            > as crate::OptionableConvert>::try_from_optioned(value.volume_devices)?,
            volume_mounts: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMount>,
            > as crate::OptionableConvert>::try_from_optioned(value.volume_mounts)?,
            working_dir: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.working_dir)?,
        })
    }
    fn merge(
        &mut self,
        other: EphemeralContainerOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.args, other.args)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.command, other.command)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::EnvVar>,
        > as crate::OptionableConvert>::merge(&mut self.env, other.env)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::EnvFromSource>,
        > as crate::OptionableConvert>::merge(&mut self.env_from, other.env_from)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.image, other.image)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.image_pull_policy,
            other.image_pull_policy,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::Lifecycle,
        > as crate::OptionableConvert>::merge(&mut self.lifecycle, other.lifecycle)?;
        <Option<
            ::k8s_openapi::api::core::v1::Probe,
        > as crate::OptionableConvert>::merge(
            &mut self.liveness_probe,
            other.liveness_probe,
        )?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ContainerPort>,
        > as crate::OptionableConvert>::merge(&mut self.ports, other.ports)?;
        <Option<
            ::k8s_openapi::api::core::v1::Probe,
        > as crate::OptionableConvert>::merge(
            &mut self.readiness_probe,
            other.readiness_probe,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ContainerResizePolicy>,
        > as crate::OptionableConvert>::merge(
            &mut self.resize_policy,
            other.resize_policy,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::ResourceRequirements,
        > as crate::OptionableConvert>::merge(&mut self.resources, other.resources)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.restart_policy,
            other.restart_policy,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ContainerRestartRule>,
        > as crate::OptionableConvert>::merge(
            &mut self.restart_policy_rules,
            other.restart_policy_rules,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::SecurityContext,
        > as crate::OptionableConvert>::merge(
            &mut self.security_context,
            other.security_context,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::Probe,
        > as crate::OptionableConvert>::merge(
            &mut self.startup_probe,
            other.startup_probe,
        )?;
        <Option<bool> as crate::OptionableConvert>::merge(&mut self.stdin, other.stdin)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.stdin_once, other.stdin_once)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.target_container_name,
            other.target_container_name,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.termination_message_path,
            other.termination_message_path,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.termination_message_policy,
            other.termination_message_policy,
        )?;
        <Option<bool> as crate::OptionableConvert>::merge(&mut self.tty, other.tty)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::VolumeDevice>,
        > as crate::OptionableConvert>::merge(
            &mut self.volume_devices,
            other.volume_devices,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::VolumeMount>,
        > as crate::OptionableConvert>::merge(
            &mut self.volume_mounts,
            other.volume_mounts,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.working_dir, other.working_dir)?;
        Ok(())
    }
}
