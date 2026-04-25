#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HorizontalPodAutoscalerBehavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively).
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerBehaviorAc {
    /// scaleDown is scaling policy for scaling Down. If not set, the default value is to allow to scale down to minReplicas pods, with a 300 second stabilization window (i.e., the highest recommendation for the last 300sec is used).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down: Option<
        <::k8s_openapi027::api::autoscaling::v2::HPAScalingRules as crate::Optionable>::Optioned,
    >,
    /// scaleUp is scaling policy for scaling Up. If not set, the default value is the higher of:
    ///   * increase no more than 4 pods per 60 seconds
    ///   * double the number of pods per 60 seconds
    /// No stabilization is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_up: Option<
        <::k8s_openapi027::api::autoscaling::v2::HPAScalingRules as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerBehavior {
    type Optioned = HorizontalPodAutoscalerBehaviorAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerBehaviorAc {
    type Optioned = HorizontalPodAutoscalerBehaviorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerBehavior {
    fn into_optioned(self) -> HorizontalPodAutoscalerBehaviorAc {
        HorizontalPodAutoscalerBehaviorAc {
            scale_down: crate::OptionableConvert::into_optioned(self.scale_down),
            scale_up: crate::OptionableConvert::into_optioned(self.scale_up),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerBehaviorAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            scale_down: crate::OptionableConvert::try_from_optioned(value.scale_down)?,
            scale_up: crate::OptionableConvert::try_from_optioned(value.scale_up)?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerBehaviorAc,
    ) -> Result<(), crate::Error> {
        if self.scale_down.is_none() {
            self.scale_down = crate::OptionableConvert::try_from_optioned(
                other.scale_down,
            )?;
        } else if let Some(self_value) = self.scale_down.as_mut()
            && let Some(other_value) = other.scale_down
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.scale_up.is_none() {
            self.scale_up = crate::OptionableConvert::try_from_optioned(other.scale_up)?;
        } else if let Some(self_value) = self.scale_up.as_mut()
            && let Some(other_value) = other.scale_up
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
> for HorizontalPodAutoscalerBehaviorAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for HorizontalPodAutoscalerBehaviorAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.scale_down, other.scale_down);
        k8s_openapi027::DeepMerge::merge_from(&mut self.scale_up, other.scale_up);
    }
}
