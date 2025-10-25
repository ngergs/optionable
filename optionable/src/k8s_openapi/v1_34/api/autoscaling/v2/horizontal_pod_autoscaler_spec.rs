pub struct HorizontalPodAutoscalerSpecOpt {
    pub behavior: <Option<
        ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
    > as crate::Optionable>::Optioned,
    pub max_replicas: Option<i32>,
    pub metrics: <Option<
        std::vec::Vec<::k8s_openapi::api::autoscaling::v2::MetricSpec>,
    > as crate::Optionable>::Optioned,
    pub min_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub scale_target_ref: Option<
        <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::horizontal_pod_autoscaler_spec::HorizontalPodAutoscalerSpec {
    type Optioned = HorizontalPodAutoscalerSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerSpecOpt {
    type Optioned = HorizontalPodAutoscalerSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::horizontal_pod_autoscaler_spec::HorizontalPodAutoscalerSpec {
    fn into_optioned(self) -> HorizontalPodAutoscalerSpecOpt {
        HorizontalPodAutoscalerSpecOpt {
            behavior: <Option<
                ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
            > as crate::OptionableConvert>::into_optioned(self.behavior),
            max_replicas: Some(self.max_replicas),
            metrics: <Option<
                std::vec::Vec<::k8s_openapi::api::autoscaling::v2::MetricSpec>,
            > as crate::OptionableConvert>::into_optioned(self.metrics),
            min_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.min_replicas),
            scale_target_ref: Some(
                <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::OptionableConvert>::into_optioned(
                    self.scale_target_ref,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            behavior: <Option<
                ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
            > as crate::OptionableConvert>::try_from_optioned(value.behavior)?,
            max_replicas: value
                .max_replicas
                .ok_or(crate::optionable::Error {
                    missing_field: "max_replicas",
                })?,
            metrics: <Option<
                std::vec::Vec<::k8s_openapi::api::autoscaling::v2::MetricSpec>,
            > as crate::OptionableConvert>::try_from_optioned(value.metrics)?,
            min_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.min_replicas)?,
            scale_target_ref: <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::OptionableConvert>::try_from_optioned(
                value
                    .scale_target_ref
                    .ok_or(crate::optionable::Error {
                        missing_field: "scale_target_ref",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerBehavior,
        > as crate::OptionableConvert>::merge(&mut self.behavior, other.behavior)?;
        if let Some(other_value) = other.max_replicas {
            self.max_replicas = other_value;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::autoscaling::v2::MetricSpec>,
        > as crate::OptionableConvert>::merge(&mut self.metrics, other.metrics)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.min_replicas,
            other.min_replicas,
        )?;
        if let Some(other_value) = other.scale_target_ref {
            <::k8s_openapi::api::autoscaling::v2::CrossVersionObjectReference as crate::OptionableConvert>::merge(
                &mut self.scale_target_ref,
                other_value,
            )?;
        }
        Ok(())
    }
}
