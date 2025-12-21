#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::NodeAddress>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocatable: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi026::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi026::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::NodeCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: <Option<
        ::k8s_openapi026::api::core::v1::NodeConfigStatus,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_endpoints: <Option<
        ::k8s_openapi026::api::core::v1::NodeDaemonEndpoints,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::ContainerImage>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info: <Option<
        ::k8s_openapi026::api::core::v1::NodeSystemInfo,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_handlers: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::NodeRuntimeHandler>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_attached: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::AttachedVolume>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_in_use: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::NodeStatus {
    type Optioned = NodeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NodeStatusAc {
    type Optioned = NodeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::NodeStatus {
    fn into_optioned(self) -> NodeStatusAc {
        NodeStatusAc {
            addresses: crate::OptionableConvert::into_optioned(self.addresses),
            allocatable: crate::OptionableConvert::into_optioned(self.allocatable),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            config: crate::OptionableConvert::into_optioned(self.config),
            daemon_endpoints: crate::OptionableConvert::into_optioned(
                self.daemon_endpoints,
            ),
            images: crate::OptionableConvert::into_optioned(self.images),
            node_info: crate::OptionableConvert::into_optioned(self.node_info),
            phase: crate::OptionableConvert::into_optioned(self.phase),
            runtime_handlers: crate::OptionableConvert::into_optioned(
                self.runtime_handlers,
            ),
            volumes_attached: crate::OptionableConvert::into_optioned(
                self.volumes_attached,
            ),
            volumes_in_use: crate::OptionableConvert::into_optioned(self.volumes_in_use),
        }
    }
    fn try_from_optioned(value: NodeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            addresses: crate::OptionableConvert::try_from_optioned(value.addresses)?,
            allocatable: crate::OptionableConvert::try_from_optioned(value.allocatable)?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            daemon_endpoints: crate::OptionableConvert::try_from_optioned(
                value.daemon_endpoints,
            )?,
            images: crate::OptionableConvert::try_from_optioned(value.images)?,
            node_info: crate::OptionableConvert::try_from_optioned(value.node_info)?,
            phase: crate::OptionableConvert::try_from_optioned(value.phase)?,
            runtime_handlers: crate::OptionableConvert::try_from_optioned(
                value.runtime_handlers,
            )?,
            volumes_attached: crate::OptionableConvert::try_from_optioned(
                value.volumes_attached,
            )?,
            volumes_in_use: crate::OptionableConvert::try_from_optioned(
                value.volumes_in_use,
            )?,
        })
    }
    fn merge(&mut self, other: NodeStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.addresses, other.addresses)?;
        crate::OptionableConvert::merge(&mut self.allocatable, other.allocatable)?;
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(
            &mut self.daemon_endpoints,
            other.daemon_endpoints,
        )?;
        crate::OptionableConvert::merge(&mut self.images, other.images)?;
        crate::OptionableConvert::merge(&mut self.node_info, other.node_info)?;
        crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        crate::OptionableConvert::merge(
            &mut self.runtime_handlers,
            other.runtime_handlers,
        )?;
        crate::OptionableConvert::merge(
            &mut self.volumes_attached,
            other.volumes_attached,
        )?;
        crate::OptionableConvert::merge(&mut self.volumes_in_use, other.volumes_in_use)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::NodeStatus> for NodeStatusAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::NodeStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::NodeStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::NodeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
