#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NodeRuntimeHandlerAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: <Option<
        ::k8s_openapi::api::core::v1::NodeRuntimeHandlerFeatures,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeRuntimeHandler {
    type Optioned = NodeRuntimeHandlerAc;
}
#[automatically_derived]
impl crate::Optionable for NodeRuntimeHandlerAc {
    type Optioned = NodeRuntimeHandlerAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeRuntimeHandler {
    fn into_optioned(self) -> NodeRuntimeHandlerAc {
        NodeRuntimeHandlerAc {
            features: crate::OptionableConvert::into_optioned(self.features),
            name: crate::OptionableConvert::into_optioned(self.name),
        }
    }
    fn try_from_optioned(value: NodeRuntimeHandlerAc) -> Result<Self, crate::Error> {
        Ok(Self {
            features: crate::OptionableConvert::try_from_optioned(value.features)?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
        })
    }
    fn merge(&mut self, other: NodeRuntimeHandlerAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.features, other.features)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        Ok(())
    }
}
