#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeSelectorTermAc {
    /// A list of node selector requirements by node's labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_expressions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::NodeSelectorRequirement as crate::Optionable>::Optioned,
        >,
    >,
    /// A list of node selector requirements by node's fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_fields: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::NodeSelectorRequirement as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeSelectorTerm {
    type Optioned = NodeSelectorTermAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSelectorTermAc {
    type Optioned = NodeSelectorTermAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeSelectorTerm {
    fn into_optioned(self) -> NodeSelectorTermAc {
        NodeSelectorTermAc {
            match_expressions: crate::OptionableConvert::into_optioned(
                self.match_expressions,
            ),
            match_fields: crate::OptionableConvert::into_optioned(self.match_fields),
        }
    }
    fn try_from_optioned(value: NodeSelectorTermAc) -> Result<Self, crate::Error> {
        Ok(Self {
            match_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_expressions,
            )?,
            match_fields: crate::OptionableConvert::try_from_optioned(
                value.match_fields,
            )?,
        })
    }
    fn merge(&mut self, other: NodeSelectorTermAc) -> Result<(), crate::Error> {
        if self.match_expressions.is_none() {
            self.match_expressions = crate::OptionableConvert::try_from_optioned(
                other.match_expressions,
            )?;
        } else {
            self.match_expressions = crate::OptionableConvert::try_from_optioned(
                other.match_expressions,
            )?;
        }
        if self.match_fields.is_none() {
            self.match_fields = crate::OptionableConvert::try_from_optioned(
                other.match_fields,
            )?;
        } else {
            self.match_fields = crate::OptionableConvert::try_from_optioned(
                other.match_fields,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeSelectorTerm>
for NodeSelectorTermAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeSelectorTerm) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeSelectorTerm, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeSelectorTerm,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
