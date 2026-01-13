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
        serialize_with = "crate::k8s_openapi027::serialize_api_version",
        deserialize_with = "crate::k8s_openapi027::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi027::serialize_kind",
        deserialize_with = "crate::k8s_openapi027::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi027::api::authentication::v1::SelfSubjectReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::authentication::v1::SelfSubjectReview {
    type Optioned = SelfSubjectReviewAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectReviewAc {
    type Optioned = SelfSubjectReviewAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authentication::v1::SelfSubjectReview {
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
impl crate::OptionedConvert<k8s_openapi027::api::authentication::v1::SelfSubjectReview>
for SelfSubjectReviewAc {
    fn from_optionable(
        value: k8s_openapi027::api::authentication::v1::SelfSubjectReview,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authentication::v1::SelfSubjectReview,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authentication::v1::SelfSubjectReview,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for SelfSubjectReviewAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::authentication::v1::SelfSubjectReview as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::authentication::v1::SelfSubjectReview as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::authentication::v1::SelfSubjectReview as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::authentication::v1::SelfSubjectReview as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::authentication::v1::SelfSubjectReview as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::authentication::v1::SelfSubjectReview as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for SelfSubjectReviewAc {
    type Ty = <k8s_openapi027::api::authentication::v1::SelfSubjectReview as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_selfsubjectreviewac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::authentication::v1::SelfSubjectReview,
    >();
}
