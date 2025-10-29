pub struct ResourceSliceSpecOpt {
    pub all_nodes: <Option<bool> as crate::Optionable>::Optioned,
    pub devices: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta2::Device>,
    > as crate::Optionable>::Optioned,
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub node_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_selector: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
    pub per_device_node_selection: <Option<bool> as crate::Optionable>::Optioned,
    pub pool: Option<
        <::k8s_openapi::api::resource::v1beta2::ResourcePool as crate::Optionable>::Optioned,
    >,
    pub shared_counters: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta2::CounterSet>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::ResourceSliceSpec {
    type Optioned = ResourceSliceSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceSliceSpecOpt {
    type Optioned = ResourceSliceSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::ResourceSliceSpec {
    fn into_optioned(self) -> ResourceSliceSpecOpt {
        ResourceSliceSpecOpt {
            all_nodes: crate::OptionableConvert::into_optioned(self.all_nodes),
            devices: crate::OptionableConvert::into_optioned(self.devices),
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            node_name: crate::OptionableConvert::into_optioned(self.node_name),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            per_device_node_selection: crate::OptionableConvert::into_optioned(
                self.per_device_node_selection,
            ),
            pool: Some(crate::OptionableConvert::into_optioned(self.pool)),
            shared_counters: crate::OptionableConvert::into_optioned(
                self.shared_counters,
            ),
        }
    }
    fn try_from_optioned(
        value: ResourceSliceSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            all_nodes: crate::OptionableConvert::try_from_optioned(value.all_nodes)?,
            devices: crate::OptionableConvert::try_from_optioned(value.devices)?,
            driver: crate::OptionableConvert::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            node_name: crate::OptionableConvert::try_from_optioned(value.node_name)?,
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
            per_device_node_selection: crate::OptionableConvert::try_from_optioned(
                value.per_device_node_selection,
            )?,
            pool: crate::OptionableConvert::try_from_optioned(
                value
                    .pool
                    .ok_or(crate::optionable::Error {
                        missing_field: "pool",
                    })?,
            )?,
            shared_counters: crate::OptionableConvert::try_from_optioned(
                value.shared_counters,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceSliceSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.all_nodes, other.all_nodes)?;
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        if let Some(other_value) = other.driver {
            crate::OptionableConvert::merge(&mut self.driver, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.node_name, other.node_name)?;
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        crate::OptionableConvert::merge(
            &mut self.per_device_node_selection,
            other.per_device_node_selection,
        )?;
        if let Some(other_value) = other.pool {
            crate::OptionableConvert::merge(&mut self.pool, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.shared_counters,
            other.shared_counters,
        )?;
        Ok(())
    }
}
