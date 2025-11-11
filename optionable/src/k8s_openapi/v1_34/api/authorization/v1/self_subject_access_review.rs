#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectAccessReviewAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        deserialize_with = "crate::k8s_openapi::deserialize_api_envelope"
    )]
    pub phantom: std::marker::PhantomData<SelfSubjectAccessReviewAc>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview {
    type Optioned = SelfSubjectAccessReviewAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectAccessReviewAc {
    type Optioned = SelfSubjectAccessReviewAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview {
    fn into_optioned(self) -> SelfSubjectAccessReviewAc {
        SelfSubjectAccessReviewAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
            phantom: Default::default(),
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
impl k8s_openapi::Resource for SelfSubjectAccessReviewAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for SelfSubjectAccessReviewAc {
    type Ty = <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReview as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
