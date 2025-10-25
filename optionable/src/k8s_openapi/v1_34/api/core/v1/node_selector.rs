pub struct NodeSelectorOpt {
    pub node_selector_terms: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::core::v1::NodeSelectorTerm,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::node_selector::NodeSelector {
    type Optioned = NodeSelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeSelectorOpt {
    type Optioned = NodeSelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::node_selector::NodeSelector {
    fn into_optioned(self) -> NodeSelectorOpt {
        NodeSelectorOpt {
            node_selector_terms: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::core::v1::NodeSelectorTerm,
                > as crate::OptionableConvert>::into_optioned(self.node_selector_terms),
            ),
        }
    }
    fn try_from_optioned(
        value: NodeSelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            node_selector_terms: <std::vec::Vec<
                ::k8s_openapi::api::core::v1::NodeSelectorTerm,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .node_selector_terms
                    .ok_or(crate::optionable::Error {
                        missing_field: "node_selector_terms",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NodeSelectorOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.node_selector_terms {
            <std::vec::Vec<
                ::k8s_openapi::api::core::v1::NodeSelectorTerm,
            > as crate::OptionableConvert>::merge(
                &mut self.node_selector_terms,
                other_value,
            )?;
        }
        Ok(())
    }
}
