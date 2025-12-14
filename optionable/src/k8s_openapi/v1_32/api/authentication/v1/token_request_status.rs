#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TokenRequestStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
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
    fn try_from_optioned(value: TokenRequestStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expiration_timestamp: crate::OptionableConvert::try_from_optioned(
                value
                    .expiration_timestamp
                    .ok_or(crate::Error {
                        missing_field: "expiration_timestamp",
                    })?,
            )?,
            token: crate::OptionableConvert::try_from_optioned(
                value
                    .token
                    .ok_or(crate::Error {
                        missing_field: "token",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: TokenRequestStatusAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::authentication::v1::TokenRequestStatus>
for TokenRequestStatusAc {
    fn from_optionable(
        value: ::k8s_openapi::api::authentication::v1::TokenRequestStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::authentication::v1::TokenRequestStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::authentication::v1::TokenRequestStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
