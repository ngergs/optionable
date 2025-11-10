#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct EvictionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_options: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<EvictionAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::policy::v1::Eviction {
    type Optioned = EvictionAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionAc {
    type Optioned = EvictionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::policy::v1::Eviction {
    fn into_optioned(self) -> EvictionAc {
        EvictionAc {
            delete_options: crate::OptionableConvert::into_optioned(self.delete_options),
            metadata: self.metadata,
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: EvictionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            delete_options: crate::OptionableConvert::try_from_optioned(
                value.delete_options,
            )?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: EvictionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.delete_options, other.delete_options)?;
        self.metadata = other.metadata;
        Ok(())
    }
}
impl k8s_openapi::Resource for EvictionAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::policy::v1::Eviction as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::policy::v1::Eviction as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::policy::v1::Eviction as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::policy::v1::Eviction as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::policy::v1::Eviction as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::policy::v1::Eviction as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for EvictionAc {
    type Ty = <::k8s_openapi::api::policy::v1::Eviction as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
