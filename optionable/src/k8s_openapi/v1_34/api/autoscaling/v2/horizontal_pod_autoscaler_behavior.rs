pub struct HorizontalPodAutoscalerBehaviorAc {
    pub scale_down: <Option<
        ::k8s_openapi::api::autoscaling::v2::HPAScalingRules,
    > as crate::Optionable>::Optioned,
    pub scale_up: <Option<
        ::k8s_openapi::api::autoscaling::v2::HPAScalingRules,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior {
    type Optioned = HorizontalPodAutoscalerBehaviorAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerBehaviorAc {
    type Optioned = HorizontalPodAutoscalerBehaviorAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior {
    fn into_optioned(self) -> HorizontalPodAutoscalerBehaviorAc {
        HorizontalPodAutoscalerBehaviorAc {
            scale_down: crate::OptionableConvert::into_optioned(self.scale_down),
            scale_up: crate::OptionableConvert::into_optioned(self.scale_up),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerBehaviorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            scale_down: crate::OptionableConvert::try_from_optioned(value.scale_down)?,
            scale_up: crate::OptionableConvert::try_from_optioned(value.scale_up)?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerBehaviorAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.scale_down, other.scale_down)?;
        crate::OptionableConvert::merge(&mut self.scale_up, other.scale_up)?;
        Ok(())
    }
}
