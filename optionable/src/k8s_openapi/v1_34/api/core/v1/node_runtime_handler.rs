pub struct NodeRuntimeHandlerOpt {
    pub features: <Option<
        ::k8s_openapi::api::core::v1::NodeRuntimeHandlerFeatures,
    > as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeRuntimeHandler {
    type Optioned = NodeRuntimeHandlerOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeRuntimeHandlerOpt {
    type Optioned = NodeRuntimeHandlerOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeRuntimeHandler {
    fn into_optioned(self) -> NodeRuntimeHandlerOpt {
        NodeRuntimeHandlerOpt {
            features: crate::OptionableConvert::into_optioned(self.features),
            name: crate::OptionableConvert::into_optioned(self.name),
        }
    }
    fn try_from_optioned(
        value: NodeRuntimeHandlerOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            features: crate::OptionableConvert::try_from_optioned(value.features)?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeRuntimeHandlerOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.features, other.features)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        Ok(())
    }
}
