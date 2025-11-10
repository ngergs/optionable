#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct TokenReviewStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticated: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: <Option<
        ::k8s_openapi::api::authentication::v1::UserInfo,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::TokenReviewStatus {
    type Optioned = TokenReviewStatusAc;
}
#[automatically_derived]
impl crate::Optionable for TokenReviewStatusAc {
    type Optioned = TokenReviewStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::TokenReviewStatus {
    fn into_optioned(self) -> TokenReviewStatusAc {
        TokenReviewStatusAc {
            audiences: crate::OptionableConvert::into_optioned(self.audiences),
            authenticated: crate::OptionableConvert::into_optioned(self.authenticated),
            error: crate::OptionableConvert::into_optioned(self.error),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(value: TokenReviewStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            audiences: crate::OptionableConvert::try_from_optioned(value.audiences)?,
            authenticated: crate::OptionableConvert::try_from_optioned(
                value.authenticated,
            )?,
            error: crate::OptionableConvert::try_from_optioned(value.error)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(&mut self, other: TokenReviewStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.audiences, other.audiences)?;
        crate::OptionableConvert::merge(&mut self.authenticated, other.authenticated)?;
        crate::OptionableConvert::merge(&mut self.error, other.error)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
