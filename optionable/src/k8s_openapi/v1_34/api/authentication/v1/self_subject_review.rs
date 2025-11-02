#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SelfSubjectReviewAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::authentication::v1::SelfSubjectReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::SelfSubjectReview {
    type Optioned = SelfSubjectReviewAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectReviewAc {
    type Optioned = SelfSubjectReviewAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::SelfSubjectReview {
    fn into_optioned(self) -> SelfSubjectReviewAc {
        SelfSubjectReviewAc {
            metadata: self.metadata,
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectReviewAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectReviewAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for SelfSubjectReviewAc {
    const API_VERSION: &'static str = "authentication.k8s.io/v1";
    const GROUP: &'static str = "authentication.k8s.io";
    const KIND: &'static str = "SelfSubjectReview";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "selfsubjectreviews";
    type Scope = k8s_openapi::ClusterResourceScope;
}
impl k8s_openapi::Metadata for SelfSubjectReviewAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
