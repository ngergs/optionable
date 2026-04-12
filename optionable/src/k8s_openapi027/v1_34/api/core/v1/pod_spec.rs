#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodSpec is a description of a pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodSpecAc {
    /// Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_deadline_seconds: Option<i64>,
    /// If specified, the pod's scheduling constraints
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity: Option<
        <::k8s_openapi027::api::core::v1::Affinity as crate::Optionable>::Optioned,
    >,
    /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automount_service_account_token: Option<bool>,
    /// List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Container as crate::Optionable>::Optioned,
        >,
    >,
    /// Specifies the DNS parameters of a pod. Parameters specified here will be merged to the generated DNS configuration based on DNSPolicy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<
        <::k8s_openapi027::api::core::v1::PodDNSConfig as crate::Optionable>::Optioned,
    >,
    /// Set DNS policy for the pod. Defaults to "ClusterFirst". Valid values are 'ClusterFirstWithHostNet', 'ClusterFirst', 'Default' or 'None'. DNS parameters given in DNSConfig will be merged with the policy selected with DNSPolicy. To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<std::string::String>,
    /// EnableServiceLinks indicates whether information about services should be injected into pod's environment variables, matching the syntax of Docker links. Optional: Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_service_links: Option<bool>,
    /// List of ephemeral containers run in this pod. Ephemeral containers may be run in an existing pod to perform user-initiated actions such as debugging. This list cannot be specified when creating a pod, and it cannot be modified by updating the pod spec. In order to add an ephemeral container to an existing pod, use the pod's ephemeralcontainers subresource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_containers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EphemeralContainer as crate::Optionable>::Optioned,
        >,
    >,
    /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_aliases: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::HostAlias as crate::Optionable>::Optioned,
        >,
    >,
    /// Use the host's ipc namespace. Optional: Default to false.
    #[serde(rename = "hostIPC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ipc: Option<bool>,
    /// Host networking requested for this pod. Use the host's network namespace. When using HostNetwork you should specify ports so the scheduler is aware. When `hostNetwork` is true, specified `hostPort` fields in port definitions must match `containerPort`, and unspecified `hostPort` fields in port definitions are defaulted to match `containerPort`. Default to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    /// Use the host's pid namespace. Optional: Default to false.
    #[serde(rename = "hostPID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_pid: Option<bool>,
    /// Use the host's user namespace. Optional: Default to true. If set to true or not present, the pod will be run in the host user namespace, useful for when the pod needs a feature only available to the host user namespace, such as loading a kernel module with CAP_SYS_MODULE. When set to false, a new userns is created for the pod. Setting false is useful for mitigating container breakout vulnerabilities even allowing users to run their containers as root without actually having root privileges on the host. This field is alpha-level and is only honored by servers that enable the UserNamespacesSupport feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_users: Option<bool>,
    /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<std::string::String>,
    /// HostnameOverride specifies an explicit override for the pod's hostname as perceived by the pod. This field only specifies the pod's hostname and does not affect its DNS records. When this field is set to a non-empty string: - It takes precedence over the values set in `hostname` and `subdomain`. - The Pod's hostname will be set to this value. - `setHostnameAsFQDN` must be nil or set to false. - `hostNetwork` must be set to false.
    ///
    /// This field must be a valid DNS subdomain as defined in RFC 1123 and contain at most 64 characters. Requires the HostnameOverride feature gate to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_override: Option<std::string::String>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
        >,
    >,
    /// List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Container as crate::Optionable>::Optioned,
        >,
    >,
    /// NodeName indicates in which node this pod is scheduled. If empty, this pod is a candidate for scheduling by the scheduler defined in schedulerName. Once this field is set, the kubelet for this node becomes responsible for the lifecycle of this pod. This field should not be used to express a desire for the pod to be scheduled on a specific node. https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/#nodename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// Specifies the OS of the containers in the pod. Some pod and container fields are restricted if this is set.
    ///
    /// If the OS field is set to linux, the following fields must be unset: -securityContext.windowsOptions
    ///
    /// If the OS field is set to windows, following fields must be unset: - spec.hostPID - spec.hostIPC - spec.hostUsers - spec.resources - spec.securityContext.appArmorProfile - spec.securityContext.seLinuxOptions - spec.securityContext.seccompProfile - spec.securityContext.fsGroup - spec.securityContext.fsGroupChangePolicy - spec.securityContext.sysctls - spec.shareProcessNamespace - spec.securityContext.runAsUser - spec.securityContext.runAsGroup - spec.securityContext.supplementalGroups - spec.securityContext.supplementalGroupsPolicy - spec.containers\[*\].securityContext.appArmorProfile - spec.containers\[*\].securityContext.seLinuxOptions - spec.containers\[*\].securityContext.seccompProfile - spec.containers\[*\].securityContext.capabilities - spec.containers\[*\].securityContext.readOnlyRootFilesystem - spec.containers\[*\].securityContext.privileged - spec.containers\[*\].securityContext.allowPrivilegeEscalation - spec.containers\[*\].securityContext.procMount - spec.containers\[*\].securityContext.runAsUser - spec.containers\[*\].securityContext.runAsGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<
        <::k8s_openapi027::api::core::v1::PodOS as crate::Optionable>::Optioned,
    >,
    /// Overhead represents the resource overhead associated with running a pod for a given RuntimeClass. This field will be autopopulated at admission time by the RuntimeClass admission controller. If the RuntimeClass admission controller is enabled, overhead must not be set in Pod create requests. The RuntimeClass admission controller will reject Pod create requests which have the overhead already set. If RuntimeClass is configured and selected in the PodSpec, Overhead will be set to the value defined in the corresponding RuntimeClass, otherwise it will remain unset and treated as zero. More info: https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// PreemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<std::string::String>,
    /// The priority value. Various system components use this field to find the priority of the pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// If specified, indicates the pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_class_name: Option<std::string::String>,
    /// If specified, all readiness gates will be evaluated for pod readiness. A pod is ready when all its containers are ready AND all conditions specified in the readiness gates have status equal to "True" More info: https://git.k8s.io/enhancements/keps/sig-network/580-pod-readiness-gates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readiness_gates: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodReadinessGate as crate::Optionable>::Optioned,
        >,
    >,
    /// ResourceClaims defines which ResourceClaims must be allocated and reserved before the Pod is allowed to start. The resources will be made available to those containers which consume them by name.
    ///
    /// This is an alpha field and requires enabling the DynamicResourceAllocation feature gate.
    ///
    /// This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claims: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodResourceClaim as crate::Optionable>::Optioned,
        >,
    >,
    /// Resources is the total amount of CPU and Memory resources required by all containers in the pod. It supports specifying Requests and Limits for "cpu", "memory" and "hugepages-" resource names only. ResourceClaims are not supported.
    ///
    /// This field enables fine-grained control over resource allocation for the entire pod, allowing resource sharing among containers in a pod.
    ///
    /// This is an alpha field and requires enabling the PodLevelResources feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        <::k8s_openapi027::api::core::v1::ResourceRequirements as crate::Optionable>::Optioned,
    >,
    /// Restart policy for all containers within the pod. One of Always, OnFailure, Never. In some contexts, only a subset of those values may be permitted. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<std::string::String>,
    /// RuntimeClassName refers to a RuntimeClass object in the node.k8s.io group, which should be used to run this pod.  If no RuntimeClass resource matches the named class, the pod will not be run. If unset or empty, the "legacy" RuntimeClass will be used, which is an implicit class with an empty definition that uses the default runtime handler. More info: https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_class_name: Option<std::string::String>,
    /// If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_name: Option<std::string::String>,
    /// SchedulingGates is an opaque list of values that if specified will block scheduling the pod. If schedulingGates is not empty, the pod will stay in the SchedulingGated state and the scheduler will not attempt to schedule the pod.
    ///
    /// SchedulingGates can only be set at pod creation time, and be removed only afterwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_gates: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodSchedulingGate as crate::Optionable>::Optioned,
        >,
    >,
    /// SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<
        <::k8s_openapi027::api::core::v1::PodSecurityContext as crate::Optionable>::Optioned,
    >,
    /// DeprecatedServiceAccount is a deprecated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<std::string::String>,
    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<std::string::String>,
    /// If true the pod's hostname will be configured as the pod's FQDN, rather than the leaf name (the default). In Linux containers, this means setting the FQDN in the hostname field of the kernel (the nodename field of struct utsname). In Windows containers, this means setting the registry value of hostname for the registry key HKEY_LOCAL_MACHINE\\\\SYSTEM\\\\CurrentControlSet\\\\Services\\\\Tcpip\\\\Parameters to FQDN. If a pod does not have FQDN, this has no effect. Default to false.
    #[serde(rename = "setHostnameAsFQDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_hostname_as_fqdn: Option<bool>,
    /// Share a single process namespace between all of the containers in a pod. When this is set containers will be able to view and signal processes from other containers in the same pod, and the first process in each container will not be assigned PID 1. HostPID and ShareProcessNamespace cannot both be set. Optional: Default to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,
    /// If specified, the fully qualified Pod hostname will be "\<hostname\>.\<subdomain\>.\<pod namespace\>.svc.\<cluster domain\>". If not specified, the pod will not have a domainname at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<std::string::String>,
    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    /// If specified, the pod's tolerations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Toleration as crate::Optionable>::Optioned,
        >,
    >,
    /// TopologySpreadConstraints describes how a group of pods ought to spread across topology domains. Scheduler will schedule pods in a way which abides by the constraints. All topologySpreadConstraints are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_spread_constraints: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::TopologySpreadConstraint as crate::Optionable>::Optioned,
        >,
    >,
    /// List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::Volume as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodSpec {
    type Optioned = PodSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodSpecAc {
    type Optioned = PodSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodSpec {
    fn into_optioned(self) -> PodSpecAc {
        PodSpecAc {
            active_deadline_seconds: self.active_deadline_seconds,
            affinity: crate::OptionableConvert::into_optioned(self.affinity),
            automount_service_account_token: self.automount_service_account_token,
            containers: Some(crate::OptionableConvert::into_optioned(self.containers)),
            dns_config: crate::OptionableConvert::into_optioned(self.dns_config),
            dns_policy: self.dns_policy,
            enable_service_links: self.enable_service_links,
            ephemeral_containers: crate::OptionableConvert::into_optioned(
                self.ephemeral_containers,
            ),
            host_aliases: crate::OptionableConvert::into_optioned(self.host_aliases),
            host_ipc: self.host_ipc,
            host_network: self.host_network,
            host_pid: self.host_pid,
            host_users: self.host_users,
            hostname: self.hostname,
            hostname_override: self.hostname_override,
            image_pull_secrets: crate::OptionableConvert::into_optioned(
                self.image_pull_secrets,
            ),
            init_containers: crate::OptionableConvert::into_optioned(
                self.init_containers,
            ),
            node_name: self.node_name,
            node_selector: self.node_selector,
            os: crate::OptionableConvert::into_optioned(self.os),
            overhead: crate::OptionableConvert::into_optioned(self.overhead),
            preemption_policy: self.preemption_policy,
            priority: self.priority,
            priority_class_name: self.priority_class_name,
            readiness_gates: crate::OptionableConvert::into_optioned(
                self.readiness_gates,
            ),
            resource_claims: crate::OptionableConvert::into_optioned(
                self.resource_claims,
            ),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            restart_policy: self.restart_policy,
            runtime_class_name: self.runtime_class_name,
            scheduler_name: self.scheduler_name,
            scheduling_gates: crate::OptionableConvert::into_optioned(
                self.scheduling_gates,
            ),
            security_context: crate::OptionableConvert::into_optioned(
                self.security_context,
            ),
            service_account: self.service_account,
            service_account_name: self.service_account_name,
            set_hostname_as_fqdn: self.set_hostname_as_fqdn,
            share_process_namespace: self.share_process_namespace,
            subdomain: self.subdomain,
            termination_grace_period_seconds: self.termination_grace_period_seconds,
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
            topology_spread_constraints: crate::OptionableConvert::into_optioned(
                self.topology_spread_constraints,
            ),
            volumes: crate::OptionableConvert::into_optioned(self.volumes),
        }
    }
    fn try_from_optioned(value: PodSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            active_deadline_seconds: value.active_deadline_seconds,
            affinity: crate::OptionableConvert::try_from_optioned(value.affinity)?,
            automount_service_account_token: value.automount_service_account_token,
            containers: crate::OptionableConvert::try_from_optioned(
                value
                    .containers
                    .ok_or(crate::Error {
                        missing_field: "containers",
                    })?,
            )?,
            dns_config: crate::OptionableConvert::try_from_optioned(value.dns_config)?,
            dns_policy: value.dns_policy,
            enable_service_links: value.enable_service_links,
            ephemeral_containers: crate::OptionableConvert::try_from_optioned(
                value.ephemeral_containers,
            )?,
            host_aliases: crate::OptionableConvert::try_from_optioned(
                value.host_aliases,
            )?,
            host_ipc: value.host_ipc,
            host_network: value.host_network,
            host_pid: value.host_pid,
            host_users: value.host_users,
            hostname: value.hostname,
            hostname_override: value.hostname_override,
            image_pull_secrets: crate::OptionableConvert::try_from_optioned(
                value.image_pull_secrets,
            )?,
            init_containers: crate::OptionableConvert::try_from_optioned(
                value.init_containers,
            )?,
            node_name: value.node_name,
            node_selector: value.node_selector,
            os: crate::OptionableConvert::try_from_optioned(value.os)?,
            overhead: crate::OptionableConvert::try_from_optioned(value.overhead)?,
            preemption_policy: value.preemption_policy,
            priority: value.priority,
            priority_class_name: value.priority_class_name,
            readiness_gates: crate::OptionableConvert::try_from_optioned(
                value.readiness_gates,
            )?,
            resource_claims: crate::OptionableConvert::try_from_optioned(
                value.resource_claims,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            restart_policy: value.restart_policy,
            runtime_class_name: value.runtime_class_name,
            scheduler_name: value.scheduler_name,
            scheduling_gates: crate::OptionableConvert::try_from_optioned(
                value.scheduling_gates,
            )?,
            security_context: crate::OptionableConvert::try_from_optioned(
                value.security_context,
            )?,
            service_account: value.service_account,
            service_account_name: value.service_account_name,
            set_hostname_as_fqdn: value.set_hostname_as_fqdn,
            share_process_namespace: value.share_process_namespace,
            subdomain: value.subdomain,
            termination_grace_period_seconds: value.termination_grace_period_seconds,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
            topology_spread_constraints: crate::OptionableConvert::try_from_optioned(
                value.topology_spread_constraints,
            )?,
            volumes: crate::OptionableConvert::try_from_optioned(value.volumes)?,
        })
    }
    fn merge(&mut self, other: PodSpecAc) -> Result<(), crate::Error> {
        if self.active_deadline_seconds.is_none() {
            self.active_deadline_seconds = crate::OptionableConvert::try_from_optioned(
                other.active_deadline_seconds,
            )?;
        } else if let Some(self_value) = self.active_deadline_seconds.as_mut()
            && let Some(other_value) = other.active_deadline_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.affinity.is_none() {
            self.affinity = crate::OptionableConvert::try_from_optioned(other.affinity)?;
        } else if let Some(self_value) = self.affinity.as_mut()
            && let Some(other_value) = other.affinity
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.automount_service_account_token.is_none() {
            self.automount_service_account_token = crate::OptionableConvert::try_from_optioned(
                other.automount_service_account_token,
            )?;
        } else if let Some(self_value) = self.automount_service_account_token.as_mut()
            && let Some(other_value) = other.automount_service_account_token
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.containers {
            crate::merge::try_merge_optioned_map(&mut self.containers, other_value)?;
        }
        if self.dns_config.is_none() {
            self.dns_config = crate::OptionableConvert::try_from_optioned(
                other.dns_config,
            )?;
        } else if let Some(self_value) = self.dns_config.as_mut()
            && let Some(other_value) = other.dns_config
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.dns_policy.is_none() {
            self.dns_policy = crate::OptionableConvert::try_from_optioned(
                other.dns_policy,
            )?;
        } else if let Some(self_value) = self.dns_policy.as_mut()
            && let Some(other_value) = other.dns_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.enable_service_links.is_none() {
            self.enable_service_links = crate::OptionableConvert::try_from_optioned(
                other.enable_service_links,
            )?;
        } else if let Some(self_value) = self.enable_service_links.as_mut()
            && let Some(other_value) = other.enable_service_links
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.ephemeral_containers.is_none() {
            self.ephemeral_containers = crate::OptionableConvert::try_from_optioned(
                other.ephemeral_containers,
            )?;
        } else if let Some(self_value) = self.ephemeral_containers.as_mut()
            && let Some(other_value) = other.ephemeral_containers
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.host_aliases.is_none() {
            self.host_aliases = crate::OptionableConvert::try_from_optioned(
                other.host_aliases,
            )?;
        } else if let Some(self_value) = self.host_aliases.as_mut()
            && let Some(other_value) = other.host_aliases
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.host_ipc.is_none() {
            self.host_ipc = crate::OptionableConvert::try_from_optioned(other.host_ipc)?;
        } else if let Some(self_value) = self.host_ipc.as_mut()
            && let Some(other_value) = other.host_ipc
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.host_network.is_none() {
            self.host_network = crate::OptionableConvert::try_from_optioned(
                other.host_network,
            )?;
        } else if let Some(self_value) = self.host_network.as_mut()
            && let Some(other_value) = other.host_network
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.host_pid.is_none() {
            self.host_pid = crate::OptionableConvert::try_from_optioned(other.host_pid)?;
        } else if let Some(self_value) = self.host_pid.as_mut()
            && let Some(other_value) = other.host_pid
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.host_users.is_none() {
            self.host_users = crate::OptionableConvert::try_from_optioned(
                other.host_users,
            )?;
        } else if let Some(self_value) = self.host_users.as_mut()
            && let Some(other_value) = other.host_users
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.hostname.is_none() {
            self.hostname = crate::OptionableConvert::try_from_optioned(other.hostname)?;
        } else if let Some(self_value) = self.hostname.as_mut()
            && let Some(other_value) = other.hostname
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.hostname_override.is_none() {
            self.hostname_override = crate::OptionableConvert::try_from_optioned(
                other.hostname_override,
            )?;
        } else if let Some(self_value) = self.hostname_override.as_mut()
            && let Some(other_value) = other.hostname_override
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.image_pull_secrets.is_none() {
            self.image_pull_secrets = crate::OptionableConvert::try_from_optioned(
                other.image_pull_secrets,
            )?;
        } else if let Some(self_value) = self.image_pull_secrets.as_mut()
            && let Some(other_value) = other.image_pull_secrets
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.init_containers.is_none() {
            self.init_containers = crate::OptionableConvert::try_from_optioned(
                other.init_containers,
            )?;
        } else if let Some(self_value) = self.init_containers.as_mut()
            && let Some(other_value) = other.init_containers
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.node_name.is_none() {
            self.node_name = crate::OptionableConvert::try_from_optioned(
                other.node_name,
            )?;
        } else if let Some(self_value) = self.node_name.as_mut()
            && let Some(other_value) = other.node_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.node_selector.is_none() {
            self.node_selector = crate::OptionableConvert::try_from_optioned(
                other.node_selector,
            )?;
        } else if let Some(self_value) = self.node_selector.as_mut()
            && let Some(other_value) = other.node_selector
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.os.is_none() {
            self.os = crate::OptionableConvert::try_from_optioned(other.os)?;
        } else if let Some(self_value) = self.os.as_mut()
            && let Some(other_value) = other.os
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.overhead.is_none() {
            self.overhead = crate::OptionableConvert::try_from_optioned(other.overhead)?;
        } else if let Some(self_value) = self.overhead.as_mut()
            && let Some(other_value) = other.overhead
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.preemption_policy.is_none() {
            self.preemption_policy = crate::OptionableConvert::try_from_optioned(
                other.preemption_policy,
            )?;
        } else if let Some(self_value) = self.preemption_policy.as_mut()
            && let Some(other_value) = other.preemption_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.priority.is_none() {
            self.priority = crate::OptionableConvert::try_from_optioned(other.priority)?;
        } else if let Some(self_value) = self.priority.as_mut()
            && let Some(other_value) = other.priority
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.priority_class_name.is_none() {
            self.priority_class_name = crate::OptionableConvert::try_from_optioned(
                other.priority_class_name,
            )?;
        } else if let Some(self_value) = self.priority_class_name.as_mut()
            && let Some(other_value) = other.priority_class_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.readiness_gates.is_none() {
            self.readiness_gates = crate::OptionableConvert::try_from_optioned(
                other.readiness_gates,
            )?;
        } else if let Some(self_value) = self.readiness_gates.as_mut()
            && let Some(other_value) = other.readiness_gates
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.resource_claims.is_none() {
            self.resource_claims = crate::OptionableConvert::try_from_optioned(
                other.resource_claims,
            )?;
        } else if let Some(self_value) = self.resource_claims.as_mut()
            && let Some(other_value) = other.resource_claims
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.resources.is_none() {
            self.resources = crate::OptionableConvert::try_from_optioned(
                other.resources,
            )?;
        } else if let Some(self_value) = self.resources.as_mut()
            && let Some(other_value) = other.resources
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.restart_policy.is_none() {
            self.restart_policy = crate::OptionableConvert::try_from_optioned(
                other.restart_policy,
            )?;
        } else if let Some(self_value) = self.restart_policy.as_mut()
            && let Some(other_value) = other.restart_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.runtime_class_name.is_none() {
            self.runtime_class_name = crate::OptionableConvert::try_from_optioned(
                other.runtime_class_name,
            )?;
        } else if let Some(self_value) = self.runtime_class_name.as_mut()
            && let Some(other_value) = other.runtime_class_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.scheduler_name.is_none() {
            self.scheduler_name = crate::OptionableConvert::try_from_optioned(
                other.scheduler_name,
            )?;
        } else if let Some(self_value) = self.scheduler_name.as_mut()
            && let Some(other_value) = other.scheduler_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.scheduling_gates.is_none() {
            self.scheduling_gates = crate::OptionableConvert::try_from_optioned(
                other.scheduling_gates,
            )?;
        } else if let Some(self_value) = self.scheduling_gates.as_mut()
            && let Some(other_value) = other.scheduling_gates
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.security_context.is_none() {
            self.security_context = crate::OptionableConvert::try_from_optioned(
                other.security_context,
            )?;
        } else if let Some(self_value) = self.security_context.as_mut()
            && let Some(other_value) = other.security_context
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.service_account.is_none() {
            self.service_account = crate::OptionableConvert::try_from_optioned(
                other.service_account,
            )?;
        } else if let Some(self_value) = self.service_account.as_mut()
            && let Some(other_value) = other.service_account
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.service_account_name.is_none() {
            self.service_account_name = crate::OptionableConvert::try_from_optioned(
                other.service_account_name,
            )?;
        } else if let Some(self_value) = self.service_account_name.as_mut()
            && let Some(other_value) = other.service_account_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.set_hostname_as_fqdn.is_none() {
            self.set_hostname_as_fqdn = crate::OptionableConvert::try_from_optioned(
                other.set_hostname_as_fqdn,
            )?;
        } else if let Some(self_value) = self.set_hostname_as_fqdn.as_mut()
            && let Some(other_value) = other.set_hostname_as_fqdn
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.share_process_namespace.is_none() {
            self.share_process_namespace = crate::OptionableConvert::try_from_optioned(
                other.share_process_namespace,
            )?;
        } else if let Some(self_value) = self.share_process_namespace.as_mut()
            && let Some(other_value) = other.share_process_namespace
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.subdomain.is_none() {
            self.subdomain = crate::OptionableConvert::try_from_optioned(
                other.subdomain,
            )?;
        } else if let Some(self_value) = self.subdomain.as_mut()
            && let Some(other_value) = other.subdomain
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.termination_grace_period_seconds.is_none() {
            self.termination_grace_period_seconds = crate::OptionableConvert::try_from_optioned(
                other.termination_grace_period_seconds,
            )?;
        } else if let Some(self_value) = self.termination_grace_period_seconds.as_mut()
            && let Some(other_value) = other.termination_grace_period_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.tolerations.is_none() {
            self.tolerations = crate::OptionableConvert::try_from_optioned(
                other.tolerations,
            )?;
        } else if let Some(self_value) = self.tolerations.as_mut()
            && let Some(other_value) = other.tolerations
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.topology_spread_constraints.is_none() {
            self.topology_spread_constraints = crate::OptionableConvert::try_from_optioned(
                other.topology_spread_constraints,
            )?;
        } else if let Some(self_value) = self.topology_spread_constraints.as_mut()
            && let Some(other_value) = other.topology_spread_constraints
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        if self.volumes.is_none() {
            self.volumes = crate::OptionableConvert::try_from_optioned(other.volumes)?;
        } else if let Some(self_value) = self.volumes.as_mut()
            && let Some(other_value) = other.volumes
        {
            crate::merge::try_merge_optioned_map(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodSpec> for PodSpecAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
