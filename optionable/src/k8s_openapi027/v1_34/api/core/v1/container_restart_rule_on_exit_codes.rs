#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerRestartRuleOnExitCodes describes the condition for handling an exited container based on its exit codes.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerRestartRuleOnExitCodesAc {
    /// Represents the relationship between the container exit code(s) and the specified values. Possible values are: - In: the requirement is satisfied if the container exit code is in the
    ///   set of specified values.
    /// - NotIn: the requirement is satisfied if the container exit code is
    ///   not in the set of specified values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<std::string::String>,
    /// Specifies the set of values to check for container exit codes. At most 255 elements are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<std::vec::Vec<i32>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::ContainerRestartRuleOnExitCodes {
    type Optioned = ContainerRestartRuleOnExitCodesAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerRestartRuleOnExitCodesAc {
    type Optioned = ContainerRestartRuleOnExitCodesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ContainerRestartRuleOnExitCodes {
    fn into_optioned(self) -> ContainerRestartRuleOnExitCodesAc {
        ContainerRestartRuleOnExitCodesAc {
            operator: Some(self.operator),
            values: self.values,
        }
    }
    fn try_from_optioned(
        value: ContainerRestartRuleOnExitCodesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            operator: value
                .operator
                .ok_or(crate::Error {
                    missing_field: "operator",
                })?,
            values: value.values,
        })
    }
    fn merge(
        &mut self,
        other: ContainerRestartRuleOnExitCodesAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.operator {
            self.operator = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.values.is_none() {
            self.values = crate::OptionableConvert::try_from_optioned(other.values)?;
        } else if let Some(self_value) = self.values.as_mut()
            && let Some(other_value) = other.values
        {
            crate::merge::try_merge_optioned_set(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::ContainerRestartRuleOnExitCodes,
> for ContainerRestartRuleOnExitCodesAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ContainerRestartRuleOnExitCodes,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ContainerRestartRuleOnExitCodes,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerRestartRuleOnExitCodes,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
