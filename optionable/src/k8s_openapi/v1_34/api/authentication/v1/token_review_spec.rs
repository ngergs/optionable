pub struct TokenReviewSpecOpt {
    pub audiences: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub token: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::TokenReviewSpec {
    type Optioned = TokenReviewSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for TokenReviewSpecOpt {
    type Optioned = TokenReviewSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::TokenReviewSpec {
    fn into_optioned(self) -> TokenReviewSpecOpt {
        TokenReviewSpecOpt {
            audiences: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.audiences),
            token: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.token),
        }
    }
    fn try_from_optioned(
        value: TokenReviewSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            audiences: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.audiences)?,
            token: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.token)?,
        })
    }
    fn merge(
        &mut self,
        other: TokenReviewSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.audiences, other.audiences)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.token, other.token)?;
        Ok(())
    }
}
