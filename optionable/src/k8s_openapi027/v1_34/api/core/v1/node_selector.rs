#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A node selector represents the union of the results of one or more label queries over a set of nodes; that is, it represents the OR of the selectors represented by the node selector terms.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeSelectorAc {
    /// Required. A list of node selector terms. The terms are ORed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_selector_terms: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::NodeSelectorTerm as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeSelector {
    type Optioned = NodeSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSelectorAc {
    type Optioned = NodeSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeSelector {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeSelector>
for NodeSelectorAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeSelector) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeSelector, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
