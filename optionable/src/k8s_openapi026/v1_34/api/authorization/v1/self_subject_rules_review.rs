#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfSubjectRulesReviewAc {
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
    pub metadata: ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi026::api::authorization::v1::SelfSubjectRulesReviewSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi026::api::authorization::v1::SubjectRulesReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview {
    type Optioned = SelfSubjectRulesReviewAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectRulesReviewAc {
    type Optioned = SelfSubjectRulesReviewAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview {
    fn into_optioned(self) -> SelfSubjectRulesReviewAc {
        SelfSubjectRulesReviewAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: SelfSubjectRulesReviewAc) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: SelfSubjectRulesReviewAc) -> Result<(), crate::Error> {
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
    k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview,
> for SelfSubjectRulesReviewAc {
    fn from_optionable(
        value: k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for SelfSubjectRulesReviewAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for SelfSubjectRulesReviewAc {
    type Ty = <k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_selfsubjectrulesreviewac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi026::api::authorization::v1::SelfSubjectRulesReview,
    >();
}
