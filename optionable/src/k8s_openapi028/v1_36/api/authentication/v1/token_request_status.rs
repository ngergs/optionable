#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TokenRequestStatus is the result of a token request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TokenRequestStatusAc {
    /// expirationTimestamp is the time of expiration of the returned token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<
        <::k8s_openapi028::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// token is the opaque bearer token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::api::authentication::v1::TokenRequestStatus {
    type Optioned = TokenRequestStatusAc;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestStatusAc {
    type Optioned = TokenRequestStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::authentication::v1::TokenRequestStatus {
    fn into_optioned(self) -> TokenRequestStatusAc {
        TokenRequestStatusAc {
            expiration_timestamp: crate::OptionableConvert::into_optioned(
                self.expiration_timestamp,
            ),
            token: self.token,
        }
    }
    fn try_from_optioned(value: TokenRequestStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expiration_timestamp: crate::OptionableConvert::try_from_optioned(
                value.expiration_timestamp,
            )?,
            token: value.token,
        })
    }
    fn merge(&mut self, other: TokenRequestStatusAc) -> Result<(), crate::Error> {
        if self.expiration_timestamp.is_none() {
            self.expiration_timestamp = crate::OptionableConvert::try_from_optioned(
                other.expiration_timestamp,
            )?;
        } else if let Some(self_value) = self.expiration_timestamp.as_mut()
            && let Some(other_value) = other.expiration_timestamp
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.token.is_none() {
            self.token = crate::OptionableConvert::try_from_optioned(other.token)?;
        } else if let Some(self_value) = self.token.as_mut()
            && let Some(other_value) = other.token
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::api::authentication::v1::TokenRequestStatus>
for TokenRequestStatusAc {
    fn from_optionable(
        value: k8s_openapi028::api::authentication::v1::TokenRequestStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::authentication::v1::TokenRequestStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::authentication::v1::TokenRequestStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for TokenRequestStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(
            &mut self.expiration_timestamp,
            other.expiration_timestamp,
        );
        k8s_openapi028::DeepMerge::merge_from(&mut self.token, other.token);
    }
}
