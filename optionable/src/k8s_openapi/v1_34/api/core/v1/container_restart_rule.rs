pub struct ContainerRestartRuleOpt {
    pub action: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub exit_codes: <Option<
        ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerRestartRule {
    type Optioned = ContainerRestartRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerRestartRuleOpt {
    type Optioned = ContainerRestartRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerRestartRule {
    fn into_optioned(self) -> ContainerRestartRuleOpt {
        ContainerRestartRuleOpt {
            action: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.action,
                ),
            ),
            exit_codes: <Option<
                ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
            > as crate::OptionableConvert>::into_optioned(self.exit_codes),
        }
    }
    fn try_from_optioned(
        value: ContainerRestartRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            action: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .action
                    .ok_or(crate::optionable::Error {
                        missing_field: "action",
                    })?,
            )?,
            exit_codes: <Option<
                ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
            > as crate::OptionableConvert>::try_from_optioned(value.exit_codes)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerRestartRuleOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.action {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.action,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
        > as crate::OptionableConvert>::merge(&mut self.exit_codes, other.exit_codes)?;
        Ok(())
    }
}
