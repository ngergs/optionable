pub struct NodeConfigSourceOpt {
    pub config_map: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::node_config_source::NodeConfigSource {
    type Optioned = NodeConfigSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeConfigSourceOpt {
    type Optioned = NodeConfigSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::node_config_source::NodeConfigSource {
    fn into_optioned(self) -> NodeConfigSourceOpt {
        NodeConfigSourceOpt {
            config_map: <Option<
                ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource,
            > as crate::OptionableConvert>::into_optioned(self.config_map),
        }
    }
    fn try_from_optioned(
        value: NodeConfigSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config_map: <Option<
                ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource,
            > as crate::OptionableConvert>::try_from_optioned(value.config_map)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeConfigSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource,
        > as crate::OptionableConvert>::merge(&mut self.config_map, other.config_map)?;
        Ok(())
    }
}
