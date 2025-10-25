pub struct NodeStatusOpt {
    pub addresses: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::NodeAddress>,
    > as crate::Optionable>::Optioned,
    pub allocatable: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::NodeCondition>,
    > as crate::Optionable>::Optioned,
    pub config: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigStatus,
    > as crate::Optionable>::Optioned,
    pub daemon_endpoints: <Option<
        ::k8s_openapi::api::core::v1::NodeDaemonEndpoints,
    > as crate::Optionable>::Optioned,
    pub features: <Option<
        ::k8s_openapi::api::core::v1::NodeFeatures,
    > as crate::Optionable>::Optioned,
    pub images: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ContainerImage>,
    > as crate::Optionable>::Optioned,
    pub node_info: <Option<
        ::k8s_openapi::api::core::v1::NodeSystemInfo,
    > as crate::Optionable>::Optioned,
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub runtime_handlers: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::NodeRuntimeHandler>,
    > as crate::Optionable>::Optioned,
    pub volumes_attached: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::AttachedVolume>,
    > as crate::Optionable>::Optioned,
    pub volumes_in_use: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeStatus {
    type Optioned = NodeStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeStatusOpt {
    type Optioned = NodeStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeStatus {
    fn into_optioned(self) -> NodeStatusOpt {
        NodeStatusOpt {
            addresses: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::NodeAddress>,
            > as crate::OptionableConvert>::into_optioned(self.addresses),
            allocatable: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.allocatable),
            capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.capacity),
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::NodeCondition>,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            config: <Option<
                ::k8s_openapi::api::core::v1::NodeConfigStatus,
            > as crate::OptionableConvert>::into_optioned(self.config),
            daemon_endpoints: <Option<
                ::k8s_openapi::api::core::v1::NodeDaemonEndpoints,
            > as crate::OptionableConvert>::into_optioned(self.daemon_endpoints),
            features: <Option<
                ::k8s_openapi::api::core::v1::NodeFeatures,
            > as crate::OptionableConvert>::into_optioned(self.features),
            images: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ContainerImage>,
            > as crate::OptionableConvert>::into_optioned(self.images),
            node_info: <Option<
                ::k8s_openapi::api::core::v1::NodeSystemInfo,
            > as crate::OptionableConvert>::into_optioned(self.node_info),
            phase: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.phase),
            runtime_handlers: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::NodeRuntimeHandler>,
            > as crate::OptionableConvert>::into_optioned(self.runtime_handlers),
            volumes_attached: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::AttachedVolume>,
            > as crate::OptionableConvert>::into_optioned(self.volumes_attached),
            volumes_in_use: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.volumes_in_use),
        }
    }
    fn try_from_optioned(
        value: NodeStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            addresses: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::NodeAddress>,
            > as crate::OptionableConvert>::try_from_optioned(value.addresses)?,
            allocatable: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.allocatable)?,
            capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.capacity)?,
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::NodeCondition>,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            config: <Option<
                ::k8s_openapi::api::core::v1::NodeConfigStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.config)?,
            daemon_endpoints: <Option<
                ::k8s_openapi::api::core::v1::NodeDaemonEndpoints,
            > as crate::OptionableConvert>::try_from_optioned(value.daemon_endpoints)?,
            features: <Option<
                ::k8s_openapi::api::core::v1::NodeFeatures,
            > as crate::OptionableConvert>::try_from_optioned(value.features)?,
            images: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::ContainerImage>,
            > as crate::OptionableConvert>::try_from_optioned(value.images)?,
            node_info: <Option<
                ::k8s_openapi::api::core::v1::NodeSystemInfo,
            > as crate::OptionableConvert>::try_from_optioned(value.node_info)?,
            phase: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.phase)?,
            runtime_handlers: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::NodeRuntimeHandler>,
            > as crate::OptionableConvert>::try_from_optioned(value.runtime_handlers)?,
            volumes_attached: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::AttachedVolume>,
            > as crate::OptionableConvert>::try_from_optioned(value.volumes_attached)?,
            volumes_in_use: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.volumes_in_use)?,
        })
    }
    fn merge(&mut self, other: NodeStatusOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::NodeAddress>,
        > as crate::OptionableConvert>::merge(&mut self.addresses, other.addresses)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.allocatable, other.allocatable)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.capacity, other.capacity)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::NodeCondition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            ::k8s_openapi::api::core::v1::NodeConfigStatus,
        > as crate::OptionableConvert>::merge(&mut self.config, other.config)?;
        <Option<
            ::k8s_openapi::api::core::v1::NodeDaemonEndpoints,
        > as crate::OptionableConvert>::merge(
            &mut self.daemon_endpoints,
            other.daemon_endpoints,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::NodeFeatures,
        > as crate::OptionableConvert>::merge(&mut self.features, other.features)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::ContainerImage>,
        > as crate::OptionableConvert>::merge(&mut self.images, other.images)?;
        <Option<
            ::k8s_openapi::api::core::v1::NodeSystemInfo,
        > as crate::OptionableConvert>::merge(&mut self.node_info, other.node_info)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.phase, other.phase)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::NodeRuntimeHandler>,
        > as crate::OptionableConvert>::merge(
            &mut self.runtime_handlers,
            other.runtime_handlers,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::AttachedVolume>,
        > as crate::OptionableConvert>::merge(
            &mut self.volumes_attached,
            other.volumes_attached,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.volumes_in_use,
            other.volumes_in_use,
        )?;
        Ok(())
    }
}
