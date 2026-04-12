#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerSpecAc {
    /// behavior configures the scaling behavior of the target in both Up and Down directions (scaleUp and scaleDown fields respectively). If not set, the default HPAScalingRules for scale up and scale down are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<
        <::k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerBehavior as crate::Optionable>::Optioned,
    >,
    /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up. It cannot be less that minReplicas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i32>,
    /// metrics contains the specifications for which to use to calculate the desired replica count (the maximum replica count across all metrics will be used).  The desired replica count is calculated multiplying the ratio between the target value and the current value by the current number of pods.  Ergo, metrics used must decrease as the pod count is increased, and vice-versa.  See the individual metric source types for more information about how each type of metric must respond. If not set, the default metric will be set to 80% average CPU utilization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::autoscaling::v2::MetricSpec as crate::Optionable>::Optioned,
        >,
    >,
    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    /// scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics should be collected, as well as to actually change the replica count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_target_ref: Option<
        <::k8s_openapi027::api::autoscaling::v2::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerSpec {
    type Optioned = HorizontalPodAutoscalerSpecAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerSpecAc {
    type Optioned = HorizontalPodAutoscalerSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerSpec {
    fn into_optioned(self) -> HorizontalPodAutoscalerSpecAc {
        HorizontalPodAutoscalerSpecAc {
            behavior: crate::OptionableConvert::into_optioned(self.behavior),
            max_replicas: Some(self.max_replicas),
            metrics: crate::OptionableConvert::into_optioned(self.metrics),
            min_replicas: self.min_replicas,
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
            min_replicas: value.min_replicas,
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
        if other.min_replicas.is_some() {
            self.min_replicas = other.min_replicas;
        }
        if let Some(other_value) = other.scale_target_ref {
            crate::OptionableConvert::merge(&mut self.scale_target_ref, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerSpec,
> for HorizontalPodAutoscalerSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
