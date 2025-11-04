#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct TopologySelectorLabelRequirementAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::TopologySelectorLabelRequirement {
    type Optioned = TopologySelectorLabelRequirementAc;
}
#[automatically_derived]
impl crate::Optionable for TopologySelectorLabelRequirementAc {
    type Optioned = TopologySelectorLabelRequirementAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::TopologySelectorLabelRequirement {
    fn into_optioned(self) -> TopologySelectorLabelRequirementAc {
        TopologySelectorLabelRequirementAc {
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            values: Some(crate::OptionableConvert::into_optioned(self.values)),
        }
    }
    fn try_from_optioned(
        value: TopologySelectorLabelRequirementAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            values: crate::OptionableConvert::try_from_optioned(
                value
                    .values
                    .ok_or(crate::optionable::Error {
                        missing_field: "values",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: TopologySelectorLabelRequirementAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        if let Some(other_value) = other.values {
            crate::OptionableConvert::merge(&mut self.values, other_value)?;
        }
        Ok(())
    }
}
