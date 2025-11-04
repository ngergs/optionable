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
    pub attributes: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::api::resource::v1beta1::DeviceAttribute,
        >,
    > as crate::Optionable>::Optioned,
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
            attributes: crate::OptionableConvert::into_optioned(self.attributes),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            consumes_counters: crate::OptionableConvert::into_optioned(
                self.consumes_counters,
            ),
            node_name: crate::OptionableConvert::into_optioned(self.node_name),
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            taints: crate::OptionableConvert::into_optioned(self.taints),
        }
    }
    fn try_from_optioned(
        value: BasicDeviceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            all_nodes: crate::OptionableConvert::try_from_optioned(value.all_nodes)?,
            attributes: crate::OptionableConvert::try_from_optioned(value.attributes)?,
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
    fn merge(&mut self, other: BasicDeviceAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.all_nodes, other.all_nodes)?;
        crate::OptionableConvert::merge(&mut self.attributes, other.attributes)?;
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
