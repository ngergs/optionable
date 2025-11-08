#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PodFailurePolicyRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit_codes: <Option<
        ::k8s_openapi::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_pod_conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::PodFailurePolicyRule {
    type Optioned = PodFailurePolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyRuleAc {
    type Optioned = PodFailurePolicyRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::PodFailurePolicyRule {
    fn into_optioned(self) -> PodFailurePolicyRuleAc {
        PodFailurePolicyRuleAc {
            action: Some(crate::OptionableConvert::into_optioned(self.action)),
            on_exit_codes: crate::OptionableConvert::into_optioned(self.on_exit_codes),
            on_pod_conditions: crate::OptionableConvert::into_optioned(
                self.on_pod_conditions,
            ),
        }
    }
    fn try_from_optioned(value: PodFailurePolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            action: crate::OptionableConvert::try_from_optioned(
                value
                    .action
                    .ok_or(crate::Error {
                        missing_field: "action",
                    })?,
            )?,
            on_exit_codes: crate::OptionableConvert::try_from_optioned(
                value.on_exit_codes,
            )?,
            on_pod_conditions: crate::OptionableConvert::try_from_optioned(
                value.on_pod_conditions,
            )?,
        })
    }
    fn merge(&mut self, other: PodFailurePolicyRuleAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.action {
            crate::OptionableConvert::merge(&mut self.action, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.on_exit_codes, other.on_exit_codes)?;
        crate::OptionableConvert::merge(
            &mut self.on_pod_conditions,
            other.on_pod_conditions,
        )?;
        Ok(())
    }
}
