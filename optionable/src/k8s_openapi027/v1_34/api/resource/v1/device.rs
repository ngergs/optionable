#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Device represents one individual hardware instance that can be selected based on its attributes. Besides the name, exactly one field must be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceAc {
    /// AllNodes indicates that all nodes have access to the device.
    ///
    /// Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,
    /// AllowMultipleAllocations marks whether the device is allowed to be allocated to multiple DeviceRequests.
    ///
    /// If AllowMultipleAllocations is set to true, the device can be allocated more than once, and all of its capacity is consumable, regardless of whether the requestPolicy is defined or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_allocations: Option<bool>,
    /// Attributes defines the set of attributes for this device. The name of each attribute must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::api::resource::v1::DeviceAttribute as crate::Optionable>::Optioned,
        >,
    >,
    /// BindingConditions defines the conditions for proceeding with binding. All of these conditions must be set in the per-device status conditions with a value of True to proceed with binding the pod to the node while scheduling the pod.
    ///
    /// The maximum number of binding conditions is 4.
    ///
    /// The conditions must be a valid condition type string.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_conditions: Option<std::vec::Vec<std::string::String>>,
    /// BindingFailureConditions defines the conditions for binding failure. They may be set in the per-device status conditions. If any is set to "True", a binding failure occurred.
    ///
    /// The maximum number of binding failure conditions is 4.
    ///
    /// The conditions must be a valid condition type string.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_failure_conditions: Option<std::vec::Vec<std::string::String>>,
    /// BindsToNode indicates if the usage of an allocation involving this device has to be limited to exactly the node that was chosen when allocating the claim. If set to true, the scheduler will set the ResourceClaim.Status.Allocation.NodeSelector to match the node where the allocation was made.
    ///
    /// This is an alpha field and requires enabling the DRADeviceBindingConditions and DRAResourceClaimDeviceStatus feature gates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binds_to_node: Option<bool>,
    /// Capacity defines the set of capacities for this device. The name of each capacity must be unique in that set.
    ///
    /// The maximum number of attributes and capacities combined is 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::api::resource::v1::DeviceCapacity as crate::Optionable>::Optioned,
        >,
    >,
    /// ConsumesCounters defines a list of references to sharedCounters and the set of counters that the device will consume from those counter sets.
    ///
    /// There can only be a single entry per counterSet.
    ///
    /// The total number of device counter consumption entries must be \<= 32. In addition, the total number in the entire ResourceSlice must be \<= 1024 (for example, 64 devices with 16 counters each).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumes_counters: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1::DeviceCounterConsumption as crate::Optionable>::Optioned,
        >,
    >,
    /// Name is unique identifier among all devices managed by the driver in the pool. It must be a DNS label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// NodeName identifies the node where the device is available.
    ///
    /// Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// NodeSelector defines the nodes where the device is available.
    ///
    /// Must use exactly one term.
    ///
    /// Must only be set if Spec.PerDeviceNodeSelection is set to true. At most one of NodeName, NodeSelector and AllNodes can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        <::k8s_openapi027::api::core::v1::NodeSelector as crate::Optionable>::Optioned,
    >,
    /// If specified, these are the driver-defined taints.
    ///
    /// The maximum number of taints is 4.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1::DeviceTaint as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1::Device {
    type Optioned = DeviceAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAc {
    type Optioned = DeviceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1::Device {
    fn into_optioned(self) -> DeviceAc {
        DeviceAc {
            all_nodes: self.all_nodes,
            allow_multiple_allocations: self.allow_multiple_allocations,
            attributes: crate::OptionableConvert::into_optioned(self.attributes),
            binding_conditions: self.binding_conditions,
            binding_failure_conditions: self.binding_failure_conditions,
            binds_to_node: self.binds_to_node,
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            consumes_counters: crate::OptionableConvert::into_optioned(
                self.consumes_counters,
            ),
            name: Some(self.name),
            node_name: self.node_name,
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            taints: crate::OptionableConvert::into_optioned(self.taints),
        }
    }
    fn try_from_optioned(value: DeviceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            all_nodes: value.all_nodes,
            allow_multiple_allocations: value.allow_multiple_allocations,
            attributes: crate::OptionableConvert::try_from_optioned(value.attributes)?,
            binding_conditions: value.binding_conditions,
            binding_failure_conditions: value.binding_failure_conditions,
            binds_to_node: value.binds_to_node,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            consumes_counters: crate::OptionableConvert::try_from_optioned(
                value.consumes_counters,
            )?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            node_name: value.node_name,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
            taints: crate::OptionableConvert::try_from_optioned(value.taints)?,
        })
    }
    fn merge(&mut self, other: DeviceAc) -> Result<(), crate::Error> {
        if other.all_nodes.is_some() {
            self.all_nodes = other.all_nodes;
        }
        if other.allow_multiple_allocations.is_some() {
            self.allow_multiple_allocations = other.allow_multiple_allocations;
        }
        crate::OptionableConvert::merge(&mut self.attributes, other.attributes)?;
        if other.binding_conditions.is_some() {
            self.binding_conditions = other.binding_conditions;
        }
        if other.binding_failure_conditions.is_some() {
            self.binding_failure_conditions = other.binding_failure_conditions;
        }
        if other.binds_to_node.is_some() {
            self.binds_to_node = other.binds_to_node;
        }
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(
            &mut self.consumes_counters,
            other.consumes_counters,
        )?;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if other.node_name.is_some() {
            self.node_name = other.node_name;
        }
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        crate::OptionableConvert::merge(&mut self.taints, other.taints)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1::Device> for DeviceAc {
    fn from_optionable(value: k8s_openapi027::api::resource::v1::Device) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1::Device, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::Device,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
