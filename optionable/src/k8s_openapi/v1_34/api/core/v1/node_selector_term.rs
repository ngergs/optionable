pub struct NodeSelectorTermAc {
    pub match_expressions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::NodeSelectorRequirement>,
    > as crate::Optionable>::Optioned,
    pub match_fields: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::NodeSelectorRequirement>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeSelectorTerm {
    type Optioned = NodeSelectorTermAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSelectorTermAc {
    type Optioned = NodeSelectorTermAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeSelectorTerm {
    fn into_optioned(self) -> NodeSelectorTermAc {
        NodeSelectorTermAc {
            match_expressions: crate::OptionableConvert::into_optioned(
                self.match_expressions,
            ),
            match_fields: crate::OptionableConvert::into_optioned(self.match_fields),
        }
    }
    fn try_from_optioned(
        value: NodeSelectorTermAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            match_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_expressions,
            )?,
            match_fields: crate::OptionableConvert::try_from_optioned(
                value.match_fields,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NodeSelectorTermAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_expressions,
            other.match_expressions,
        )?;
        crate::OptionableConvert::merge(&mut self.match_fields, other.match_fields)?;
        Ok(())
    }
}
