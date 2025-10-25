pub struct PodFailurePolicyRuleOpt {
    pub action: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub on_exit_codes: <Option<
        ::k8s_openapi::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
    > as crate::Optionable>::Optioned,
    pub on_pod_conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::PodFailurePolicyRule {
    type Optioned = PodFailurePolicyRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for PodFailurePolicyRuleOpt {
    type Optioned = PodFailurePolicyRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::batch::v1::PodFailurePolicyRule {
    fn into_optioned(self) -> PodFailurePolicyRuleOpt {
        PodFailurePolicyRuleOpt {
            action: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.action,
                ),
            ),
            on_exit_codes: <Option<
                ::k8s_openapi::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
            > as crate::OptionableConvert>::into_optioned(self.on_exit_codes),
            on_pod_conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
                >,
            > as crate::OptionableConvert>::into_optioned(self.on_pod_conditions),
        }
    }
    fn try_from_optioned(
        value: PodFailurePolicyRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            action: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .action
                    .ok_or(crate::optionable::Error {
                        missing_field: "action",
                    })?,
            )?,
            on_exit_codes: <Option<
                ::k8s_openapi::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
            > as crate::OptionableConvert>::try_from_optioned(value.on_exit_codes)?,
            on_pod_conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.on_pod_conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: PodFailurePolicyRuleOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.action {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.action,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::batch::v1::PodFailurePolicyOnExitCodesRequirement,
        > as crate::OptionableConvert>::merge(
            &mut self.on_exit_codes,
            other.on_exit_codes,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::batch::v1::PodFailurePolicyOnPodConditionsPattern,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.on_pod_conditions,
            other.on_pod_conditions,
        )?;
        Ok(())
    }
}
