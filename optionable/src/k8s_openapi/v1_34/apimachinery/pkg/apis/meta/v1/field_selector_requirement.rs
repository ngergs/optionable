pub struct FieldSelectorRequirementOpt {
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub values: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement {
    type Optioned = FieldSelectorRequirementOpt;
}
#[automatically_derived]
impl crate::Optionable for FieldSelectorRequirementOpt {
    type Optioned = FieldSelectorRequirementOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement {
    fn into_optioned(self) -> FieldSelectorRequirementOpt {
        FieldSelectorRequirementOpt {
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            operator: Some(crate::OptionableConvert::into_optioned(self.operator)),
            values: crate::OptionableConvert::into_optioned(self.values),
        }
    }
    fn try_from_optioned(
        value: FieldSelectorRequirementOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
            operator: crate::OptionableConvert::try_from_optioned(
                value
                    .operator
                    .ok_or(crate::optionable::Error {
                        missing_field: "operator",
                    })?,
            )?,
            values: crate::OptionableConvert::try_from_optioned(value.values)?,
        })
    }
    fn merge(
        &mut self,
        other: FieldSelectorRequirementOpt,
    ) -> Result<(), crate::optionable::Error> {
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
