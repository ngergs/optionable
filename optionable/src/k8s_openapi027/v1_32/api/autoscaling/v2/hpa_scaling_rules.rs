#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HPAScalingRules configures the scaling behavior for one direction. These Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HPAScalingRulesAc {
    /// policies is a list of potential scaling polices which can be used during scaling. At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::autoscaling::v2::HPAScalingPolicy as crate::Optionable>::Optioned,
        >,
    >,
    /// selectPolicy is used to specify which policy should be used. If not set, the default value Max is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_policy: Option<std::string::String>,
    /// stabilizationWindowSeconds is the number of seconds for which past recommendations should be considered while scaling up or scaling down. StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour). If not set, use the default values: - For scale up: 0 (i.e. no stabilization is done). - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stabilization_window_seconds: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v2::HPAScalingRules {
    type Optioned = HPAScalingRulesAc;
}
#[automatically_derived]
impl crate::Optionable for HPAScalingRulesAc {
    type Optioned = HPAScalingRulesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::autoscaling::v2::HPAScalingRules {
    fn into_optioned(self) -> HPAScalingRulesAc {
        HPAScalingRulesAc {
            policies: crate::OptionableConvert::into_optioned(self.policies),
            select_policy: self.select_policy,
            stabilization_window_seconds: self.stabilization_window_seconds,
        }
    }
    fn try_from_optioned(value: HPAScalingRulesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            policies: crate::OptionableConvert::try_from_optioned(value.policies)?,
            select_policy: value.select_policy,
            stabilization_window_seconds: value.stabilization_window_seconds,
        })
    }
    fn merge(&mut self, other: HPAScalingRulesAc) -> Result<(), crate::Error> {
        if self.policies.is_none() {
            self.policies = crate::OptionableConvert::try_from_optioned(other.policies)?;
        } else if let Some(self_value) = self.policies.as_mut()
            && let Some(other_value) = other.policies
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.select_policy.is_none() {
            self.select_policy = crate::OptionableConvert::try_from_optioned(
                other.select_policy,
            )?;
        } else if let Some(self_value) = self.select_policy.as_mut()
            && let Some(other_value) = other.select_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.stabilization_window_seconds.is_none() {
            self.stabilization_window_seconds = crate::OptionableConvert::try_from_optioned(
                other.stabilization_window_seconds,
            )?;
        } else if let Some(self_value) = self.stabilization_window_seconds.as_mut()
            && let Some(other_value) = other.stabilization_window_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v2::HPAScalingRules>
for HPAScalingRulesAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::HPAScalingRules,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v2::HPAScalingRules, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::HPAScalingRules,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for HPAScalingRulesAc {
    fn merge_from(&mut self, other: Self) {
        self.policies = other.policies;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.select_policy,
            other.select_policy,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.stabilization_window_seconds,
            other.stabilization_window_seconds,
        );
    }
}
