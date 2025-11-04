#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalPodAutoscalerStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_cpu_utilization_percentage: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scale_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscalerStatus {
    type Optioned = HorizontalPodAutoscalerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerStatusAc {
    type Optioned = HorizontalPodAutoscalerStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscalerStatus {
    fn into_optioned(self) -> HorizontalPodAutoscalerStatusAc {
        HorizontalPodAutoscalerStatusAc {
            current_cpu_utilization_percentage: crate::OptionableConvert::into_optioned(
                self.current_cpu_utilization_percentage,
            ),
            current_replicas: Some(self.current_replicas),
            desired_replicas: Some(self.desired_replicas),
            last_scale_time: crate::OptionableConvert::into_optioned(
                self.last_scale_time,
            ),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            current_cpu_utilization_percentage: crate::OptionableConvert::try_from_optioned(
                value.current_cpu_utilization_percentage,
            )?,
            current_replicas: value
                .current_replicas
                .ok_or(crate::optionable::Error {
                    missing_field: "current_replicas",
                })?,
            desired_replicas: value
                .desired_replicas
                .ok_or(crate::optionable::Error {
                    missing_field: "desired_replicas",
                })?,
            last_scale_time: crate::OptionableConvert::try_from_optioned(
                value.last_scale_time,
            )?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.current_cpu_utilization_percentage,
            other.current_cpu_utilization_percentage,
        )?;
        if let Some(other_value) = other.current_replicas {
            self.current_replicas = other_value;
        }
        if let Some(other_value) = other.desired_replicas {
            self.desired_replicas = other_value;
        }
        crate::OptionableConvert::merge(
            &mut self.last_scale_time,
            other.last_scale_time,
        )?;
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        Ok(())
    }
}
