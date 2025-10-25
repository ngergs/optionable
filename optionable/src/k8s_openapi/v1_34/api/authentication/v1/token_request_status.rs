pub struct TokenRequestStatusOpt {
    pub expiration_timestamp: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    pub token: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authentication::v1::token_request_status::TokenRequestStatus {
    type Optioned = TokenRequestStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestStatusOpt {
    type Optioned = TokenRequestStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::token_request_status::TokenRequestStatus {
    fn into_optioned(self) -> TokenRequestStatusOpt {
        TokenRequestStatusOpt {
            expiration_timestamp: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time as crate::OptionableConvert>::into_optioned(
                    self.expiration_timestamp,
                ),
            ),
            token: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.token,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: TokenRequestStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expiration_timestamp: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time as crate::OptionableConvert>::try_from_optioned(
                value
                    .expiration_timestamp
                    .ok_or(crate::optionable::Error {
                        missing_field: "expiration_timestamp",
                    })?,
            )?,
            token: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .token
                    .ok_or(crate::optionable::Error {
                        missing_field: "token",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: TokenRequestStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.expiration_timestamp {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time as crate::OptionableConvert>::merge(
                &mut self.expiration_timestamp,
                other_value,
            )?;
        }
        if let Some(other_value) = other.token {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.token,
                other_value,
            )?;
        }
        Ok(())
    }
}
