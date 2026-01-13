#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionAc {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_options: <Option<
        ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::DeleteOptions,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::policy::v1::Eviction {
    type Optioned = EvictionAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionAc {
    type Optioned = EvictionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::policy::v1::Eviction {
    fn into_optioned(self) -> EvictionAc {
        EvictionAc {
            api_version: Default::default(),
            kind: Default::default(),
            delete_options: crate::OptionableConvert::into_optioned(self.delete_options),
            metadata: self.metadata,
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::policy::v1::Eviction> for EvictionAc {
    fn from_optionable(value: k8s_openapi026::api::policy::v1::Eviction) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::policy::v1::Eviction, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::policy::v1::Eviction,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for EvictionAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::policy::v1::Eviction as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::policy::v1::Eviction as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::policy::v1::Eviction as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::policy::v1::Eviction as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::policy::v1::Eviction as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::policy::v1::Eviction as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for EvictionAc {
    type Ty = <k8s_openapi026::api::policy::v1::Eviction as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_evictionac() {
    crate::testutil::roundtrip_test::<k8s_openapi026::api::policy::v1::Eviction>();
}
