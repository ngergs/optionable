#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A topology selector requirement is a selector that matches given label. This is an alpha feature and may change in the future.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TopologySelectorLabelRequirementAc {
    /// The label key that the selector applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    /// An array of string values. One value must match the label to be selected. Each entry in Values is ORed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::TopologySelectorLabelRequirement {
    type Optioned = TopologySelectorLabelRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for TopologySelectorLabelRequirementAc {
    type Optioned = TopologySelectorLabelRequirementAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::TopologySelectorLabelRequirement {
    fn into_optioned(self) -> TopologySelectorLabelRequirementAc {
        TopologySelectorLabelRequirementAc {
            key: Some(self.key),
            values: Some(self.values),
        }
    }
    fn try_from_optioned(
        value: TopologySelectorLabelRequirementAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            key: value
                .key
                .ok_or(crate::Error {
                    missing_field: "key",
                })?,
            values: value
                .values
                .ok_or(crate::Error {
                    missing_field: "values",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: TopologySelectorLabelRequirementAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            self.key = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.values {
            self.values = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::TopologySelectorLabelRequirement,
> for TopologySelectorLabelRequirementAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::TopologySelectorLabelRequirement,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::TopologySelectorLabelRequirement,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::TopologySelectorLabelRequirement,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
