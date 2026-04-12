#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A single application container that you want to run within a pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerAc {
    /// Arguments to the entrypoint. The container image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<std::vec::Vec<std::string::String>>,
    /// Entrypoint array. Not executed within a shell. The container image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<std::vec::Vec<std::string::String>>,
    /// List of environment variables to set in the container. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EnvVar as crate::Optionable>::Optioned,
        >,
    >,
    /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_from: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EnvFromSource as crate::Optionable>::Optioned,
        >,
    >,
    /// Container image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<std::string::String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<std::string::String>,
    /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<
        <::k8s_openapi027::api::core::v1::Lifecycle as crate::Optionable>::Optioned,
    >,
    /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<
        <::k8s_openapi027::api::core::v1::Probe as crate::Optionable>::Optioned,
    >,
    /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
    pub name: std::string::String,
    /// List of ports to expose from the container. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Modifying this array with strategic merge patch may corrupt the data. For more information See https://github.com/kubernetes/kubernetes/issues/108255. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ContainerPort as crate::Optionable>::Optioned,
        >,
    >,
    /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
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
    /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        <::k8s_openapi027::api::core::v1::ResourceRequirements as crate::Optionable>::Optioned,
    >,
    /// RestartPolicy defines the restart behavior of individual containers in a pod. This field may only be set for init containers, and the only allowed value is "Always". For non-init containers or when this field is not specified, the restart behavior is defined by the Pod's restart policy and the container type. Setting the RestartPolicy as "Always" for the init container will have the following effect: this init container will be continually restarted on exit until all regular containers have terminated. Once all regular containers have completed, all init containers with restartPolicy "Always" will be shut down. This lifecycle differs from normal init containers and is often referred to as a "sidecar" container. Although this init container still starts in the init container sequence, it does not wait for the container to complete before proceeding to the next init container. Instead, the next init container starts immediately after this init container is started, or after any startupProbe has successfully completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<std::string::String>,
    /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<
        <::k8s_openapi027::api::core::v1::SecurityContext as crate::Optionable>::Optioned,
    >,
    /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_devices: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::VolumeDevice as crate::Optionable>::Optioned,
        >,
    >,
    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::VolumeMount as crate::Optionable>::Optioned,
        >,
    >,
    /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Container {
    type Optioned = ContainerAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerAc {
    type Optioned = ContainerAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Container {
    fn into_optioned(self) -> ContainerAc {
        ContainerAc {
            args: self.args,
            command: self.command,
            env: crate::OptionableConvert::into_optioned(self.env),
            env_from: crate::OptionableConvert::into_optioned(self.env_from),
            image: self.image,
            image_pull_policy: self.image_pull_policy,
            lifecycle: crate::OptionableConvert::into_optioned(self.lifecycle),
            liveness_probe: crate::OptionableConvert::into_optioned(self.liveness_probe),
            name: self.name,
            ports: crate::OptionableConvert::into_optioned(self.ports),
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
            termination_message_path: self.termination_message_path,
            termination_message_policy: self.termination_message_policy,
            tty: self.tty,
            volume_devices: crate::OptionableConvert::into_optioned(self.volume_devices),
            volume_mounts: crate::OptionableConvert::into_optioned(self.volume_mounts),
            working_dir: self.working_dir,
        }
    }
    fn try_from_optioned(value: ContainerAc) -> Result<Self, crate::Error> {
        Ok(Self {
            args: value.args,
            command: value.command,
            env: crate::OptionableConvert::try_from_optioned(value.env)?,
            env_from: crate::OptionableConvert::try_from_optioned(value.env_from)?,
            image: value.image,
            image_pull_policy: value.image_pull_policy,
            lifecycle: crate::OptionableConvert::try_from_optioned(value.lifecycle)?,
            liveness_probe: crate::OptionableConvert::try_from_optioned(
                value.liveness_probe,
            )?,
            name: value.name,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
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
            termination_message_path: value.termination_message_path,
            termination_message_policy: value.termination_message_policy,
            tty: value.tty,
            volume_devices: crate::OptionableConvert::try_from_optioned(
                value.volume_devices,
            )?,
            volume_mounts: crate::OptionableConvert::try_from_optioned(
                value.volume_mounts,
            )?,
            working_dir: value.working_dir,
        })
    }
    fn merge(&mut self, other: ContainerAc) -> Result<(), crate::Error> {
        if self.args.is_none() {
            self.args = other.args;
        }
        if let Some(other_value) = other.args {
            self.args = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.command.is_none() {
            self.command = other.command;
        }
        if let Some(other_value) = other.command {
            self.command = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.env.is_none() {
            self.env = other.env;
        }
        if let Some(other_value) = other.env {
            crate::merge::try_merge_optioned_map(&mut self.env, other_value)?;
        }
        if self.env_from.is_none() {
            self.env_from = other.env_from;
        }
        if let Some(other_value) = other.env_from {
            self.env_from = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.image.is_none() {
            self.image = other.image;
        }
        if let Some(other_value) = other.image {
            crate::OptionableConvert::merge(&mut self.image, other_value)?;
        }
        if self.image_pull_policy.is_none() {
            self.image_pull_policy = other.image_pull_policy;
        }
        if let Some(other_value) = other.image_pull_policy {
            crate::OptionableConvert::merge(&mut self.image_pull_policy, other_value)?;
        }
        if self.lifecycle.is_none() {
            self.lifecycle = other.lifecycle;
        }
        if let Some(other_value) = other.lifecycle {
            crate::OptionableConvert::merge(&mut self.lifecycle, other_value)?;
        }
        if self.liveness_probe.is_none() {
            self.liveness_probe = other.liveness_probe;
        }
        if let Some(other_value) = other.liveness_probe {
            crate::OptionableConvert::merge(&mut self.liveness_probe, other_value)?;
        }
        self.name = other.name;
        if self.ports.is_none() {
            self.ports = other.ports;
        }
        if let Some(other_value) = other.ports {
            crate::merge::try_merge_optioned_map(&mut self.ports, other_value)?;
        }
        if self.readiness_probe.is_none() {
            self.readiness_probe = other.readiness_probe;
        }
        if let Some(other_value) = other.readiness_probe {
            crate::OptionableConvert::merge(&mut self.readiness_probe, other_value)?;
        }
        if self.resize_policy.is_none() {
            self.resize_policy = other.resize_policy;
        }
        if let Some(other_value) = other.resize_policy {
            self.resize_policy = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.resources.is_none() {
            self.resources = other.resources;
        }
        if let Some(other_value) = other.resources {
            crate::OptionableConvert::merge(&mut self.resources, other_value)?;
        }
        if self.restart_policy.is_none() {
            self.restart_policy = other.restart_policy;
        }
        if let Some(other_value) = other.restart_policy {
            crate::OptionableConvert::merge(&mut self.restart_policy, other_value)?;
        }
        if self.security_context.is_none() {
            self.security_context = other.security_context;
        }
        if let Some(other_value) = other.security_context {
            crate::OptionableConvert::merge(&mut self.security_context, other_value)?;
        }
        if self.startup_probe.is_none() {
            self.startup_probe = other.startup_probe;
        }
        if let Some(other_value) = other.startup_probe {
            crate::OptionableConvert::merge(&mut self.startup_probe, other_value)?;
        }
        if self.stdin.is_none() {
            self.stdin = other.stdin;
        }
        if let Some(other_value) = other.stdin {
            crate::OptionableConvert::merge(&mut self.stdin, other_value)?;
        }
        if self.stdin_once.is_none() {
            self.stdin_once = other.stdin_once;
        }
        if let Some(other_value) = other.stdin_once {
            crate::OptionableConvert::merge(&mut self.stdin_once, other_value)?;
        }
        if self.termination_message_path.is_none() {
            self.termination_message_path = other.termination_message_path;
        }
        if let Some(other_value) = other.termination_message_path {
            crate::OptionableConvert::merge(
                &mut self.termination_message_path,
                other_value,
            )?;
        }
        if self.termination_message_policy.is_none() {
            self.termination_message_policy = other.termination_message_policy;
        }
        if let Some(other_value) = other.termination_message_policy {
            crate::OptionableConvert::merge(
                &mut self.termination_message_policy,
                other_value,
            )?;
        }
        if self.tty.is_none() {
            self.tty = other.tty;
        }
        if let Some(other_value) = other.tty {
            crate::OptionableConvert::merge(&mut self.tty, other_value)?;
        }
        if self.volume_devices.is_none() {
            self.volume_devices = other.volume_devices;
        }
        if let Some(other_value) = other.volume_devices {
            crate::merge::try_merge_optioned_map(&mut self.volume_devices, other_value)?;
        }
        if self.volume_mounts.is_none() {
            self.volume_mounts = other.volume_mounts;
        }
        if let Some(other_value) = other.volume_mounts {
            crate::merge::try_merge_optioned_map(&mut self.volume_mounts, other_value)?;
        }
        if self.working_dir.is_none() {
            self.working_dir = other.working_dir;
        }
        if let Some(other_value) = other.working_dir {
            crate::OptionableConvert::merge(&mut self.working_dir, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::Container {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Container> for ContainerAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Container) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Container, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Container,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
