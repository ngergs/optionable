#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfigSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeConfigSource {
    type Optioned = NodeConfigSourceAc;
}
#[automatically_derived]
impl crate::Optionable for NodeConfigSourceAc {
    type Optioned = NodeConfigSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeConfigSource {
    fn into_optioned(self) -> NodeConfigSourceAc {
        NodeConfigSourceAc {
            config_map: crate::OptionableConvert::into_optioned(self.config_map),
        }
    }
    fn try_from_optioned(value: NodeConfigSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config_map: crate::OptionableConvert::try_from_optioned(value.config_map)?,
        })
    }
    fn merge(&mut self, other: NodeConfigSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.config_map, other.config_map)?;
        Ok(())
    }
}
