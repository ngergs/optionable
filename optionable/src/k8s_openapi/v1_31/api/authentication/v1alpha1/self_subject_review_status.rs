#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectReviewStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: <Option<
        ::k8s_openapi::api::authentication::v1::UserInfo,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authentication::v1alpha1::SelfSubjectReviewStatus {
    type Optioned = SelfSubjectReviewStatusAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectReviewStatusAc {
    type Optioned = SelfSubjectReviewStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1alpha1::SelfSubjectReviewStatus {
    fn into_optioned(self) -> SelfSubjectReviewStatusAc {
        SelfSubjectReviewStatusAc {
            user_info: crate::OptionableConvert::into_optioned(self.user_info),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectReviewStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            user_info: crate::OptionableConvert::try_from_optioned(value.user_info)?,
        })
    }
    fn merge(&mut self, other: SelfSubjectReviewStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.user_info, other.user_info)?;
        Ok(())
    }
}
