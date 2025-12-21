#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<
            ::k8s_openapi026::api::autoscaling::v2::HorizontalPodAutoscalerCondition,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_metrics: <Option<
        std::vec::Vec<::k8s_openapi026::api::autoscaling::v2::MetricStatus>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_replicas: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scale_time: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::autoscaling::v2::HorizontalPodAutoscalerStatus {
    type Optioned = HorizontalPodAutoscalerStatusAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerStatusAc {
    type Optioned = HorizontalPodAutoscalerStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::autoscaling::v2::HorizontalPodAutoscalerStatus {
    fn into_optioned(self) -> HorizontalPodAutoscalerStatusAc {
        HorizontalPodAutoscalerStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_metrics: crate::OptionableConvert::into_optioned(
                self.current_metrics,
            ),
            current_replicas: crate::OptionableConvert::into_optioned(
                self.current_replicas,
            ),
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
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_metrics: crate::OptionableConvert::try_from_optioned(
                value.current_metrics,
            )?,
            current_replicas: crate::OptionableConvert::try_from_optioned(
                value.current_replicas,
            )?,
            desired_replicas: value
                .desired_replicas
                .ok_or(crate::Error {
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
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.current_metrics,
            other.current_metrics,
        )?;
        crate::OptionableConvert::merge(
            &mut self.current_replicas,
            other.current_replicas,
        )?;
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
> for HorizontalPodAutoscalerStatusAc {
    fn from_optionable(
        value: k8s_openapi026::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
