#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceSliceSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta1::Device as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        <::k8s_openapi027::api::core::v1::NodeSelector as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_device_node_selection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<
        <::k8s_openapi027::api::resource::v1beta1::ResourcePool as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_counters: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta1::CounterSet as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta1::ResourceSliceSpec {
    type Optioned = ResourceSliceSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceSliceSpecAc {
    type Optioned = ResourceSliceSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::ResourceSliceSpec {
    fn into_optioned(self) -> ResourceSliceSpecAc {
        ResourceSliceSpecAc {
            all_nodes: self.all_nodes,
            devices: crate::OptionableConvert::into_optioned(self.devices),
            driver: Some(self.driver),
            node_name: self.node_name,
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            per_device_node_selection: self.per_device_node_selection,
            pool: Some(crate::OptionableConvert::into_optioned(self.pool)),
            shared_counters: crate::OptionableConvert::into_optioned(
                self.shared_counters,
            ),
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
        })
    }
    fn merge(&mut self, other: ResourceSliceSpecAc) -> Result<(), crate::Error> {
        self.all_nodes = other.all_nodes;
        crate::OptionableConvert::merge(&mut self.devices, other.devices)?;
        if let Some(other_value) = other.driver {
            self.driver = other_value;
        }
        self.node_name = other.node_name;
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        self.per_device_node_selection = other.per_device_node_selection;
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta1::ResourceSliceSpec>
for ResourceSliceSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::ResourceSliceSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1beta1::ResourceSliceSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::ResourceSliceSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
