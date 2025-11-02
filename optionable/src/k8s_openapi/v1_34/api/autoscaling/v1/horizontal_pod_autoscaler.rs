#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HorizontalPodAutoscalerAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscalerSpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
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
impl k8s_openapi::Resource for HorizontalPodAutoscalerAc {
    const API_VERSION: &'static str = "autoscaling/v1";
    const GROUP: &'static str = "autoscaling";
    const KIND: &'static str = "HorizontalPodAutoscaler";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "horizontalpodautoscalers";
    type Scope = k8s_openapi::NamespaceResourceScope;
}
impl k8s_openapi::Metadata for HorizontalPodAutoscalerAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
