pub struct HorizontalPodAutoscalerOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscalerStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscaler {
    type Optioned = HorizontalPodAutoscalerOpt;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerOpt {
    type Optioned = HorizontalPodAutoscalerOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::HorizontalPodAutoscaler {
    fn into_optioned(self) -> HorizontalPodAutoscalerOpt {
        HorizontalPodAutoscalerOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
