#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct EndpointsAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subsets: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EndpointSubset>,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<EndpointsAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Endpoints {
    type Optioned = EndpointsAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointsAc {
    type Optioned = EndpointsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Endpoints {
    fn into_optioned(self) -> EndpointsAc {
        EndpointsAc {
            metadata: self.metadata,
            subsets: crate::OptionableConvert::into_optioned(self.subsets),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: EndpointsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            subsets: crate::OptionableConvert::try_from_optioned(value.subsets)?,
        })
    }
    fn merge(&mut self, other: EndpointsAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.subsets, other.subsets)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for EndpointsAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::core::v1::Endpoints as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::core::v1::Endpoints as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::core::v1::Endpoints as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::core::v1::Endpoints as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::core::v1::Endpoints as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::core::v1::Endpoints as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for EndpointsAc {
    type Ty = <::k8s_openapi::api::core::v1::Endpoints as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
