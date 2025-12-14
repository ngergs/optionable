#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimParametersAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_requests: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha2::DriverRequests>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_from: <Option<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParametersReference,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shareable: <Option<bool> as crate::Optionable>::Optioned,
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
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters {
    type Optioned = ResourceClaimParametersAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimParametersAc {
    type Optioned = ResourceClaimParametersAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters {
    fn into_optioned(self) -> ResourceClaimParametersAc {
        ResourceClaimParametersAc {
            driver_requests: crate::OptionableConvert::into_optioned(
                self.driver_requests,
            ),
            generated_from: crate::OptionableConvert::into_optioned(self.generated_from),
            metadata: self.metadata,
            shareable: crate::OptionableConvert::into_optioned(self.shareable),
            api_version: Default::default(),
            kind: Default::default(),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimParametersAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            driver_requests: crate::OptionableConvert::try_from_optioned(
                value.driver_requests,
            )?,
            generated_from: crate::OptionableConvert::try_from_optioned(
                value.generated_from,
            )?,
            metadata: value.metadata,
            shareable: crate::OptionableConvert::try_from_optioned(value.shareable)?,
        })
    }
    fn merge(&mut self, other: ResourceClaimParametersAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.driver_requests,
            other.driver_requests,
        )?;
        crate::OptionableConvert::merge(&mut self.generated_from, other.generated_from)?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.shareable, other.shareable)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters,
> for ResourceClaimParametersAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for ResourceClaimParametersAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ResourceClaimParametersAc {
    type Ty = <::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_resourceclaimparametersac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::api::resource::v1alpha2::ResourceClaimParameters,
    >();
}
