#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TokenRequestSpec contains client provided parameters of a token request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TokenRequestSpecAc {
    /// Audiences are the intendend audiences of the token. A recipient of a token must identify themself with an identifier in the list of audiences of the token, and otherwise should reject the token. A token issued for multiple audiences may be used to authenticate against any of the audiences listed but implies a high degree of trust between the target audiences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<std::vec::Vec<std::string::String>>,
    /// BoundObjectRef is a reference to an object that the token will be bound to. The token will only be valid for as long as the bound object exists. NOTE: The API server's TokenReview endpoint will validate the BoundObjectRef, but other audiences may not. Keep ExpirationSeconds small if you want prompt revocation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_object_ref: Option<
        <::k8s_openapi027::api::authentication::v1::BoundObjectReference as crate::Optionable>::Optioned,
    >,
    /// ExpirationSeconds is the requested duration of validity of the request. The token issuer may return a token with a different validity duration so a client needs to check the 'expiration' field in a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::authentication::v1::TokenRequestSpec {
    type Optioned = TokenRequestSpecAc;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestSpecAc {
    type Optioned = TokenRequestSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authentication::v1::TokenRequestSpec {
    fn into_optioned(self) -> TokenRequestSpecAc {
        TokenRequestSpecAc {
            audiences: Some(self.audiences),
            bound_object_ref: crate::OptionableConvert::into_optioned(
                self.bound_object_ref,
            ),
            expiration_seconds: self.expiration_seconds,
        }
    }
    fn try_from_optioned(value: TokenRequestSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            audiences: value
                .audiences
                .ok_or(crate::Error {
                    missing_field: "audiences",
                })?,
            bound_object_ref: crate::OptionableConvert::try_from_optioned(
                value.bound_object_ref,
            )?,
            expiration_seconds: value.expiration_seconds,
        })
    }
    fn merge(&mut self, other: TokenRequestSpecAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.audiences {
            self.audiences = other_value;
        }
        if self.bound_object_ref.is_none() {
            self.bound_object_ref = other.bound_object_ref;
        }
        if let Some(other_value) = other.bound_object_ref {
            crate::OptionableConvert::merge(&mut self.bound_object_ref, other_value)?;
        }
        if self.expiration_seconds.is_none() {
            self.expiration_seconds = other.expiration_seconds;
        }
        if let Some(other_value) = other.expiration_seconds {
            crate::OptionableConvert::merge(&mut self.expiration_seconds, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::authentication::v1::TokenRequestSpec>
for TokenRequestSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::authentication::v1::TokenRequestSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authentication::v1::TokenRequestSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authentication::v1::TokenRequestSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
