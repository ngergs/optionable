#[derive(kube::Resource)]
#[resource(inherit = HorizontalPodAutoscaler)]
pub struct HorizontalPodAutoscalerAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscalerSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscalerStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler {
    type Optioned = HorizontalPodAutoscalerAc;
}
#[automatically_derived]
impl crate::Optionable for HorizontalPodAutoscalerAc {
    type Optioned = HorizontalPodAutoscalerAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler {
    fn into_optioned(self) -> HorizontalPodAutoscalerAc {
        HorizontalPodAutoscalerAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: HorizontalPodAutoscalerAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler;
