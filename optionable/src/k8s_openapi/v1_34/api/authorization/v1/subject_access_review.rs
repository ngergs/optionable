#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = SubjectAccessReview)]
pub struct SubjectAccessReviewAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::authorization::v1::SubjectAccessReviewSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authorization::v1::SubjectAccessReview {
    type Optioned = SubjectAccessReviewAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectAccessReviewAc {
    type Optioned = SubjectAccessReviewAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SubjectAccessReview {
    fn into_optioned(self) -> SubjectAccessReviewAc {
        SubjectAccessReviewAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: SubjectAccessReviewAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: SubjectAccessReviewAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::authorization::v1::SubjectAccessReview;
