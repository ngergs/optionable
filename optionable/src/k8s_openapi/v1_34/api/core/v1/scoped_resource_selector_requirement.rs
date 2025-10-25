pub struct ScopedResourceSelectorRequirementOpt {
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub scope_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub values: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement {
    type Optioned = ScopedResourceSelectorRequirementOpt;
}
#[automatically_derived]
impl crate::Optionable for ScopedResourceSelectorRequirementOpt {
    type Optioned = ScopedResourceSelectorRequirementOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ScopedResourceSelectorRequirement {
    fn into_optioned(self) -> ScopedResourceSelectorRequirementOpt {
        ScopedResourceSelectorRequirementOpt {
            operator: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.operator,
                ),
            ),
            scope_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.scope_name,
                ),
            ),
            values: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.values),
        }
    }
    fn try_from_optioned(
        value: ScopedResourceSelectorRequirementOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            operator: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .operator
                    .ok_or(crate::optionable::Error {
                        missing_field: "operator",
                    })?,
            )?,
            scope_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .scope_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "scope_name",
                    })?,
            )?,
            values: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.values)?,
        })
    }
    fn merge(
        &mut self,
        other: ScopedResourceSelectorRequirementOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.operator {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.operator,
                other_value,
            )?;
        }
        if let Some(other_value) = other.scope_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.scope_name,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.values, other.values)?;
        Ok(())
    }
}
