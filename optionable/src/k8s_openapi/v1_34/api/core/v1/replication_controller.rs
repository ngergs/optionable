#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplicationControllerAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::core::v1::ReplicationControllerSpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::core::v1::ReplicationControllerStatus,
    > as crate::Optionable>::Optioned,
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
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ReplicationController {
    type Optioned = ReplicationControllerAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicationControllerAc {
    type Optioned = ReplicationControllerAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ReplicationController {
    fn into_optioned(self) -> ReplicationControllerAc {
        ReplicationControllerAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
            api_version: Default::default(),
            kind: Default::default(),
        }
    }
    fn try_from_optioned(value: ReplicationControllerAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: ReplicationControllerAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::ReplicationController>
for ReplicationControllerAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::ReplicationController,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::ReplicationController, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ReplicationController,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for ReplicationControllerAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::core::v1::ReplicationController as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::core::v1::ReplicationController as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::core::v1::ReplicationController as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::core::v1::ReplicationController as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::core::v1::ReplicationController as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::core::v1::ReplicationController as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ReplicationControllerAc {
    type Ty = <::k8s_openapi::api::core::v1::ReplicationController as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_replicationcontrollerac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::api::core::v1::ReplicationController,
    >();
}
