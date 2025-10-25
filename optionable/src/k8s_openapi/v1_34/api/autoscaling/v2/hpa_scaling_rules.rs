pub struct HPAScalingRulesOpt {
    pub policies: <Option<
        std::vec::Vec<::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy>,
    > as crate::Optionable>::Optioned,
    pub select_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub stabilization_window_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub tolerance: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::hpa_scaling_rules::HPAScalingRules {
    type Optioned = HPAScalingRulesOpt;
}
#[automatically_derived]
impl crate::Optionable for HPAScalingRulesOpt {
    type Optioned = HPAScalingRulesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::hpa_scaling_rules::HPAScalingRules {
    fn into_optioned(self) -> HPAScalingRulesOpt {
        HPAScalingRulesOpt {
            policies: <Option<
                std::vec::Vec<::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy>,
            > as crate::OptionableConvert>::into_optioned(self.policies),
            select_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.select_policy),
            stabilization_window_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(
                self.stabilization_window_seconds,
            ),
            tolerance: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::into_optioned(self.tolerance),
        }
    }
    fn try_from_optioned(
        value: HPAScalingRulesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            policies: <Option<
                std::vec::Vec<::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy>,
            > as crate::OptionableConvert>::try_from_optioned(value.policies)?,
            select_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.select_policy)?,
            stabilization_window_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.stabilization_window_seconds,
            )?,
            tolerance: <Option<
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            > as crate::OptionableConvert>::try_from_optioned(value.tolerance)?,
        })
    }
    fn merge(
        &mut self,
        other: HPAScalingRulesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::autoscaling::v2::HPAScalingPolicy>,
        > as crate::OptionableConvert>::merge(&mut self.policies, other.policies)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.select_policy,
            other.select_policy,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.stabilization_window_seconds,
            other.stabilization_window_seconds,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        > as crate::OptionableConvert>::merge(&mut self.tolerance, other.tolerance)?;
        Ok(())
    }
}
