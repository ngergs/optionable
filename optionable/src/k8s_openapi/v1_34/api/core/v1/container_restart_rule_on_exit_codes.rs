pub struct ContainerRestartRuleOnExitCodesOpt {
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub values: <Option<std::vec::Vec<i32>> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::container_restart_rule_on_exit_codes::ContainerRestartRuleOnExitCodes {
    type Optioned = ContainerRestartRuleOnExitCodesOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerRestartRuleOnExitCodesOpt {
    type Optioned = ContainerRestartRuleOnExitCodesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::container_restart_rule_on_exit_codes::ContainerRestartRuleOnExitCodes {
    fn into_optioned(self) -> ContainerRestartRuleOnExitCodesOpt {
        ContainerRestartRuleOnExitCodesOpt {
            operator: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.operator,
                ),
            ),
            values: <Option<
                std::vec::Vec<i32>,
            > as crate::OptionableConvert>::into_optioned(self.values),
        }
    }
    fn try_from_optioned(
        value: ContainerRestartRuleOnExitCodesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            operator: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .operator
                    .ok_or(crate::optionable::Error {
                        missing_field: "operator",
                    })?,
            )?,
            values: <Option<
                std::vec::Vec<i32>,
            > as crate::OptionableConvert>::try_from_optioned(value.values)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerRestartRuleOnExitCodesOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.operator {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.operator,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<i32>,
        > as crate::OptionableConvert>::merge(&mut self.values, other.values)?;
        Ok(())
    }
}
