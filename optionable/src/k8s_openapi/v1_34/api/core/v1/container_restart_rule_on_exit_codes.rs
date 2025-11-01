pub struct ContainerRestartRuleOnExitCodesAc {
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub values: <Option<std::vec::Vec<i32>> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes {
    type Optioned = ContainerRestartRuleOnExitCodesAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerRestartRuleOnExitCodesAc {
    type Optioned = ContainerRestartRuleOnExitCodesAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes {
    fn into_optioned(self) -> ContainerRestartRuleOnExitCodesAc {
        ContainerRestartRuleOnExitCodesAc {
            operator: Some(crate::OptionableConvert::into_optioned(self.operator)),
            values: crate::OptionableConvert::into_optioned(self.values),
        }
    }
    fn try_from_optioned(
        value: ContainerRestartRuleOnExitCodesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
        other: ContainerRestartRuleOnExitCodesAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.operator {
            crate::OptionableConvert::merge(&mut self.operator, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.values, other.values)?;
        Ok(())
    }
}
