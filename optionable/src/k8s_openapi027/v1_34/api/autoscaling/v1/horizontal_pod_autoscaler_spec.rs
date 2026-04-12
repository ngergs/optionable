#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// specification of a horizontal pod autoscaler.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerSpecAc {
    /// maxReplicas is the upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i32>,
    /// minReplicas is the lower limit for the number of replicas to which the autoscaler can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the alpha feature gate HPAScaleToZero is enabled and at least one Object or External metric is configured.  Scaling is active as long as at least one metric value is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_replicas: Option<i32>,
    /// reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_target_ref: Option<
        <::k8s_openapi027::api::autoscaling::v1::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
    /// targetCPUUtilizationPercentage is the target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.
    #[serde(rename = "targetCPUUtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cpu_utilization_percentage: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerSpec {
    type Optioned = HorizontalPodAutoscalerSpecAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerSpecAc {
    type Optioned = HorizontalPodAutoscalerSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerSpec {
    fn into_optioned(self) -> HorizontalPodAutoscalerSpecAc {
        HorizontalPodAutoscalerSpecAc {
            max_replicas: Some(self.max_replicas),
            min_replicas: self.min_replicas,
            scale_target_ref: Some(
                crate::OptionableConvert::into_optioned(self.scale_target_ref),
            ),
            target_cpu_utilization_percentage: self.target_cpu_utilization_percentage,
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            max_replicas: value
                .max_replicas
                .ok_or(crate::Error {
                    missing_field: "max_replicas",
                })?,
            min_replicas: value.min_replicas,
            scale_target_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .scale_target_ref
                    .ok_or(crate::Error {
                        missing_field: "scale_target_ref",
                    })?,
            )?,
            target_cpu_utilization_percentage: value.target_cpu_utilization_percentage,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerSpecAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.max_replicas {
            self.max_replicas = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.min_replicas.is_none() {
            self.min_replicas = crate::OptionableConvert::try_from_optioned(
                other.min_replicas,
            )?;
        } else if let Some(self_value) = self.min_replicas.as_mut()
            && let Some(other_value) = other.min_replicas
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.scale_target_ref {
            self.scale_target_ref = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.target_cpu_utilization_percentage.is_none() {
            self.target_cpu_utilization_percentage = crate::OptionableConvert::try_from_optioned(
                other.target_cpu_utilization_percentage,
            )?;
        } else if let Some(self_value) = self.target_cpu_utilization_percentage.as_mut()
            && let Some(other_value) = other.target_cpu_utilization_percentage
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerSpec,
> for HorizontalPodAutoscalerSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v1::HorizontalPodAutoscalerSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
