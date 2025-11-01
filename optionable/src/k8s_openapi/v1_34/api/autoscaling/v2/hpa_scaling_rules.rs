pub struct HPAScalingRulesAc {
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
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::HPAScalingRules {
    type Optioned = HPAScalingRulesAc;
}
#[automatically_derived]
impl crate::Optionable for HPAScalingRulesAc {
    type Optioned = HPAScalingRulesAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v2::HPAScalingRules {
    fn into_optioned(self) -> HPAScalingRulesAc {
        HPAScalingRulesAc {
            policies: crate::OptionableConvert::into_optioned(self.policies),
            select_policy: crate::OptionableConvert::into_optioned(self.select_policy),
            stabilization_window_seconds: crate::OptionableConvert::into_optioned(
                self.stabilization_window_seconds,
            ),
            tolerance: crate::OptionableConvert::into_optioned(self.tolerance),
        }
    }
    fn try_from_optioned(
        value: HPAScalingRulesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            policies: crate::OptionableConvert::try_from_optioned(value.policies)?,
            select_policy: crate::OptionableConvert::try_from_optioned(
                value.select_policy,
            )?,
            stabilization_window_seconds: crate::OptionableConvert::try_from_optioned(
                value.stabilization_window_seconds,
            )?,
            tolerance: crate::OptionableConvert::try_from_optioned(value.tolerance)?,
        })
    }
    fn merge(
        &mut self,
        other: HPAScalingRulesAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.policies, other.policies)?;
        crate::OptionableConvert::merge(&mut self.select_policy, other.select_policy)?;
        crate::OptionableConvert::merge(
            &mut self.stabilization_window_seconds,
            other.stabilization_window_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.tolerance, other.tolerance)?;
        Ok(())
    }
}
