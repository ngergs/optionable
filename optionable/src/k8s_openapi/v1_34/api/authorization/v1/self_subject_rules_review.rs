pub struct SelfSubjectRulesReviewOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: Option<
        <::k8s_openapi::api::authorization::v1::SelfSubjectRulesReviewSpec as crate::Optionable>::Optioned,
    >,
    pub status: <Option<
        ::k8s_openapi::api::authorization::v1::SubjectRulesReviewStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SelfSubjectRulesReview {
    type Optioned = SelfSubjectRulesReviewOpt;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectRulesReviewOpt {
    type Optioned = SelfSubjectRulesReviewOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SelfSubjectRulesReview {
    fn into_optioned(self) -> SelfSubjectRulesReviewOpt {
        SelfSubjectRulesReviewOpt {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectRulesReviewOpt,
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
        other: SelfSubjectRulesReviewOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
