#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HPAScalingPolicyAc {
    /// periodSeconds specifies the window of time for which the policy should hold true. PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    /// type is used to specify the scaling policy.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
    /// value contains the amount of change which is permitted by the policy. It must be greater than zero
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::autoscaling::v2::HPAScalingPolicy {
    type Optioned = HPAScalingPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for HPAScalingPolicyAc {
    type Optioned = HPAScalingPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::HPAScalingPolicy {
    fn into_optioned(self) -> HPAScalingPolicyAc {
        HPAScalingPolicyAc {
            period_seconds: Some(self.period_seconds),
            type_: Some(self.type_),
            value: Some(self.value),
        }
    }
    fn try_from_optioned(value: HPAScalingPolicyAc) -> Result<Self, crate::Error> {
        Ok(Self {
            period_seconds: value
                .period_seconds
                .ok_or(crate::Error {
                    missing_field: "period_seconds",
                })?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
            value: value
                .value
                .ok_or(crate::Error {
                    missing_field: "value",
                })?,
        })
    }
    fn merge(&mut self, other: HPAScalingPolicyAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.period_seconds {
            self.period_seconds = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.value {
            self.value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::autoscaling::v2::HPAScalingPolicy>
for HPAScalingPolicyAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::HPAScalingPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::autoscaling::v2::HPAScalingPolicy, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::HPAScalingPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for HPAScalingPolicyAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.period_seconds,
            other.period_seconds,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.type_, other.type_);
        k8s_openapi027::DeepMerge::merge_from(&mut self.value, other.value);
    }
}
