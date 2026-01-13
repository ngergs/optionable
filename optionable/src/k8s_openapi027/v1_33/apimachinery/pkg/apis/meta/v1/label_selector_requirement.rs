#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LabelSelectorRequirementAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement {
    type Optioned = LabelSelectorRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for LabelSelectorRequirementAc {
    type Optioned = LabelSelectorRequirementAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement {
    fn into_optioned(self) -> LabelSelectorRequirementAc {
        LabelSelectorRequirementAc {
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            operator: Some(crate::OptionableConvert::into_optioned(self.operator)),
            values: crate::OptionableConvert::into_optioned(self.values),
        }
    }
    fn try_from_optioned(
        value: LabelSelectorRequirementAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::Error {
                        missing_field: "key",
                    })?,
            )?,
            operator: crate::OptionableConvert::try_from_optioned(
                value
                    .operator
                    .ok_or(crate::Error {
                        missing_field: "operator",
                    })?,
            )?,
            values: crate::OptionableConvert::try_from_optioned(value.values)?,
        })
    }
    fn merge(&mut self, other: LabelSelectorRequirementAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        if let Some(other_value) = other.operator {
            crate::OptionableConvert::merge(&mut self.operator, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.values, other.values)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
> for LabelSelectorRequirementAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
