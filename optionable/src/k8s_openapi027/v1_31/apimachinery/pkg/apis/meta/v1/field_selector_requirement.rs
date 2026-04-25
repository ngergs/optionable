#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FieldSelectorRequirement is a selector that contains values, a key, and an operator that relates the key and values.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldSelectorRequirementAc {
    /// key is the field selector key that the requirement applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. The list of operators may grow in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<std::string::String>,
    /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement {
    type Optioned = FieldSelectorRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for FieldSelectorRequirementAc {
    type Optioned = FieldSelectorRequirementAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement {
    fn into_optioned(self) -> FieldSelectorRequirementAc {
        FieldSelectorRequirementAc {
            key: Some(self.key),
            operator: Some(self.operator),
            values: self.values,
        }
    }
    fn try_from_optioned(
        value: FieldSelectorRequirementAc,
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
    fn merge(&mut self, other: FieldSelectorRequirementAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.operator {
            self.operator = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.values.is_none() {
            self.values = crate::OptionableConvert::try_from_optioned(other.values)?;
        } else if let Some(self_value) = self.values.as_mut()
            && let Some(other_value) = other.values
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
> for FieldSelectorRequirementAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for FieldSelectorRequirementAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.key, other.key);
        k8s_openapi027::DeepMerge::merge_from(&mut self.operator, other.operator);
        self.values = other.values;
    }
}
