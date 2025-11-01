pub struct TokenReviewSpecAc {
    pub audiences: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub token: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::TokenReviewSpec {
    type Optioned = TokenReviewSpecAc;
}
#[automatically_derived]
impl crate::Optionable for TokenReviewSpecAc {
    type Optioned = TokenReviewSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::TokenReviewSpec {
    fn into_optioned(self) -> TokenReviewSpecAc {
        TokenReviewSpecAc {
            audiences: crate::OptionableConvert::into_optioned(self.audiences),
            token: crate::OptionableConvert::into_optioned(self.token),
        }
    }
    fn try_from_optioned(
        value: TokenReviewSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            audiences: crate::OptionableConvert::try_from_optioned(value.audiences)?,
            token: crate::OptionableConvert::try_from_optioned(value.token)?,
        })
    }
    fn merge(
        &mut self,
        other: TokenReviewSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.audiences, other.audiences)?;
        crate::OptionableConvert::merge(&mut self.token, other.token)?;
        Ok(())
    }
}
