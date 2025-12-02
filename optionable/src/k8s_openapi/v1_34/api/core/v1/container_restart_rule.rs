#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ContainerRestartRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerRestartRule {
    fn into_optioned(self) -> ContainerRestartRuleAc {
        ContainerRestartRuleAc {
            action: Some(crate::OptionableConvert::into_optioned(self.action)),
            exit_codes: crate::OptionableConvert::into_optioned(self.exit_codes),
        }
    }
    fn try_from_optioned(value: ContainerRestartRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            action: crate::OptionableConvert::try_from_optioned(
                value
                    .action
                    .ok_or(crate::Error {
                        missing_field: "action",
                    })?,
            )?,
            exit_codes: crate::OptionableConvert::try_from_optioned(value.exit_codes)?,
        })
    }
    fn merge(&mut self, other: ContainerRestartRuleAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.action {
            crate::OptionableConvert::merge(&mut self.action, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.exit_codes, other.exit_codes)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::ContainerRestartRule>
for ContainerRestartRuleAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::ContainerRestartRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::ContainerRestartRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ContainerRestartRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
