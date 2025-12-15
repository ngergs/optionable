#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HorizontalPodAutoscalerAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler {
    fn into_optioned(self) -> HorizontalPodAutoscalerAc {
        HorizontalPodAutoscalerAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: HorizontalPodAutoscalerAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: HorizontalPodAutoscalerAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler>
for HorizontalPodAutoscalerAc {
    fn from_optionable(
        value: ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for HorizontalPodAutoscalerAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for HorizontalPodAutoscalerAc {
    type Ty = <::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_horizontalpodautoscalerac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::api::autoscaling::v1::HorizontalPodAutoscaler,
    >();
}
