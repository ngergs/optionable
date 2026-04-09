#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerStatusAc {
    /// conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met.
    pub conditions: Option<
        std::vec::Vec<
            ::k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerCondition,
        >,
    >,
    /// currentMetrics is the last read state of the metrics used by this autoscaler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_metrics: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::autoscaling::v2::MetricStatus as crate::Optionable>::Optioned,
        >,
    >,
    /// currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i32>,
    /// desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_replicas: Option<i32>,
    /// lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scale_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// observedGeneration is the most recent generation observed by this autoscaler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerStatus {
    type Optioned = HorizontalPodAutoscalerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerStatusAc {
    type Optioned = HorizontalPodAutoscalerStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerStatus {
    fn into_optioned(self) -> HorizontalPodAutoscalerStatusAc {
        HorizontalPodAutoscalerStatusAc {
            conditions: self.conditions,
            current_metrics: crate::OptionableConvert::into_optioned(
                self.current_metrics,
            ),
            current_replicas: self.current_replicas,
            desired_replicas: Some(self.desired_replicas),
            last_scale_time: crate::OptionableConvert::into_optioned(
                self.last_scale_time,
            ),
            observed_generation: self.observed_generation,
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: value.conditions,
            current_metrics: crate::OptionableConvert::try_from_optioned(
                value.current_metrics,
            )?,
            current_replicas: value.current_replicas,
            desired_replicas: value
                .desired_replicas
                .ok_or(crate::Error {
                    missing_field: "desired_replicas",
                })?,
            last_scale_time: crate::OptionableConvert::try_from_optioned(
                value.last_scale_time,
            )?,
            observed_generation: value.observed_generation,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerStatusAc,
    ) -> Result<(), crate::Error> {
        self.conditions = other.conditions;
        crate::OptionableConvert::merge(
            &mut self.current_metrics,
            other.current_metrics,
        )?;
        self.current_replicas = other.current_replicas;
        if let Some(other_value) = other.desired_replicas {
            self.desired_replicas = other_value;
        }
        crate::OptionableConvert::merge(
            &mut self.last_scale_time,
            other.last_scale_time,
        )?;
        self.observed_generation = other.observed_generation;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
> for HorizontalPodAutoscalerStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
