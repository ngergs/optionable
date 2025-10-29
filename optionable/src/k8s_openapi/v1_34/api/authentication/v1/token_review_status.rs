pub struct TokenReviewStatusOpt {
    pub audiences: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub authenticated: <Option<bool> as crate::Optionable>::Optioned,
    pub error: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub user: <Option<
        ::k8s_openapi::api::authentication::v1::UserInfo,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::TokenReviewStatus {
    type Optioned = TokenReviewStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for TokenReviewStatusOpt {
    type Optioned = TokenReviewStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::TokenReviewStatus {
    fn into_optioned(self) -> TokenReviewStatusOpt {
        TokenReviewStatusOpt {
            audiences: crate::OptionableConvert::into_optioned(self.audiences),
            authenticated: crate::OptionableConvert::into_optioned(self.authenticated),
            error: crate::OptionableConvert::into_optioned(self.error),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: TokenReviewStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            audiences: crate::OptionableConvert::try_from_optioned(value.audiences)?,
            authenticated: crate::OptionableConvert::try_from_optioned(
                value.authenticated,
            )?,
            error: crate::OptionableConvert::try_from_optioned(value.error)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(
        &mut self,
        other: TokenReviewStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.audiences, other.audiences)?;
        crate::OptionableConvert::merge(&mut self.authenticated, other.authenticated)?;
        crate::OptionableConvert::merge(&mut self.error, other.error)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
