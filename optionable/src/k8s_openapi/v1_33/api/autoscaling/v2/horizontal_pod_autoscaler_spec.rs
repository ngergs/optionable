#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: <Option<
        ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: <Option<
        std::vec::Vec<::k8s_openapi::api::autoscaling::v2::MetricSpec>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_target_ref: Option<
        <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerSpec {
    type Optioned = HorizontalPodAutoscalerSpecAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerSpecAc {
    type Optioned = HorizontalPodAutoscalerSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerSpec {
    fn into_optioned(self) -> HorizontalPodAutoscalerSpecAc {
        HorizontalPodAutoscalerSpecAc {
            behavior: crate::OptionableConvert::into_optioned(self.behavior),
            max_replicas: Some(self.max_replicas),
            metrics: crate::OptionableConvert::into_optioned(self.metrics),
            min_replicas: crate::OptionableConvert::into_optioned(self.min_replicas),
            scale_target_ref: Some(
                crate::OptionableConvert::into_optioned(self.scale_target_ref),
            ),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            behavior: crate::OptionableConvert::try_from_optioned(value.behavior)?,
            max_replicas: value
                .max_replicas
                .ok_or(crate::Error {
                    missing_field: "max_replicas",
                })?,
            metrics: crate::OptionableConvert::try_from_optioned(value.metrics)?,
            min_replicas: crate::OptionableConvert::try_from_optioned(
                value.min_replicas,
            )?,
            scale_target_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .scale_target_ref
                    .ok_or(crate::Error {
                        missing_field: "scale_target_ref",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerSpecAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.behavior, other.behavior)?;
        if let Some(other_value) = other.max_replicas {
            self.max_replicas = other_value;
        }
        crate::OptionableConvert::merge(&mut self.metrics, other.metrics)?;
        crate::OptionableConvert::merge(&mut self.min_replicas, other.min_replicas)?;
        if let Some(other_value) = other.scale_target_ref {
            crate::OptionableConvert::merge(&mut self.scale_target_ref, other_value)?;
        }
        Ok(())
    }
}
