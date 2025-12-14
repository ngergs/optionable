#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerRestartRuleOnExitCodesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
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
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            operator: crate::OptionableConvert::try_from_optioned(
                value
                    .operator
                    .ok_or(crate::Error {
                        missing_field: "operator",
                    })?,
            )?,
            values: crate::OptionableConvert::try_from_optioned(value.values)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerRestartRuleOnExitCodesAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.operator {
            crate::OptionableConvert::merge(&mut self.operator, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.values, other.values)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
> for ContainerRestartRuleOnExitCodesAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ContainerRestartRuleOnExitCodes,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
