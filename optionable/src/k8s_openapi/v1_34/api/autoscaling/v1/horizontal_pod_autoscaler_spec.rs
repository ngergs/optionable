pub struct HorizontalPodAutoscalerSpecOpt {
    pub max_replicas: Option<i32>,
    pub min_replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub scale_target_ref: Option<
        <::k8s_openapi::api::autoscaling::v1::CrossVersionObjectReference as crate::Optionable>::Optioned,
    >,
    pub target_cpu_utilization_percentage: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscalerSpec {
    type Optioned = HorizontalPodAutoscalerSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerSpecOpt {
    type Optioned = HorizontalPodAutoscalerSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscalerSpec {
    fn into_optioned(self) -> HorizontalPodAutoscalerSpecOpt {
        HorizontalPodAutoscalerSpecOpt {
            max_replicas: Some(self.max_replicas),
            min_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.min_replicas),
            scale_target_ref: Some(
                <::k8s_openapi::api::autoscaling::v1::CrossVersionObjectReference as crate::OptionableConvert>::into_optioned(
                    self.scale_target_ref,
                ),
            ),
            target_cpu_utilization_percentage: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(
                self.target_cpu_utilization_percentage,
            ),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            max_replicas: value
                .max_replicas
                .ok_or(crate::optionable::Error {
                    missing_field: "max_replicas",
                })?,
            min_replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.min_replicas)?,
            scale_target_ref: <::k8s_openapi::api::autoscaling::v1::CrossVersionObjectReference as crate::OptionableConvert>::try_from_optioned(
                value
                    .scale_target_ref
                    .ok_or(crate::optionable::Error {
                        missing_field: "scale_target_ref",
                    })?,
            )?,
            target_cpu_utilization_percentage: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.target_cpu_utilization_percentage,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.max_replicas {
            self.max_replicas = other_value;
        }
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.min_replicas,
            other.min_replicas,
        )?;
        if let Some(other_value) = other.scale_target_ref {
            <::k8s_openapi::api::autoscaling::v1::CrossVersionObjectReference as crate::OptionableConvert>::merge(
                &mut self.scale_target_ref,
                other_value,
            )?;
        }
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.target_cpu_utilization_percentage,
            other.target_cpu_utilization_percentage,
        )?;
        Ok(())
    }
}
