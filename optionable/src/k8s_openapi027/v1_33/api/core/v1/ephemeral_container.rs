#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// An EphemeralContainer is a temporary container that you may add to an existing Pod for user-initiated activities such as debugging. Ephemeral containers have no resource or scheduling guarantees, and they will not be restarted when they exit or when a Pod is removed or restarted. The kubelet may evict a Pod if an ephemeral container causes the Pod to exceed its resource allocation.
///
/// To add an ephemeral container, use the ephemeralcontainers subresource of an existing Pod. Ephemeral containers may not be removed or restarted.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EphemeralContainerAc {
    /// Arguments to the entrypoint. The image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<std::vec::Vec<std::string::String>>,
    /// Entrypoint array. Not executed within a shell. The image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<std::vec::Vec<std::string::String>>,
    /// List of environment variables to set in the container. Cannot be updated.
    pub env: Option<std::vec::Vec<::k8s_openapi027::api::core::v1::EnvVar>>,
    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_from: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EnvFromSource as crate::Optionable>::Optioned,
        >,
    >,
    /// Container image name. More info: https://kubernetes.io/docs/concepts/containers/images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<std::string::String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<std::string::String>,
    /// Lifecycle is not allowed for ephemeral containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<
        <::k8s_openapi027::api::core::v1::Lifecycle as crate::Optionable>::Optioned,
    >,
    /// Probes are not allowed for ephemeral containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<
        <::k8s_openapi027::api::core::v1::Probe as crate::Optionable>::Optioned,
    >,
    /// Name of the ephemeral container specified as a DNS_LABEL. This name must be unique among all containers, init containers and ephemeral containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Ports are not allowed for ephemeral containers.
    pub ports: Option<std::vec::Vec<::k8s_openapi027::api::core::v1::ContainerPort>>,
    /// Probes are not allowed for ephemeral containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<
        <::k8s_openapi027::api::core::v1::Probe as crate::Optionable>::Optioned,
    >,
    /// Resources resize policy for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_policy: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ContainerResizePolicy as crate::Optionable>::Optioned,
        >,
    >,
    /// Resources are not allowed for ephemeral containers. Ephemeral containers use spare resources already allocated to the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        <::k8s_openapi027::api::core::v1::ResourceRequirements as crate::Optionable>::Optioned,
    >,
    /// Restart policy for the container to manage the restart behavior of each container within a pod. This may only be set for init containers. You cannot set this field on ephemeral containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<std::string::String>,
    /// Optional: SecurityContext defines the security options the ephemeral container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<
        <::k8s_openapi027::api::core::v1::SecurityContext as crate::Optionable>::Optioned,
    >,
    /// Probes are not allowed for ephemeral containers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_probe: Option<
        <::k8s_openapi027::api::core::v1::Probe as crate::Optionable>::Optioned,
    >,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// If set, the name of the container from PodSpec that this ephemeral container targets. The ephemeral container will be run in the namespaces (IPC, PID, etc) of this container. If not set then the ephemeral container uses the namespaces configured in the Pod spec.
    ///
    /// The container runtime must implement support for this feature. If the runtime does not support namespace targeting then the result of setting this field is undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_container_name: Option<std::string::String>,
    /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_message_path: Option<std::string::String>,
    /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_message_policy: Option<std::string::String>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// volumeDevices is the list of block devices to be used by the container.
    pub volume_devices: Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::VolumeDevice>,
    >,
    /// Pod volumes to mount into the container's filesystem. Subpath mounts are not allowed for ephemeral containers. Cannot be updated.
    pub volume_mounts: Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::VolumeMount>,
    >,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EphemeralContainer {
    type Optioned = EphemeralContainerAc;
}
#[automatically_derived]
impl crate::Optionable for EphemeralContainerAc {
    type Optioned = EphemeralContainerAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EphemeralContainer {
    fn into_optioned(self) -> EphemeralContainerAc {
        EphemeralContainerAc {
            args: self.args,
            command: self.command,
            env: self.env,
            env_from: crate::OptionableConvert::into_optioned(self.env_from),
            image: self.image,
            image_pull_policy: self.image_pull_policy,
            lifecycle: crate::OptionableConvert::into_optioned(self.lifecycle),
            liveness_probe: crate::OptionableConvert::into_optioned(self.liveness_probe),
            name: Some(self.name),
            ports: self.ports,
            readiness_probe: crate::OptionableConvert::into_optioned(
                self.readiness_probe,
            ),
            resize_policy: crate::OptionableConvert::into_optioned(self.resize_policy),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            restart_policy: self.restart_policy,
            security_context: crate::OptionableConvert::into_optioned(
                self.security_context,
            ),
            startup_probe: crate::OptionableConvert::into_optioned(self.startup_probe),
            stdin: self.stdin,
            stdin_once: self.stdin_once,
            target_container_name: self.target_container_name,
            termination_message_path: self.termination_message_path,
            termination_message_policy: self.termination_message_policy,
            tty: self.tty,
            volume_devices: self.volume_devices,
            volume_mounts: self.volume_mounts,
            working_dir: self.working_dir,
        }
    }
    fn try_from_optioned(value: EphemeralContainerAc) -> Result<Self, crate::Error> {
        Ok(Self {
            args: value.args,
            command: value.command,
            env: value.env,
            env_from: crate::OptionableConvert::try_from_optioned(value.env_from)?,
            image: value.image,
            image_pull_policy: value.image_pull_policy,
            lifecycle: crate::OptionableConvert::try_from_optioned(value.lifecycle)?,
            liveness_probe: crate::OptionableConvert::try_from_optioned(
                value.liveness_probe,
            )?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            ports: value.ports,
            readiness_probe: crate::OptionableConvert::try_from_optioned(
                value.readiness_probe,
            )?,
            resize_policy: crate::OptionableConvert::try_from_optioned(
                value.resize_policy,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            restart_policy: value.restart_policy,
            security_context: crate::OptionableConvert::try_from_optioned(
                value.security_context,
            )?,
            startup_probe: crate::OptionableConvert::try_from_optioned(
                value.startup_probe,
            )?,
            stdin: value.stdin,
            stdin_once: value.stdin_once,
            target_container_name: value.target_container_name,
            termination_message_path: value.termination_message_path,
            termination_message_policy: value.termination_message_policy,
            tty: value.tty,
            volume_devices: value.volume_devices,
            volume_mounts: value.volume_mounts,
            working_dir: value.working_dir,
        })
    }
    fn merge(&mut self, other: EphemeralContainerAc) -> Result<(), crate::Error> {
        self.args = other.args;
        self.command = other.command;
        self.env = other.env;
        crate::OptionableConvert::merge(&mut self.env_from, other.env_from)?;
        self.image = other.image;
        self.image_pull_policy = other.image_pull_policy;
        crate::OptionableConvert::merge(&mut self.lifecycle, other.lifecycle)?;
        crate::OptionableConvert::merge(&mut self.liveness_probe, other.liveness_probe)?;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.ports = other.ports;
        crate::OptionableConvert::merge(
            &mut self.readiness_probe,
            other.readiness_probe,
        )?;
        crate::OptionableConvert::merge(&mut self.resize_policy, other.resize_policy)?;
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        self.restart_policy = other.restart_policy;
        crate::OptionableConvert::merge(
            &mut self.security_context,
            other.security_context,
        )?;
        crate::OptionableConvert::merge(&mut self.startup_probe, other.startup_probe)?;
        self.stdin = other.stdin;
        self.stdin_once = other.stdin_once;
        self.target_container_name = other.target_container_name;
        self.termination_message_path = other.termination_message_path;
        self.termination_message_policy = other.termination_message_policy;
        self.tty = other.tty;
        self.volume_devices = other.volume_devices;
        self.volume_mounts = other.volume_mounts;
        self.working_dir = other.working_dir;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EphemeralContainer>
for EphemeralContainerAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::EphemeralContainer,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EphemeralContainer, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EphemeralContainer,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
