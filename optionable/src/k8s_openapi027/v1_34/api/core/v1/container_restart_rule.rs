#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerRestartRule describes how a container exit is handled.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerRestartRuleAc {
    /// Specifies the action taken on a container exit if the requirements are satisfied. The only possible value is "Restart" to restart the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<std::string::String>,
    /// Represents the exit codes to check on container exits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_codes: Option<
        <::k8s_openapi027::api::core::v1::ContainerRestartRuleOnExitCodes as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ContainerRestartRule {
    type Optioned = ContainerRestartRuleAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerRestartRuleAc {
    type Optioned = ContainerRestartRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ContainerRestartRule {
    fn into_optioned(self) -> ContainerRestartRuleAc {
        ContainerRestartRuleAc {
            action: Some(self.action),
            exit_codes: crate::OptionableConvert::into_optioned(self.exit_codes),
        }
    }
    fn try_from_optioned(value: ContainerRestartRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            action: value
                .action
                .ok_or(crate::Error {
                    missing_field: "action",
                })?,
            exit_codes: crate::OptionableConvert::try_from_optioned(value.exit_codes)?,
        })
    }
    fn merge(&mut self, other: ContainerRestartRuleAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.action {
            self.action = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.exit_codes.is_none() {
            self.exit_codes = crate::OptionableConvert::try_from_optioned(
                other.exit_codes,
            )?;
        } else if let Some(self_value) = self.exit_codes.as_mut()
            && let Some(other_value) = other.exit_codes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ContainerRestartRule>
for ContainerRestartRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ContainerRestartRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ContainerRestartRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerRestartRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ContainerRestartRuleAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.action, other.action);
        k8s_openapi027::DeepMerge::merge_from(&mut self.exit_codes, other.exit_codes);
    }
}
