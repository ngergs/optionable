pub struct MatchConditionOpt {
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::match_condition::MatchCondition {
    type Optioned = MatchConditionOpt;
}
#[automatically_derived]
impl crate::Optionable for MatchConditionOpt {
    type Optioned = MatchConditionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::match_condition::MatchCondition {
    fn into_optioned(self) -> MatchConditionOpt {
        MatchConditionOpt {
            expression: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.expression,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: MatchConditionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .expression
                    .ok_or(crate::optionable::Error {
                        missing_field: "expression",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: MatchConditionOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.expression {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.expression,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
