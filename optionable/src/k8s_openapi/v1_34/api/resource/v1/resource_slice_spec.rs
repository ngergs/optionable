pub struct ResourceSliceSpecOpt {
    pub all_nodes: <Option<bool> as crate::Optionable>::Optioned,
    pub devices: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::Device>,
    > as crate::Optionable>::Optioned,
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_selector: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
    pub per_device_node_selection: <Option<bool> as crate::Optionable>::Optioned,
    pub pool: Option<
        <::k8s_openapi::api::resource::v1::ResourcePool as crate::Optionable>::Optioned,
    >,
    pub shared_counters: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1::CounterSet>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1::resource_slice_spec::ResourceSliceSpec {
    type Optioned = ResourceSliceSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceSliceSpecOpt {
    type Optioned = ResourceSliceSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::resource_slice_spec::ResourceSliceSpec {
    fn into_optioned(self) -> ResourceSliceSpecOpt {
        ResourceSliceSpecOpt {
            all_nodes: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.all_nodes),
            devices: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::Device>,
            > as crate::OptionableConvert>::into_optioned(self.devices),
            driver: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.driver,
                ),
            ),
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.node_name),
            node_selector: <Option<
                ::k8s_openapi::api::core::v1::NodeSelector,
            > as crate::OptionableConvert>::into_optioned(self.node_selector),
            per_device_node_selection: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.per_device_node_selection,
            ),
            pool: Some(
                <::k8s_openapi::api::resource::v1::ResourcePool as crate::OptionableConvert>::into_optioned(
                    self.pool,
                ),
            ),
            shared_counters: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::CounterSet>,
            > as crate::OptionableConvert>::into_optioned(self.shared_counters),
        }
    }
    fn try_from_optioned(
        value: ResourceSliceSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            all_nodes: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.all_nodes)?,
            devices: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::Device>,
            > as crate::OptionableConvert>::try_from_optioned(value.devices)?,
            driver: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            node_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.node_name)?,
            node_selector: <Option<
                ::k8s_openapi::api::core::v1::NodeSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.node_selector)?,
            per_device_node_selection: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.per_device_node_selection,
            )?,
            pool: <::k8s_openapi::api::resource::v1::ResourcePool as crate::OptionableConvert>::try_from_optioned(
                value
                    .pool
                    .ok_or(crate::optionable::Error {
                        missing_field: "pool",
                    })?,
            )?,
            shared_counters: <Option<
                std::vec::Vec<::k8s_openapi::api::resource::v1::CounterSet>,
            > as crate::OptionableConvert>::try_from_optioned(value.shared_counters)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceSliceSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.all_nodes, other.all_nodes)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::Device>,
        > as crate::OptionableConvert>::merge(&mut self.devices, other.devices)?;
        if let Some(other_value) = other.driver {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.driver,
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
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.per_device_node_selection,
            other.per_device_node_selection,
        )?;
        if let Some(other_value) = other.pool {
            <::k8s_openapi::api::resource::v1::ResourcePool as crate::OptionableConvert>::merge(
                &mut self.pool,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::resource::v1::CounterSet>,
        > as crate::OptionableConvert>::merge(
            &mut self.shared_counters,
            other.shared_counters,
        )?;
        Ok(())
    }
}
