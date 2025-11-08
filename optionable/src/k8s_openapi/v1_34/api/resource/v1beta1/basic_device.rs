#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct BasicDeviceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_nodes: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_allocations: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1beta1::DeviceAttribute,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_conditions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_failure_conditions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binds_to_node: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1beta1::DeviceCapacity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumes_counters: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceCounterConsumption>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceTaint>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::BasicDevice {
    type Optioned = BasicDeviceAc;
}
#[automatically_derived]
impl crate::Optionable for BasicDeviceAc {
    type Optioned = BasicDeviceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::BasicDevice {
    fn into_optioned(self) -> BasicDeviceAc {
        BasicDeviceAc {
            all_nodes: crate::OptionableConvert::into_optioned(self.all_nodes),
            allow_multiple_allocations: crate::OptionableConvert::into_optioned(
                self.allow_multiple_allocations,
            ),
            attributes: crate::OptionableConvert::into_optioned(self.attributes),
            binding_conditions: crate::OptionableConvert::into_optioned(
                self.binding_conditions,
            ),
            binding_failure_conditions: crate::OptionableConvert::into_optioned(
                self.binding_failure_conditions,
            ),
            binds_to_node: crate::OptionableConvert::into_optioned(self.binds_to_node),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            consumes_counters: crate::OptionableConvert::into_optioned(
                self.consumes_counters,
            ),
            node_name: crate::OptionableConvert::into_optioned(self.node_name),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            taints: crate::OptionableConvert::into_optioned(self.taints),
        }
    }
    fn try_from_optioned(value: BasicDeviceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            all_nodes: crate::OptionableConvert::try_from_optioned(value.all_nodes)?,
            allow_multiple_allocations: crate::OptionableConvert::try_from_optioned(
                value.allow_multiple_allocations,
            )?,
            attributes: crate::OptionableConvert::try_from_optioned(value.attributes)?,
            binding_conditions: crate::OptionableConvert::try_from_optioned(
                value.binding_conditions,
            )?,
            binding_failure_conditions: crate::OptionableConvert::try_from_optioned(
                value.binding_failure_conditions,
            )?,
            binds_to_node: crate::OptionableConvert::try_from_optioned(
                value.binds_to_node,
            )?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            consumes_counters: crate::OptionableConvert::try_from_optioned(
                value.consumes_counters,
            )?,
            node_name: crate::OptionableConvert::try_from_optioned(value.node_name)?,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
            taints: crate::OptionableConvert::try_from_optioned(value.taints)?,
        })
    }
    fn merge(&mut self, other: BasicDeviceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.all_nodes, other.all_nodes)?;
        crate::OptionableConvert::merge(
            &mut self.allow_multiple_allocations,
            other.allow_multiple_allocations,
        )?;
        crate::OptionableConvert::merge(&mut self.attributes, other.attributes)?;
        crate::OptionableConvert::merge(
            &mut self.binding_conditions,
            other.binding_conditions,
        )?;
        crate::OptionableConvert::merge(
            &mut self.binding_failure_conditions,
            other.binding_failure_conditions,
        )?;
        crate::OptionableConvert::merge(&mut self.binds_to_node, other.binds_to_node)?;
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(
            &mut self.consumes_counters,
            other.consumes_counters,
        )?;
        crate::OptionableConvert::merge(&mut self.node_name, other.node_name)?;
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        crate::OptionableConvert::merge(&mut self.taints, other.taints)?;
        Ok(())
    }
}
