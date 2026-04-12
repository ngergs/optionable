#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeStatus is information about the current status of a node.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeStatusAc {
    /// List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses Note: This field is declared as mergeable, but the merge key is not sufficiently unique, which can cause data corruption when it is merged. Callers should instead use a full-replacement patch. See https://pr.k8s.io/79391 for an example. Consumers should assume that addresses can change during the lifetime of a Node. However, there are some exceptions where this may not be possible, such as Pods that inherit a Node's address in its own status or consumers of the downward API (status.hostIP).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::NodeAddress as crate::Optionable>::Optioned,
        >,
    >,
    /// Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocatable: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/reference/node/node-status/#capacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::NodeCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// Status of the config assigned to the node via the dynamic Kubelet config feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<
        <::k8s_openapi027::api::core::v1::NodeConfigStatus as crate::Optionable>::Optioned,
    >,
    /// Endpoints of daemons running on the Node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_endpoints: Option<
        <::k8s_openapi027::api::core::v1::NodeDaemonEndpoints as crate::Optionable>::Optioned,
    >,
    /// Features describes the set of features implemented by the CRI implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<
        <::k8s_openapi027::api::core::v1::NodeFeatures as crate::Optionable>::Optioned,
    >,
    /// List of container images on this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::ContainerImage as crate::Optionable>::Optioned,
        >,
    >,
    /// Set of ids/uuids to uniquely identify the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info: Option<
        <::k8s_openapi027::api::core::v1::NodeSystemInfo as crate::Optionable>::Optioned,
    >,
    /// NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<std::string::String>,
    /// The available runtime handlers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_handlers: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::NodeRuntimeHandler as crate::Optionable>::Optioned,
        >,
    >,
    /// List of volumes that are attached to the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_attached: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::AttachedVolume as crate::Optionable>::Optioned,
        >,
    >,
    /// List of attachable volumes in use (mounted) by the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_in_use: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeStatus {
    type Optioned = NodeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NodeStatusAc {
    type Optioned = NodeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeStatus {
    fn into_optioned(self) -> NodeStatusAc {
        NodeStatusAc {
            addresses: crate::OptionableConvert::into_optioned(self.addresses),
            allocatable: crate::OptionableConvert::into_optioned(self.allocatable),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            config: crate::OptionableConvert::into_optioned(self.config),
            daemon_endpoints: crate::OptionableConvert::into_optioned(
                self.daemon_endpoints,
            ),
            features: crate::OptionableConvert::into_optioned(self.features),
            images: crate::OptionableConvert::into_optioned(self.images),
            node_info: crate::OptionableConvert::into_optioned(self.node_info),
            phase: self.phase,
            runtime_handlers: crate::OptionableConvert::into_optioned(
                self.runtime_handlers,
            ),
            volumes_attached: crate::OptionableConvert::into_optioned(
                self.volumes_attached,
            ),
            volumes_in_use: self.volumes_in_use,
        }
    }
    fn try_from_optioned(value: NodeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            addresses: crate::OptionableConvert::try_from_optioned(value.addresses)?,
            allocatable: crate::OptionableConvert::try_from_optioned(value.allocatable)?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            daemon_endpoints: crate::OptionableConvert::try_from_optioned(
                value.daemon_endpoints,
            )?,
            features: crate::OptionableConvert::try_from_optioned(value.features)?,
            images: crate::OptionableConvert::try_from_optioned(value.images)?,
            node_info: crate::OptionableConvert::try_from_optioned(value.node_info)?,
            phase: value.phase,
            runtime_handlers: crate::OptionableConvert::try_from_optioned(
                value.runtime_handlers,
            )?,
            volumes_attached: crate::OptionableConvert::try_from_optioned(
                value.volumes_attached,
            )?,
            volumes_in_use: value.volumes_in_use,
        })
    }
    fn merge(&mut self, other: NodeStatusAc) -> Result<(), crate::Error> {
        if self.addresses.is_none() {
            self.addresses = crate::OptionableConvert::try_from_optioned(
                other.addresses,
            )?;
        } else {
            crate::merge::try_merge_optioned_map(&mut self.addresses, other.addresses)?;
        }
        if self.allocatable.is_none() {
            self.allocatable = crate::OptionableConvert::try_from_optioned(
                other.allocatable,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.allocatable, other.allocatable)?;
        }
        if self.capacity.is_none() {
            self.capacity = crate::OptionableConvert::try_from_optioned(other.capacity)?;
        } else {
            crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        }
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else {
            crate::merge::try_merge_optioned_map(
                &mut self.conditions,
                other.conditions,
            )?;
        }
        if self.config.is_none() {
            self.config = crate::OptionableConvert::try_from_optioned(other.config)?;
        } else {
            crate::OptionableConvert::merge(&mut self.config, other.config)?;
        }
        if self.daemon_endpoints.is_none() {
            self.daemon_endpoints = crate::OptionableConvert::try_from_optioned(
                other.daemon_endpoints,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.daemon_endpoints,
                other.daemon_endpoints,
            )?;
        }
        if self.features.is_none() {
            self.features = crate::OptionableConvert::try_from_optioned(other.features)?;
        } else {
            crate::OptionableConvert::merge(&mut self.features, other.features)?;
        }
        if self.images.is_none() {
            self.images = crate::OptionableConvert::try_from_optioned(other.images)?;
        } else {
            self.images = crate::OptionableConvert::try_from_optioned(other.images)?;
        }
        if self.node_info.is_none() {
            self.node_info = crate::OptionableConvert::try_from_optioned(
                other.node_info,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.node_info, other.node_info)?;
        }
        if self.phase.is_none() {
            self.phase = crate::OptionableConvert::try_from_optioned(other.phase)?;
        } else {
            crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        }
        if self.runtime_handlers.is_none() {
            self.runtime_handlers = crate::OptionableConvert::try_from_optioned(
                other.runtime_handlers,
            )?;
        } else {
            self.runtime_handlers = crate::OptionableConvert::try_from_optioned(
                other.runtime_handlers,
            )?;
        }
        if self.volumes_attached.is_none() {
            self.volumes_attached = crate::OptionableConvert::try_from_optioned(
                other.volumes_attached,
            )?;
        } else {
            self.volumes_attached = crate::OptionableConvert::try_from_optioned(
                other.volumes_attached,
            )?;
        }
        if self.volumes_in_use.is_none() {
            self.volumes_in_use = crate::OptionableConvert::try_from_optioned(
                other.volumes_in_use,
            )?;
        } else {
            self.volumes_in_use = crate::OptionableConvert::try_from_optioned(
                other.volumes_in_use,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeStatus> for NodeStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
