#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfSubjectAccessReviewAc {
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
    pub spec: Option<
        <::k8s_openapi027::api::authorization::v1::SelfSubjectAccessReviewSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi027::api::authorization::v1::SubjectAccessReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview {
    type Optioned = SelfSubjectAccessReviewAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectAccessReviewAc {
    type Optioned = SelfSubjectAccessReviewAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview {
    fn into_optioned(self) -> SelfSubjectAccessReviewAc {
        SelfSubjectAccessReviewAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectAccessReviewAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: SelfSubjectAccessReviewAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview,
> for SelfSubjectAccessReviewAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for SelfSubjectAccessReviewAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for SelfSubjectAccessReviewAc {
    type Ty = <k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_selfsubjectaccessreviewac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::authorization::v1::SelfSubjectAccessReview,
    >();
}
