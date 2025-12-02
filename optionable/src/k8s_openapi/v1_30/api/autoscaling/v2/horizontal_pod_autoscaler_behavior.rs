#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerBehaviorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down: <Option<
        ::k8s_openapi::api::autoscaling::v2::HPAScalingRules,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
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
        crate::OptionableConvert::merge(&mut self.scale_down, other.scale_down)?;
        crate::OptionableConvert::merge(&mut self.scale_up, other.scale_up)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
> for HorizontalPodAutoscalerBehaviorAc {
    fn from_optionable(
        value: ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
