#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfSubjectReviewStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: <Option<
        ::k8s_openapi026::api::authentication::v1::UserInfo,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::authentication::v1beta1::SelfSubjectReviewStatus {
    type Optioned = SelfSubjectReviewStatusAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectReviewStatusAc {
    type Optioned = SelfSubjectReviewStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::authentication::v1beta1::SelfSubjectReviewStatus {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::authentication::v1beta1::SelfSubjectReviewStatus,
> for SelfSubjectReviewStatusAc {
    fn from_optionable(
        value: k8s_openapi026::api::authentication::v1beta1::SelfSubjectReviewStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::authentication::v1beta1::SelfSubjectReviewStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::authentication::v1beta1::SelfSubjectReviewStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
