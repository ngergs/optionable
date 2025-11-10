#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NodeSelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector_terms: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::core::v1::NodeSelectorTerm,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeSelector {
    type Optioned = NodeSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSelectorAc {
    type Optioned = NodeSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeSelector {
    fn into_optioned(self) -> NodeSelectorAc {
        NodeSelectorAc {
            node_selector_terms: Some(
                crate::OptionableConvert::into_optioned(self.node_selector_terms),
            ),
        }
    }
    fn try_from_optioned(value: NodeSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            node_selector_terms: crate::OptionableConvert::try_from_optioned(
                value
                    .node_selector_terms
                    .ok_or(crate::Error {
                        missing_field: "node_selector_terms",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NodeSelectorAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.node_selector_terms {
            crate::OptionableConvert::merge(&mut self.node_selector_terms, other_value)?;
        }
        Ok(())
    }
}
