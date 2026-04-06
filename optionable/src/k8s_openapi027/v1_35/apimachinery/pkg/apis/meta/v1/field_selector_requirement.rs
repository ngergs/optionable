#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldSelectorRequirementAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<std::string::String>,
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
            self.key = other_value;
        }
        if let Some(other_value) = other.operator {
            self.operator = other_value;
        }
        self.values = other.values;
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
