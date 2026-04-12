#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeConfigSource specifies a source of node configuration. Exactly one subfield (excluding metadata) must be non-nil. This API is deprecated since 1.22
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeConfigSourceAc {
    /// ConfigMap is a reference to a Node's ConfigMap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: Option<
        <::k8s_openapi027::api::core::v1::ConfigMapNodeConfigSource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeConfigSource {
    type Optioned = NodeConfigSourceAc;
}
#[automatically_derived]
impl crate::Optionable for NodeConfigSourceAc {
    type Optioned = NodeConfigSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeConfigSource {
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
        if self.config_map.is_none() {
            self.config_map = crate::OptionableConvert::try_from_optioned(
                other.config_map,
            )?;
        } else if let Some(self_value) = self.config_map.as_mut()
            && let Some(other_value) = other.config_map
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeConfigSource>
for NodeConfigSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeConfigSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeConfigSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeConfigSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
