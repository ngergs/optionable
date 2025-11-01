#[derive(kube::Resource)]
#[resource(inherit = SelfSubjectReview)]
pub struct SelfSubjectReviewAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
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
#[allow(unused_imports)]
use ::k8s_openapi::api::authentication::v1::SelfSubjectReview;
