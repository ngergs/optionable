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
    /// The total number of device counter consumption entries must be \<= 32. In addition, the total number in the entire ResourceSlice must be \<= 1024 (for example, 64 devices with 16 counters each).
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
    /// The maximum number of taints is 4.
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
            attributes: crate::OptionableConvert::into_optioned(self.attributes),
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
            attributes: crate::OptionableConvert::try_from_optioned(value.attributes)?,
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
        self.all_nodes = other.all_nodes;
        crate::OptionableConvert::merge(&mut self.attributes, other.attributes)?;
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(
            &mut self.consumes_counters,
            other.consumes_counters,
        )?;
        self.node_name = other.node_name;
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        crate::OptionableConvert::merge(&mut self.taints, other.taints)?;
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
