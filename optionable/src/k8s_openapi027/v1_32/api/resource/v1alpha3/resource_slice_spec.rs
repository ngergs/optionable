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
    /// Exactly one of NodeName, NodeSelector and AllNodes must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<bool>,
    /// Devices lists some or all of the devices in this pool.
    ///
    /// Must not have more than 128 entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::Device as crate::Optionable>::Optioned,
        >,
    >,
    /// Driver identifies the DRA driver providing the capacity information. A field selector can be used to list only ResourceSlice objects with a certain driver name.
    ///
    /// Must be a DNS subdomain and should end with a DNS domain owned by the vendor of the driver. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// NodeName identifies the node which provides the resources in this pool. A field selector can be used to list only ResourceSlice objects belonging to a certain node.
    ///
    /// This field can be used to limit access from nodes to ResourceSlices with the same node name. It also indicates to autoscalers that adding new nodes of the same type as some old node might also make new resources available.
    ///
    /// Exactly one of NodeName, NodeSelector and AllNodes must be set. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<std::string::String>,
    /// NodeSelector defines which nodes have access to the resources in the pool, when that pool is not limited to a single node.
    ///
    /// Must use exactly one term.
    ///
    /// Exactly one of NodeName, NodeSelector and AllNodes must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector: Option<
        <::k8s_openapi027::api::core::v1::NodeSelector as crate::Optionable>::Optioned,
    >,
    /// Pool describes the pool that this ResourceSlice belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<
        <::k8s_openapi027::api::resource::v1alpha3::ResourcePool as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::ResourceSliceSpec {
    type Optioned = ResourceSliceSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceSliceSpecAc {
    type Optioned = ResourceSliceSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::ResourceSliceSpec {
    fn into_optioned(self) -> ResourceSliceSpecAc {
        ResourceSliceSpecAc {
            all_nodes: self.all_nodes,
            devices: crate::OptionableConvert::into_optioned(self.devices),
            driver: Some(self.driver),
            node_name: self.node_name,
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            pool: Some(crate::OptionableConvert::into_optioned(self.pool)),
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
            pool: crate::OptionableConvert::try_from_optioned(
                value
                    .pool
                    .ok_or(crate::Error {
                        missing_field: "pool",
                    })?,
            )?,
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
            crate::OptionableConvert::merge(self_value, other_value)?;
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
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.pool {
            self.pool = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::ResourceSliceSpec>
for ResourceSliceSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::ResourceSliceSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::ResourceSliceSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::ResourceSliceSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
