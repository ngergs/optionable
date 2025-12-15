#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfSubjectReviewAc {
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
    pub status: <Option<
        ::k8s_openapi::api::authentication::v1beta1::SelfSubjectReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview {
    type Optioned = SelfSubjectReviewAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectReviewAc {
    type Optioned = SelfSubjectReviewAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview {
    fn into_optioned(self) -> SelfSubjectReviewAc {
        SelfSubjectReviewAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: SelfSubjectReviewAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: SelfSubjectReviewAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview,
> for SelfSubjectReviewAc {
    fn from_optionable(
        value: ::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for SelfSubjectReviewAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for SelfSubjectReviewAc {
    type Ty = <::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_selfsubjectreviewac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::api::authentication::v1beta1::SelfSubjectReview,
    >();
}
