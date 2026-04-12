#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TokenRequest contains parameters of a service account token.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TokenRequestAc {
    /// audience is the intended audience of the token in "TokenRequestSpec". It will default to the audiences of kube apiserver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<std::string::String>,
    /// expirationSeconds is the duration of validity of the token in "TokenRequestSpec". It has the same default value of "ExpirationSeconds" in "TokenRequestSpec".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::TokenRequest {
    type Optioned = TokenRequestAc;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestAc {
    type Optioned = TokenRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::storage::v1::TokenRequest {
    fn into_optioned(self) -> TokenRequestAc {
        TokenRequestAc {
            audience: Some(self.audience),
            expiration_seconds: self.expiration_seconds,
        }
    }
    fn try_from_optioned(value: TokenRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            audience: value
                .audience
                .ok_or(crate::Error {
                    missing_field: "audience",
                })?,
            expiration_seconds: value.expiration_seconds,
        })
    }
    fn merge(&mut self, other: TokenRequestAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.audience {
            self.audience = other_value;
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
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::TokenRequest>
for TokenRequestAc {
    fn from_optionable(value: k8s_openapi027::api::storage::v1::TokenRequest) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::TokenRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::TokenRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
