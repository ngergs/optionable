pub struct NodeConfigStatusOpt {
    pub active: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigSource,
    > as crate::Optionable>::Optioned,
    pub assigned: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigSource,
    > as crate::Optionable>::Optioned,
    pub error: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub last_known_good: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::node_config_status::NodeConfigStatus {
    type Optioned = NodeConfigStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeConfigStatusOpt {
    type Optioned = NodeConfigStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::node_config_status::NodeConfigStatus {
    fn into_optioned(self) -> NodeConfigStatusOpt {
        NodeConfigStatusOpt {
            active: <Option<
                ::k8s_openapi::api::core::v1::NodeConfigSource,
            > as crate::OptionableConvert>::into_optioned(self.active),
            assigned: <Option<
                ::k8s_openapi::api::core::v1::NodeConfigSource,
            > as crate::OptionableConvert>::into_optioned(self.assigned),
            error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.error),
            last_known_good: <Option<
                ::k8s_openapi::api::core::v1::NodeConfigSource,
            > as crate::OptionableConvert>::into_optioned(self.last_known_good),
        }
    }
    fn try_from_optioned(
        value: NodeConfigStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            active: <Option<
                ::k8s_openapi::api::core::v1::NodeConfigSource,
            > as crate::OptionableConvert>::try_from_optioned(value.active)?,
            assigned: <Option<
                ::k8s_openapi::api::core::v1::NodeConfigSource,
            > as crate::OptionableConvert>::try_from_optioned(value.assigned)?,
            error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.error)?,
            last_known_good: <Option<
                ::k8s_openapi::api::core::v1::NodeConfigSource,
            > as crate::OptionableConvert>::try_from_optioned(value.last_known_good)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeConfigStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::NodeConfigSource,
        > as crate::OptionableConvert>::merge(&mut self.active, other.active)?;
        <Option<
            ::k8s_openapi::api::core::v1::NodeConfigSource,
        > as crate::OptionableConvert>::merge(&mut self.assigned, other.assigned)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.error, other.error)?;
        <Option<
            ::k8s_openapi::api::core::v1::NodeConfigSource,
        > as crate::OptionableConvert>::merge(
            &mut self.last_known_good,
            other.last_known_good,
        )?;
        Ok(())
    }
}
