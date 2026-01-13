#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_target_ref: Option<
        <::k8s_openapi027::api::autoscaling::v1::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
    #[serde(rename = "targetCPUUtilizationPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cpu_utilization_percentage: <Option<i32> as crate::Optionable>::Optioned,
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
            min_replicas: crate::OptionableConvert::into_optioned(self.min_replicas),
            scale_target_ref: Some(
                crate::OptionableConvert::into_optioned(self.scale_target_ref),
            ),
            target_cpu_utilization_percentage: crate::OptionableConvert::into_optioned(
                self.target_cpu_utilization_percentage,
            ),
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
            target_cpu_utilization_percentage: crate::OptionableConvert::try_from_optioned(
                value.target_cpu_utilization_percentage,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerSpecAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.max_replicas {
            self.max_replicas = other_value;
        }
        crate::OptionableConvert::merge(&mut self.min_replicas, other.min_replicas)?;
        if let Some(other_value) = other.scale_target_ref {
            crate::OptionableConvert::merge(&mut self.scale_target_ref, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.target_cpu_utilization_percentage,
            other.target_cpu_utilization_percentage,
        )?;
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
