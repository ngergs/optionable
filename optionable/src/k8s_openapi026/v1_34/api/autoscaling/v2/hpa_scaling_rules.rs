#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HPAScalingRulesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: <Option<
        std::vec::Vec<::k8s_openapi026::api::autoscaling::v2::HPAScalingPolicy>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stabilization_window_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: <Option<
        ::k8s_openapi026::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::autoscaling::v2::HPAScalingRules {
    type Optioned = HPAScalingRulesAc;
}
#[automatically_derived]
impl crate::Optionable for HPAScalingRulesAc {
    type Optioned = HPAScalingRulesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::autoscaling::v2::HPAScalingRules {
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
    fn try_from_optioned(value: HPAScalingRulesAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: HPAScalingRulesAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::autoscaling::v2::HPAScalingRules>
for HPAScalingRulesAc {
    fn from_optionable(
        value: k8s_openapi026::api::autoscaling::v2::HPAScalingRules,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::autoscaling::v2::HPAScalingRules, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::autoscaling::v2::HPAScalingRules,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
