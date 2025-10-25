pub struct TokenRequestOpt {
    pub audience: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub expiration_seconds: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::TokenRequest {
    type Optioned = TokenRequestOpt;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestOpt {
    type Optioned = TokenRequestOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::TokenRequest {
    fn into_optioned(self) -> TokenRequestOpt {
        TokenRequestOpt {
            audience: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.audience,
                ),
            ),
            expiration_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.expiration_seconds),
        }
    }
    fn try_from_optioned(
        value: TokenRequestOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            audience: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .audience
                    .ok_or(crate::optionable::Error {
                        missing_field: "audience",
                    })?,
            )?,
            expiration_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.expiration_seconds)?,
        })
    }
    fn merge(&mut self, other: TokenRequestOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.audience {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.audience,
                other_value,
            )?;
        }
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.expiration_seconds,
            other.expiration_seconds,
        )?;
        Ok(())
    }
}
