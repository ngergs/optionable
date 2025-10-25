pub struct SelfSubjectReviewOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub status: <Option<
        ::k8s_openapi::api::authentication::v1::SelfSubjectReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::SelfSubjectReview {
    type Optioned = SelfSubjectReviewOpt;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectReviewOpt {
    type Optioned = SelfSubjectReviewOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::SelfSubjectReview {
    fn into_optioned(self) -> SelfSubjectReviewOpt {
        SelfSubjectReviewOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            status: <Option<
                ::k8s_openapi::api::authentication::v1::SelfSubjectReviewStatus,
            > as crate::OptionableConvert>::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectReviewOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            status: <Option<
                ::k8s_openapi::api::authentication::v1::SelfSubjectReviewStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectReviewOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::authentication::v1::SelfSubjectReviewStatus,
        > as crate::OptionableConvert>::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
