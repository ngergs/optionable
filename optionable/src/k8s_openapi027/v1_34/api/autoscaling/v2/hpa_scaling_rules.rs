#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HPAScalingRules configures the scaling behavior for one direction via scaling Policy Rules and a configurable metric tolerance.
///
/// Scaling Policy Rules are applied after calculating DesiredReplicas from metrics for the HPA. They can limit the scaling velocity by specifying scaling policies. They can prevent flapping by specifying the stabilization window, so that the number of replicas is not set instantly, instead, the safest value from the stabilization window is chosen.
///
/// The tolerance is applied to the metric values and prevents scaling too eagerly for small metric variations. (Note that setting a tolerance requires enabling the alpha HPAConfigurableTolerance feature gate.)
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HPAScalingRulesAc {
    /// policies is a list of potential scaling polices which can be used during scaling. If not set, use the default values: - For scale up: allow doubling the number of pods, or an absolute change of 4 pods in a 15s window. - For scale down: allow all pods to be removed in a 15s window.
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
    /// tolerance is the tolerance on the ratio between the current and desired metric value under which no updates are made to the desired number of replicas (e.g. 0.01 for 1%). Must be greater than or equal to zero. If not set, the default cluster-wide tolerance is applied (by default 10%).
    ///
    /// For example, if autoscaling is configured with a memory consumption target of 100Mi, and scale-down and scale-up tolerances of 5% and 1% respectively, scaling will be triggered when the actual consumption falls below 95Mi or exceeds 101Mi.
    ///
    /// This is an alpha field and requires enabling the HPAConfigurableTolerance feature gate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<
        <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
    >,
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
            tolerance: crate::OptionableConvert::into_optioned(self.tolerance),
        }
    }
    fn try_from_optioned(value: HPAScalingRulesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            policies: crate::OptionableConvert::try_from_optioned(value.policies)?,
            select_policy: value.select_policy,
            stabilization_window_seconds: value.stabilization_window_seconds,
            tolerance: crate::OptionableConvert::try_from_optioned(value.tolerance)?,
        })
    }
    fn merge(&mut self, other: HPAScalingRulesAc) -> Result<(), crate::Error> {
        if self.policies.is_none() {
            self.policies = other.policies;
        }
        if let Some(other_value) = other.policies {
            self.policies = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.select_policy.is_none() {
            self.select_policy = other.select_policy;
        }
        if let Some(other_value) = other.select_policy {
            crate::OptionableConvert::merge(&mut self.select_policy, other_value)?;
        }
        if self.stabilization_window_seconds.is_none() {
            self.stabilization_window_seconds = other.stabilization_window_seconds;
        }
        if let Some(other_value) = other.stabilization_window_seconds {
            crate::OptionableConvert::merge(
                &mut self.stabilization_window_seconds,
                other_value,
            )?;
        }
        if self.tolerance.is_none() {
            self.tolerance = other.tolerance;
        }
        if let Some(other_value) = other.tolerance {
            crate::OptionableConvert::merge(&mut self.tolerance, other_value)?;
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
