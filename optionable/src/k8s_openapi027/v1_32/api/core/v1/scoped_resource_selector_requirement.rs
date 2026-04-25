#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A scoped-resource selector requirement is a selector that contains values, a scope name, and an operator that relates the scope name and values.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScopedResourceSelectorRequirementAc {
    /// Represents a scope's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<std::string::String>,
    /// The name of the scope that the selector applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<std::string::String>,
    /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::ScopedResourceSelectorRequirement {
    type Optioned = ScopedResourceSelectorRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for ScopedResourceSelectorRequirementAc {
    type Optioned = ScopedResourceSelectorRequirementAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ScopedResourceSelectorRequirement {
    fn into_optioned(self) -> ScopedResourceSelectorRequirementAc {
        ScopedResourceSelectorRequirementAc {
            operator: Some(self.operator),
            scope_name: Some(self.scope_name),
            values: self.values,
        }
    }
    fn try_from_optioned(
        value: ScopedResourceSelectorRequirementAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            operator: value
                .operator
                .ok_or(crate::Error {
                    missing_field: "operator",
                })?,
            scope_name: value
                .scope_name
                .ok_or(crate::Error {
                    missing_field: "scope_name",
                })?,
            values: value.values,
        })
    }
    fn merge(
        &mut self,
        other: ScopedResourceSelectorRequirementAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.operator {
            self.operator = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.scope_name {
            self.scope_name = crate::OptionableConvert::try_from_optioned(other_value)?;
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
    k8s_openapi027::api::core::v1::ScopedResourceSelectorRequirement,
> for ScopedResourceSelectorRequirementAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ScopedResourceSelectorRequirement,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ScopedResourceSelectorRequirement,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ScopedResourceSelectorRequirement,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ScopedResourceSelectorRequirementAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.operator, other.operator);
        k8s_openapi027::DeepMerge::merge_from(&mut self.scope_name, other.scope_name);
        self.values = other.values;
    }
}
