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
            audiences: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.audiences),
            authenticated: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.authenticated),
            error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.error),
            user: <Option<
                ::k8s_openapi::api::authentication::v1::UserInfo,
            > as crate::OptionableConvert>::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: TokenReviewStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            audiences: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.audiences)?,
            authenticated: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.authenticated)?,
            error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.error)?,
            user: <Option<
                ::k8s_openapi::api::authentication::v1::UserInfo,
            > as crate::OptionableConvert>::try_from_optioned(value.user)?,
        })
    }
    fn merge(
        &mut self,
        other: TokenReviewStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.audiences, other.audiences)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.authenticated,
            other.authenticated,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.error, other.error)?;
        <Option<
            ::k8s_openapi::api::authentication::v1::UserInfo,
        > as crate::OptionableConvert>::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
