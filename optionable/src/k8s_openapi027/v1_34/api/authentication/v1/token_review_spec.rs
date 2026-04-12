#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TokenReviewSpec is a description of the token authentication request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TokenReviewSpecAc {
    /// Audiences is a list of the identifiers that the resource server presented with the token identifies as. Audience-aware token authenticators will verify that the token was intended for at least one of the audiences in this list. If no audiences are provided, the audience will default to the audience of the Kubernetes apiserver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<std::vec::Vec<std::string::String>>,
    /// Token is the opaque bearer token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::authentication::v1::TokenReviewSpec {
    type Optioned = TokenReviewSpecAc;
}
#[automatically_derived]
impl crate::Optionable for TokenReviewSpecAc {
    type Optioned = TokenReviewSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authentication::v1::TokenReviewSpec {
    fn into_optioned(self) -> TokenReviewSpecAc {
        TokenReviewSpecAc {
            audiences: self.audiences,
            token: self.token,
        }
    }
    fn try_from_optioned(value: TokenReviewSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            audiences: value.audiences,
            token: value.token,
        })
    }
    fn merge(&mut self, other: TokenReviewSpecAc) -> Result<(), crate::Error> {
        if self.audiences.is_none() {
            self.audiences = other.audiences;
        }
        if let Some(other_value) = other.audiences {
            self.audiences = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.token.is_none() {
            self.token = other.token;
        }
        if let Some(other_value) = other.token {
            crate::OptionableConvert::merge(&mut self.token, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::authentication::v1::TokenReviewSpec>
for TokenReviewSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::authentication::v1::TokenReviewSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::authentication::v1::TokenReviewSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authentication::v1::TokenReviewSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
