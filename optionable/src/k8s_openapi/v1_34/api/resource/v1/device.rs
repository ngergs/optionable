pub struct DeviceOpt {
    pub all_nodes: <Option<bool> as crate::Optionable>::Optioned,
    pub allow_multiple_allocations: <Option<bool> as crate::Optionable>::Optioned,
    pub attributes: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1::DeviceAttribute,
        >,
    > as crate::Optionable>::Optioned,
    pub binding_conditions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub binding_failure_conditions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub binds_to_node: <Option<bool> as crate::Optionable>::Optioned,
    pub capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1::DeviceCapacity,
        >,
    > as crate::Optionable>::Optioned,
    pub consumes_counters: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceCounterConsumption>,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_selector: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
    pub taints: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceTaint>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::device::Device {
    type Optioned = DeviceOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceOpt {
    type Optioned = DeviceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::device::Device {
    fn into_optioned(self) -> DeviceOpt {
        DeviceOpt {
            all_nodes: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.all_nodes),
            allow_multiple_allocations: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.allow_multiple_allocations,
            ),
            attributes: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::api::resource::v1::DeviceAttribute,
                >,
            > as crate::OptionableConvert>::into_optioned(self.attributes),
            binding_conditions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.binding_conditions),
            binding_failure_conditions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(
                self.binding_failure_conditions,
            ),
            binds_to_node: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.binds_to_node),
            capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::api::resource::v1::DeviceCapacity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.capacity),
            consumes_counters: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceCounterConsumption>,
            > as crate::OptionableConvert>::into_optioned(self.consumes_counters),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.node_name),
            node_selector: <Option<
                ::k8s_openapi::api::core::v1::NodeSelector,
            > as crate::OptionableConvert>::into_optioned(self.node_selector),
            taints: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceTaint>,
            > as crate::OptionableConvert>::into_optioned(self.taints),
        }
    }
    fn try_from_optioned(value: DeviceOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            all_nodes: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.all_nodes)?,
            allow_multiple_allocations: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.allow_multiple_allocations,
            )?,
            attributes: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::api::resource::v1::DeviceAttribute,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.attributes)?,
            binding_conditions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.binding_conditions)?,
            binding_failure_conditions: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.binding_failure_conditions,
            )?,
            binds_to_node: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.binds_to_node)?,
            capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::api::resource::v1::DeviceCapacity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.capacity)?,
            consumes_counters: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceCounterConsumption>,
            > as crate::OptionableConvert>::try_from_optioned(value.consumes_counters)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.node_name)?,
            node_selector: <Option<
                ::k8s_openapi::api::core::v1::NodeSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.node_selector)?,
            taints: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceTaint>,
            > as crate::OptionableConvert>::try_from_optioned(value.taints)?,
        })
    }
    fn merge(&mut self, other: DeviceOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.all_nodes, other.all_nodes)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.allow_multiple_allocations,
            other.allow_multiple_allocations,
        )?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::api::resource::v1::DeviceAttribute,
            >,
        > as crate::OptionableConvert>::merge(&mut self.attributes, other.attributes)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.binding_conditions,
            other.binding_conditions,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.binding_failure_conditions,
            other.binding_failure_conditions,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.binds_to_node,
            other.binds_to_node,
        )?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::api::resource::v1::DeviceCapacity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.capacity, other.capacity)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceCounterConsumption>,
        > as crate::OptionableConvert>::merge(
            &mut self.consumes_counters,
            other.consumes_counters,
        )?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.node_name, other.node_name)?;
        <Option<
            ::k8s_openapi::api::core::v1::NodeSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.node_selector,
            other.node_selector,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::DeviceTaint>,
        > as crate::OptionableConvert>::merge(&mut self.taints, other.taints)?;
        Ok(())
    }
}
