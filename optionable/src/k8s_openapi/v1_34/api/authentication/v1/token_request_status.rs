pub struct TokenRequestStatusAc {
    pub expiration_timestamp: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    pub token: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::TokenRequestStatus {
    type Optioned = TokenRequestStatusAc;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestStatusAc {
    type Optioned = TokenRequestStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::TokenRequestStatus {
    fn into_optioned(self) -> TokenRequestStatusAc {
        TokenRequestStatusAc {
            expiration_timestamp: Some(
                crate::OptionableConvert::into_optioned(self.expiration_timestamp),
            ),
            token: Some(crate::OptionableConvert::into_optioned(self.token)),
        }
    }
    fn try_from_optioned(
        value: TokenRequestStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expiration_timestamp: crate::OptionableConvert::try_from_optioned(
                value
                    .expiration_timestamp
                    .ok_or(crate::optionable::Error {
                        missing_field: "expiration_timestamp",
                    })?,
            )?,
            token: crate::OptionableConvert::try_from_optioned(
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
        other: TokenRequestStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.expiration_timestamp {
            crate::OptionableConvert::merge(
                &mut self.expiration_timestamp,
                other_value,
            )?;
        }
        if let Some(other_value) = other.token {
            crate::OptionableConvert::merge(&mut self.token, other_value)?;
        }
        Ok(())
    }
}
