pub struct SelfSubjectAccessReviewOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: Option<
        <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec as crate::Optionable>::Optioned,
    >,
    pub status: <Option<
        ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::self_subject_access_review::SelfSubjectAccessReview {
    type Optioned = SelfSubjectAccessReviewOpt;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectAccessReviewOpt {
    type Optioned = SelfSubjectAccessReviewOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::self_subject_access_review::SelfSubjectAccessReview {
    fn into_optioned(self) -> SelfSubjectAccessReviewOpt {
        SelfSubjectAccessReviewOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            spec: Some(
                <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec as crate::OptionableConvert>::into_optioned(
                    self.spec,
                ),
            ),
            status: <Option<
                ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus,
            > as crate::OptionableConvert>::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectAccessReviewOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec as crate::OptionableConvert>::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: <Option<
                ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectAccessReviewOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        if let Some(other_value) = other.spec {
            <::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec as crate::OptionableConvert>::merge(
                &mut self.spec,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus,
        > as crate::OptionableConvert>::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
