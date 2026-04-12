#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeRuntimeHandler is a set of runtime handler information.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeRuntimeHandlerAc {
    /// Supported features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<
        <::k8s_openapi027::api::core::v1::NodeRuntimeHandlerFeatures as crate::Optionable>::Optioned,
    >,
    /// Runtime handler name. Empty for the default runtime handler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeRuntimeHandler {
    type Optioned = NodeRuntimeHandlerAc;
}
#[automatically_derived]
impl crate::Optionable for NodeRuntimeHandlerAc {
    type Optioned = NodeRuntimeHandlerAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeRuntimeHandler {
    fn into_optioned(self) -> NodeRuntimeHandlerAc {
        NodeRuntimeHandlerAc {
            features: crate::OptionableConvert::into_optioned(self.features),
            name: self.name,
        }
    }
    fn try_from_optioned(value: NodeRuntimeHandlerAc) -> Result<Self, crate::Error> {
        Ok(Self {
            features: crate::OptionableConvert::try_from_optioned(value.features)?,
            name: value.name,
        })
    }
    fn merge(&mut self, other: NodeRuntimeHandlerAc) -> Result<(), crate::Error> {
        if self.features.is_none() {
            self.features = crate::OptionableConvert::try_from_optioned(other.features)?;
        } else {
            crate::OptionableConvert::merge(&mut self.features, other.features)?;
        }
        if self.name.is_none() {
            self.name = crate::OptionableConvert::try_from_optioned(other.name)?;
        } else {
            crate::OptionableConvert::merge(&mut self.name, other.name)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeRuntimeHandler>
for NodeRuntimeHandlerAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::NodeRuntimeHandler,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeRuntimeHandler, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeRuntimeHandler,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
