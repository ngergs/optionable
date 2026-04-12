#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodStatus represents information about the status of a pod. Status may trail the actual state of a system, especially if the node that hosts the pod cannot contact the control plane.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodStatusAc {
    /// Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// Statuses of containers in this pod. Each container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_statuses: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ContainerStatus as crate::Optionable>::Optioned,
        >,
    >,
    /// Statuses for any ephemeral containers that have run in this pod. Each ephemeral container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_container_statuses: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ContainerStatus as crate::Optionable>::Optioned,
        >,
    >,
    /// hostIP holds the IP address of the host to which the pod is assigned. Empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns mean that HostIP will not be updated even if there is a node is assigned to pod
    #[serde(rename = "hostIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<std::string::String>,
    /// hostIPs holds the IP addresses allocated to the host. If this field is specified, the first entry must match the hostIP field. This list is empty if the pod has not started yet. A pod can be assigned to a node that has a problem in kubelet which in turns means that HostIPs will not be updated even if there is a node is assigned to this pod.
    #[serde(rename = "hostIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ips: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::HostIP as crate::Optionable>::Optioned,
        >,
    >,
    /// Statuses of init containers in this pod. The most recent successful non-restartable init container will have ready = true, the most recently started container will have startTime set. Each init container in the pod should have at most one status in this list, and all statuses should be for containers in the pod. However this is not enforced. If a status for a non-existent container is present in the list, or the list has duplicate names, the behavior of various Kubernetes components is not defined and those statuses might be ignored. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-and-container-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_container_statuses: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ContainerStatus as crate::Optionable>::Optioned,
        >,
    >,
    /// A human readable message indicating details about why the pod is in this condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominated_node_name: Option<std::string::String>,
    /// If set, this represents the .metadata.generation that the pod status was set based upon. This is an alpha field. Enable PodObservedGenerationTracking to be able to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    /// The phase of a Pod is a simple, high-level summary of where the Pod is in its lifecycle. The conditions array, the reason and message fields, and the individual container status arrays contain more detail about the pod's status. There are five possible phase values:
    ///
    /// Pending: The pod has been accepted by the Kubernetes system, but one or more of the container images has not been created. This includes time before being scheduled as well as time spent downloading images over the network, which could take a while. Running: The pod has been bound to a node, and all of the containers have been created. At least one container is still running, or is in the process of starting or restarting. Succeeded: All containers in the pod have terminated in success, and will not be restarted. Failed: All containers in the pod have terminated, and at least one container has terminated in failure. The container either exited with non-zero status or was terminated by the system. Unknown: For some reason the state of the pod could not be obtained, typically due to an error in communicating with the host of the pod.
    ///
    /// More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<std::string::String>,
    /// podIP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.
    #[serde(rename = "podIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<std::string::String>,
    /// podIPs holds the IP addresses allocated to the pod. If this field is specified, the 0th entry must match the podIP field. Pods may be allocated at most 1 value for each of IPv4 and IPv6. This list is empty if no IPs have been allocated yet.
    #[serde(rename = "podIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ips: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodIP as crate::Optionable>::Optioned,
        >,
    >,
    /// The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-qos/#quality-of-service-classes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos_class: Option<std::string::String>,
    /// A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// Status of resources resize desired for pod's containers. It is empty if no resources resize is pending. Any changes to container resources will automatically set this to "Proposed" Deprecated: Resize status is moved to two pod conditions PodResizePending and PodResizeInProgress. PodResizePending will track states where the spec has been resized, but the Kubelet has not yet allocated the resources. PodResizeInProgress will track in-progress resizes, and should be present whenever allocated resources != acknowledged resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize: Option<std::string::String>,
    /// Status of resource claims.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_statuses: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodResourceClaimStatus as crate::Optionable>::Optioned,
        >,
    >,
    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodStatus {
    type Optioned = PodStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodStatusAc {
    type Optioned = PodStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodStatus {
    fn into_optioned(self) -> PodStatusAc {
        PodStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            container_statuses: crate::OptionableConvert::into_optioned(
                self.container_statuses,
            ),
            ephemeral_container_statuses: crate::OptionableConvert::into_optioned(
                self.ephemeral_container_statuses,
            ),
            host_ip: self.host_ip,
            host_ips: crate::OptionableConvert::into_optioned(self.host_ips),
            init_container_statuses: crate::OptionableConvert::into_optioned(
                self.init_container_statuses,
            ),
            message: self.message,
            nominated_node_name: self.nominated_node_name,
            observed_generation: self.observed_generation,
            phase: self.phase,
            pod_ip: self.pod_ip,
            pod_ips: crate::OptionableConvert::into_optioned(self.pod_ips),
            qos_class: self.qos_class,
            reason: self.reason,
            resize: self.resize,
            resource_claim_statuses: crate::OptionableConvert::into_optioned(
                self.resource_claim_statuses,
            ),
            start_time: crate::OptionableConvert::into_optioned(self.start_time),
        }
    }
    fn try_from_optioned(value: PodStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            container_statuses: crate::OptionableConvert::try_from_optioned(
                value.container_statuses,
            )?,
            ephemeral_container_statuses: crate::OptionableConvert::try_from_optioned(
                value.ephemeral_container_statuses,
            )?,
            host_ip: value.host_ip,
            host_ips: crate::OptionableConvert::try_from_optioned(value.host_ips)?,
            init_container_statuses: crate::OptionableConvert::try_from_optioned(
                value.init_container_statuses,
            )?,
            message: value.message,
            nominated_node_name: value.nominated_node_name,
            observed_generation: value.observed_generation,
            phase: value.phase,
            pod_ip: value.pod_ip,
            pod_ips: crate::OptionableConvert::try_from_optioned(value.pod_ips)?,
            qos_class: value.qos_class,
            reason: value.reason,
            resize: value.resize,
            resource_claim_statuses: crate::OptionableConvert::try_from_optioned(
                value.resource_claim_statuses,
            )?,
            start_time: crate::OptionableConvert::try_from_optioned(value.start_time)?,
        })
    }
    fn merge(&mut self, other: PodStatusAc) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = other.conditions;
        }
        if let Some(other_value) = other.conditions {
            crate::merge::try_merge_optioned_map(&mut self.conditions, other_value)?;
        }
        if self.container_statuses.is_none() {
            self.container_statuses = other.container_statuses;
        }
        if let Some(other_value) = other.container_statuses {
            self.container_statuses = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.ephemeral_container_statuses.is_none() {
            self.ephemeral_container_statuses = other.ephemeral_container_statuses;
        }
        if let Some(other_value) = other.ephemeral_container_statuses {
            self.ephemeral_container_statuses = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.host_ip.is_none() {
            self.host_ip = other.host_ip;
        }
        if let Some(other_value) = other.host_ip {
            crate::OptionableConvert::merge(&mut self.host_ip, other_value)?;
        }
        if self.host_ips.is_none() {
            self.host_ips = other.host_ips;
        }
        if let Some(other_value) = other.host_ips {
            self.host_ips = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.init_container_statuses.is_none() {
            self.init_container_statuses = other.init_container_statuses;
        }
        if let Some(other_value) = other.init_container_statuses {
            self.init_container_statuses = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.message.is_none() {
            self.message = other.message;
        }
        if let Some(other_value) = other.message {
            crate::OptionableConvert::merge(&mut self.message, other_value)?;
        }
        if self.nominated_node_name.is_none() {
            self.nominated_node_name = other.nominated_node_name;
        }
        if let Some(other_value) = other.nominated_node_name {
            crate::OptionableConvert::merge(&mut self.nominated_node_name, other_value)?;
        }
        if self.observed_generation.is_none() {
            self.observed_generation = other.observed_generation;
        }
        if let Some(other_value) = other.observed_generation {
            crate::OptionableConvert::merge(&mut self.observed_generation, other_value)?;
        }
        if self.phase.is_none() {
            self.phase = other.phase;
        }
        if let Some(other_value) = other.phase {
            crate::OptionableConvert::merge(&mut self.phase, other_value)?;
        }
        if self.pod_ip.is_none() {
            self.pod_ip = other.pod_ip;
        }
        if let Some(other_value) = other.pod_ip {
            crate::OptionableConvert::merge(&mut self.pod_ip, other_value)?;
        }
        if self.pod_ips.is_none() {
            self.pod_ips = other.pod_ips;
        }
        if let Some(other_value) = other.pod_ips {
            crate::merge::try_merge_optioned_map(&mut self.pod_ips, other_value)?;
        }
        if self.qos_class.is_none() {
            self.qos_class = other.qos_class;
        }
        if let Some(other_value) = other.qos_class {
            crate::OptionableConvert::merge(&mut self.qos_class, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = other.reason;
        }
        if let Some(other_value) = other.reason {
            crate::OptionableConvert::merge(&mut self.reason, other_value)?;
        }
        if self.resize.is_none() {
            self.resize = other.resize;
        }
        if let Some(other_value) = other.resize {
            crate::OptionableConvert::merge(&mut self.resize, other_value)?;
        }
        if self.resource_claim_statuses.is_none() {
            self.resource_claim_statuses = other.resource_claim_statuses;
        }
        if let Some(other_value) = other.resource_claim_statuses {
            crate::merge::try_merge_optioned_map(
                &mut self.resource_claim_statuses,
                other_value,
            )?;
        }
        if self.start_time.is_none() {
            self.start_time = other.start_time;
        }
        if let Some(other_value) = other.start_time {
            crate::OptionableConvert::merge(&mut self.start_time, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodStatus> for PodStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
