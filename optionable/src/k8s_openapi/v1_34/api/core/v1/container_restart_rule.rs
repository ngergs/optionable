pub struct ContainerRestartRuleAc {
    pub action: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub exit_codes: <Option<
        ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerRestartRule {
    type Optioned = ContainerRestartRuleAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerRestartRuleAc {
    type Optioned = ContainerRestartRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerRestartRule {
    fn into_optioned(self) -> ContainerRestartRuleAc {
        ContainerRestartRuleAc {
            action: Some(crate::OptionableConvert::into_optioned(self.action)),
            exit_codes: crate::OptionableConvert::into_optioned(self.exit_codes),
        }
    }
    fn try_from_optioned(
        value: ContainerRestartRuleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            action: crate::OptionableConvert::try_from_optioned(
                value
                    .action
                    .ok_or(crate::optionable::Error {
                        missing_field: "action",
                    })?,
            )?,
            exit_codes: crate::OptionableConvert::try_from_optioned(value.exit_codes)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerRestartRuleAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.action {
            crate::OptionableConvert::merge(&mut self.action, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.exit_codes, other.exit_codes)?;
        Ok(())
    }
}
