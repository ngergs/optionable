#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// BasicDevice defines one device instance.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BasicDeviceAc {
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
            <::k8s_openapi027::api::resource::v1beta1::DeviceAttribute as crate::Optionable>::Optioned,
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
    /// BindingFailureConditions defines the conditions for binding failure. They may be set in the per-device status conditions. If any is true, a binding failure occurred.
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
            <::k8s_openapi027::api::resource::v1beta1::DeviceCapacity as crate::Optionable>::Optioned,
        >,
    >,
    /// ConsumesCounters defines a list of references to sharedCounters and the set of counters that the device will consume from those counter sets.
    ///
    /// There can only be a single entry per counterSet.
    ///
    /// The maximum number of device counter consumptions per device is 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumes_counters: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta1::DeviceCounterConsumption as crate::Optionable>::Optioned,
        >,
    >,
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
    /// The maximum number of taints is 16. If taints are set for any device in a ResourceSlice, then the maximum number of allowed devices per ResourceSlice is 64 instead of 128.
    ///
    /// This is an alpha field and requires enabling the DRADeviceTaints feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta1::DeviceTaint as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta1::BasicDevice {
    type Optioned = BasicDeviceAc;
}
#[automatically_derived]
impl crate::Optionable for BasicDeviceAc {
    type Optioned = BasicDeviceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1beta1::BasicDevice {
    fn into_optioned(self) -> BasicDeviceAc {
        BasicDeviceAc {
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
            node_name: self.node_name,
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            taints: crate::OptionableConvert::into_optioned(self.taints),
        }
    }
    fn try_from_optioned(value: BasicDeviceAc) -> Result<Self, crate::Error> {
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
            node_name: value.node_name,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
            taints: crate::OptionableConvert::try_from_optioned(value.taints)?,
        })
    }
    fn merge(&mut self, other: BasicDeviceAc) -> Result<(), crate::Error> {
        if self.all_nodes.is_none() {
            self.all_nodes = crate::OptionableConvert::try_from_optioned(
                other.all_nodes,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.all_nodes, other.all_nodes)?;
        }
        if self.allow_multiple_allocations.is_none() {
            self.allow_multiple_allocations = crate::OptionableConvert::try_from_optioned(
                other.allow_multiple_allocations,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.allow_multiple_allocations,
                other.allow_multiple_allocations,
            )?;
        }
        if self.attributes.is_none() {
            self.attributes = crate::OptionableConvert::try_from_optioned(
                other.attributes,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.attributes, other.attributes)?;
        }
        if self.binding_conditions.is_none() {
            self.binding_conditions = crate::OptionableConvert::try_from_optioned(
                other.binding_conditions,
            )?;
        } else {
            self.binding_conditions = crate::OptionableConvert::try_from_optioned(
                other.binding_conditions,
            )?;
        }
        if self.binding_failure_conditions.is_none() {
            self.binding_failure_conditions = crate::OptionableConvert::try_from_optioned(
                other.binding_failure_conditions,
            )?;
        } else {
            self.binding_failure_conditions = crate::OptionableConvert::try_from_optioned(
                other.binding_failure_conditions,
            )?;
        }
        if self.binds_to_node.is_none() {
            self.binds_to_node = crate::OptionableConvert::try_from_optioned(
                other.binds_to_node,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.binds_to_node,
                other.binds_to_node,
            )?;
        }
        if self.capacity.is_none() {
            self.capacity = crate::OptionableConvert::try_from_optioned(other.capacity)?;
        } else {
            crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        }
        if self.consumes_counters.is_none() {
            self.consumes_counters = crate::OptionableConvert::try_from_optioned(
                other.consumes_counters,
            )?;
        } else {
            self.consumes_counters = crate::OptionableConvert::try_from_optioned(
                other.consumes_counters,
            )?;
        }
        if self.node_name.is_none() {
            self.node_name = crate::OptionableConvert::try_from_optioned(
                other.node_name,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.node_name, other.node_name)?;
        }
        if self.node_selector.is_none() {
            self.node_selector = crate::OptionableConvert::try_from_optioned(
                other.node_selector,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.node_selector,
                other.node_selector,
            )?;
        }
        if self.taints.is_none() {
            self.taints = crate::OptionableConvert::try_from_optioned(other.taints)?;
        } else {
            self.taints = crate::OptionableConvert::try_from_optioned(other.taints)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta1::BasicDevice>
for BasicDeviceAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::BasicDevice,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta1::BasicDevice, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::BasicDevice,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
