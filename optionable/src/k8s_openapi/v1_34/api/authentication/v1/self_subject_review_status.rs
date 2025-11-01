pub struct SelfSubjectReviewStatusAc {
    pub user_info: <Option<
        ::k8s_openapi::api::authentication::v1::UserInfo,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authentication::v1::SelfSubjectReviewStatus {
    type Optioned = SelfSubjectReviewStatusAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectReviewStatusAc {
    type Optioned = SelfSubjectReviewStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::SelfSubjectReviewStatus {
    fn into_optioned(self) -> SelfSubjectReviewStatusAc {
        SelfSubjectReviewStatusAc {
            user_info: crate::OptionableConvert::into_optioned(self.user_info),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectReviewStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            user_info: crate::OptionableConvert::try_from_optioned(value.user_info)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectReviewStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.user_info, other.user_info)?;
        Ok(())
    }
}
