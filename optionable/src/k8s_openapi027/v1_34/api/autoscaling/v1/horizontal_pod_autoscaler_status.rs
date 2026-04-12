#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// current status of a horizontal pod autoscaler
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerStatusAc {
    /// currentCPUUtilizationPercentage is the current average CPU utilization over all pods, represented as a percentage of requested CPU, e.g. 70 means that an average pod is using now 70% of its requested CPU.
    #[serde(rename = "currentCPUUtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_cpu_utilization_percentage: Option<i32>,
    /// currentReplicas is the current number of replicas of pods managed by this autoscaler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i32>,
    /// desiredReplicas is the  desired number of replicas of pods managed by this autoscaler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_replicas: Option<i32>,
    /// lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods; used by the autoscaler to control how often the number of pods is changed.
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
for k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerStatus {
    type Optioned = HorizontalPodAutoscalerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerStatusAc {
    type Optioned = HorizontalPodAutoscalerStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerStatus {
    fn into_optioned(self) -> HorizontalPodAutoscalerStatusAc {
        HorizontalPodAutoscalerStatusAc {
            current_cpu_utilization_percentage: self.current_cpu_utilization_percentage,
            current_replicas: Some(self.current_replicas),
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
            current_cpu_utilization_percentage: value.current_cpu_utilization_percentage,
            current_replicas: value
                .current_replicas
                .ok_or(crate::Error {
                    missing_field: "current_replicas",
                })?,
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
        if self.current_cpu_utilization_percentage.is_none() {
            self.current_cpu_utilization_percentage = crate::OptionableConvert::try_from_optioned(
                other.current_cpu_utilization_percentage,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.current_cpu_utilization_percentage,
                other.current_cpu_utilization_percentage,
            )?;
        }
        if let Some(other_value) = other.current_replicas {
            self.current_replicas = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.desired_replicas {
            self.desired_replicas = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.last_scale_time.is_none() {
            self.last_scale_time = crate::OptionableConvert::try_from_optioned(
                other.last_scale_time,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.last_scale_time,
                other.last_scale_time,
            )?;
        }
        if self.observed_generation.is_none() {
            self.observed_generation = crate::OptionableConvert::try_from_optioned(
                other.observed_generation,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.observed_generation,
                other.observed_generation,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerStatus,
> for HorizontalPodAutoscalerStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
