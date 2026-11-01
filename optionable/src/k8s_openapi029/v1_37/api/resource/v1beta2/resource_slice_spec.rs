#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceSliceSpec contains the information published by the driver in one ResourceSlice.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceSliceSpecAc {
    /// AllNodes indicates that all nodes have access to the resources in the pool.
    ///
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,
    /// Devices lists some or all of the devices in this pool.
    ///
    /// Must not have more than 128 entries. If any device uses taints or consumes counters the limit is 64.
    ///
    /// Only one of Devices and SharedCounters can be set in a ResourceSlice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::resource::v1beta2::Device as crate::Optionable>::Optioned,
        >,
    >,
    /// Driver identifies the DRA driver providing the capacity information. A field selector can be used to list only ResourceSlice objects with a certain driver name.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. It should use only lower case characters. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// NodeName identifies the node which provides the resources in this pool. A field selector can be used to list only ResourceSlice objects belonging to a certain node.
    ///
    /// This field can be used to limit access from nodes to ResourceSlices with the same node name. It also indicates to autoscalers that adding new nodes of the same type as some old node might also make new resources available.
    ///
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// NodeSelector defines which nodes have access to the resources in the pool, when that pool is not limited to a single node.
    ///
    /// Must use exactly one term.
    ///
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        <::k8s_openapi029::api::core::v1::NodeSelector as crate::Optionable>::Optioned,
    >,
    /// PartitionTypeAttribute names a string device attribute (by fully qualified name, e.g. "gpu.example.com/profile") whose value labels each device with its partition type, such as "Full" or "Half" for a MIG-style GPU.
    ///
    /// When set, every partitionable device in the slice must carry the attribute and devices sharing a value must share the same ConsumesCounters cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_type_attribute: Option<std::string::String>,
    /// PerDeviceNodeSelection defines whether the access from nodes to resources in the pool is set on the ResourceSlice level or on each device. If it is set to true, every device defined the ResourceSlice must specify this individually.
    ///
    /// Exactly one of NodeName, NodeSelector, AllNodes, and PerDeviceNodeSelection must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_device_node_selection: Option<bool>,
    /// Pool describes the pool that this ResourceSlice belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<
        <::k8s_openapi029::api::resource::v1beta2::ResourcePool as crate::Optionable>::Optioned,
    >,
    /// SharedCounters defines a list of counter sets, each of which has a name and a list of counters available.
    ///
    /// The names of the counter sets must be unique in the ResourcePool.
    ///
    /// Only one of Devices and SharedCounters can be set in a ResourceSlice.
    ///
    /// The maximum number of counter sets is 8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_counters: Option<
        std::vec::Vec<
            <::k8s_openapi029::api::resource::v1beta2::CounterSet as crate::Optionable>::Optioned,
        >,
    >,
    /// SkipNodeOperations lists node-local resource operations (gRPC calls) that will be skipped for the devices in this slice when determining whether operations are necessary on the node. If all allocated devices for a driver in a claim skip an operation, that gRPC call will be skipped. Valid values are:
    ///
    /// - "NodePrepareResources": NodePrepareResources gRPC calls are skipped. This
    ///   value cannot be specified unless "NodeUnprepareResources" is also listed
    ///   (or "*" is specified).
    /// - "NodeUnprepareResources": NodeUnprepareResources gRPC calls are skipped. - "*": All node-local resource operations are skipped.
    ///
    /// Other values may be added in the future. The kubelet must ignore unknown values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_node_operations: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::resource::v1beta2::ResourceSliceSpec {
    type Optioned = ResourceSliceSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceSliceSpecAc {
    type Optioned = ResourceSliceSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::resource::v1beta2::ResourceSliceSpec {
    fn into_optioned(self) -> ResourceSliceSpecAc {
        ResourceSliceSpecAc {
            all_nodes: self.all_nodes,
            devices: crate::OptionableConvert::into_optioned(self.devices),
            driver: Some(self.driver),
            node_name: self.node_name,
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            partition_type_attribute: self.partition_type_attribute,
            per_device_node_selection: self.per_device_node_selection,
            pool: Some(crate::OptionableConvert::into_optioned(self.pool)),
            shared_counters: crate::OptionableConvert::into_optioned(
                self.shared_counters,
            ),
            skip_node_operations: self.skip_node_operations,
        }
    }
    fn try_from_optioned(value: ResourceSliceSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            all_nodes: value.all_nodes,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            node_name: value.node_name,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
            partition_type_attribute: value.partition_type_attribute,
            per_device_node_selection: value.per_device_node_selection,
            pool: crate::OptionableConvert::try_from_optioned(
                value
                    .pool
                    .ok_or(crate::Error {
                        missing_field: "pool",
                    })?,
            )?,
            shared_counters: crate::OptionableConvert::try_from_optioned(
                value.shared_counters,
            )?,
            skip_node_operations: value.skip_node_operations,
        })
    }
    fn merge(&mut self, other: ResourceSliceSpecAc) -> Result<(), crate::Error> {
        if self.all_nodes.is_none() {
            self.all_nodes = crate::OptionableConvert::try_from_optioned(
                other.all_nodes,
            )?;
        } else if let Some(self_value) = self.all_nodes.as_mut()
            && let Some(other_value) = other.all_nodes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.devices.is_none() {
            self.devices = crate::OptionableConvert::try_from_optioned(other.devices)?;
        } else if let Some(self_value) = self.devices.as_mut()
            && let Some(other_value) = other.devices
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.driver {
            self.driver = crate::OptionableConvert::try_from_optioned(other_value)?;
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
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.partition_type_attribute.is_none() {
            self.partition_type_attribute = crate::OptionableConvert::try_from_optioned(
                other.partition_type_attribute,
            )?;
        } else if let Some(self_value) = self.partition_type_attribute.as_mut()
            && let Some(other_value) = other.partition_type_attribute
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.per_device_node_selection.is_none() {
            self.per_device_node_selection = crate::OptionableConvert::try_from_optioned(
                other.per_device_node_selection,
            )?;
        } else if let Some(self_value) = self.per_device_node_selection.as_mut()
            && let Some(other_value) = other.per_device_node_selection
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.pool {
            crate::OptionableConvert::merge(&mut self.pool, other_value)?;
        }
        if self.shared_counters.is_none() {
            self.shared_counters = crate::OptionableConvert::try_from_optioned(
                other.shared_counters,
            )?;
        } else if let Some(self_value) = self.shared_counters.as_mut()
            && let Some(other_value) = other.shared_counters
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.skip_node_operations.is_none() {
            self.skip_node_operations = crate::OptionableConvert::try_from_optioned(
                other.skip_node_operations,
            )?;
        } else if let Some(self_value) = self.skip_node_operations.as_mut()
            && let Some(other_value) = other.skip_node_operations
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::resource::v1beta2::ResourceSliceSpec>
for ResourceSliceSpecAc {
    fn from_optionable(
        value: k8s_openapi029::api::resource::v1beta2::ResourceSliceSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::resource::v1beta2::ResourceSliceSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::resource::v1beta2::ResourceSliceSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for ResourceSliceSpecAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.all_nodes, other.all_nodes);
        self.devices = other.devices;
        k8s_openapi029::DeepMerge::merge_from(&mut self.driver, other.driver);
        k8s_openapi029::DeepMerge::merge_from(&mut self.node_name, other.node_name);
        self.node_selector = other.node_selector;
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.partition_type_attribute,
            other.partition_type_attribute,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.per_device_node_selection,
            other.per_device_node_selection,
        );
        k8s_openapi029::DeepMerge::merge_from(&mut self.pool, other.pool);
        self.shared_counters = other.shared_counters;
        crate::merge::merge_append_not_present_option_wrapped(
            &mut self.skip_node_operations,
            other.skip_node_operations,
        );
    }
}
