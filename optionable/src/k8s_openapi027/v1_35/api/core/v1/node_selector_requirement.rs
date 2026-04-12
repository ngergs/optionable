#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeSelectorRequirementAc {
    /// The label key that the selector applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<std::string::String>,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeSelectorRequirement {
    type Optioned = NodeSelectorRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSelectorRequirementAc {
    type Optioned = NodeSelectorRequirementAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::NodeSelectorRequirement {
    fn into_optioned(self) -> NodeSelectorRequirementAc {
        NodeSelectorRequirementAc {
            key: Some(self.key),
            operator: Some(self.operator),
            values: self.values,
        }
    }
    fn try_from_optioned(
        value: NodeSelectorRequirementAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            key: value
                .key
                .ok_or(crate::Error {
                    missing_field: "key",
                })?,
            operator: value
                .operator
                .ok_or(crate::Error {
                    missing_field: "operator",
                })?,
            values: value.values,
        })
    }
    fn merge(&mut self, other: NodeSelectorRequirementAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.operator {
            self.operator = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.values.is_none() {
            self.values = crate::OptionableConvert::try_from_optioned(other.values)?;
        } else {
            self.values = crate::OptionableConvert::try_from_optioned(other.values)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeSelectorRequirement>
for NodeSelectorRequirementAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::NodeSelectorRequirement,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeSelectorRequirement, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeSelectorRequirement,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
