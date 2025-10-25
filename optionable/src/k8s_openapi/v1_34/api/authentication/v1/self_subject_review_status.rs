pub struct SelfSubjectReviewStatusOpt {
    pub user_info: <Option<
        ::k8s_openapi::api::authentication::v1::UserInfo,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authentication::v1::self_subject_review_status::SelfSubjectReviewStatus {
    type Optioned = SelfSubjectReviewStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectReviewStatusOpt {
    type Optioned = SelfSubjectReviewStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::self_subject_review_status::SelfSubjectReviewStatus {
    fn into_optioned(self) -> SelfSubjectReviewStatusOpt {
        SelfSubjectReviewStatusOpt {
            user_info: <Option<
                ::k8s_openapi::api::authentication::v1::UserInfo,
            > as crate::OptionableConvert>::into_optioned(self.user_info),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectReviewStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            user_info: <Option<
                ::k8s_openapi::api::authentication::v1::UserInfo,
            > as crate::OptionableConvert>::try_from_optioned(value.user_info)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectReviewStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::authentication::v1::UserInfo,
        > as crate::OptionableConvert>::merge(&mut self.user_info, other.user_info)?;
        Ok(())
    }
}
