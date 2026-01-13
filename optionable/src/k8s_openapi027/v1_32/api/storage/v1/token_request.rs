#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TokenRequestAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: <Option<i64> as crate::Optionable>::Optioned,
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
            audience: Some(crate::OptionableConvert::into_optioned(self.audience)),
            expiration_seconds: crate::OptionableConvert::into_optioned(
                self.expiration_seconds,
            ),
        }
    }
    fn try_from_optioned(value: TokenRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            audience: crate::OptionableConvert::try_from_optioned(
                value
                    .audience
                    .ok_or(crate::Error {
                        missing_field: "audience",
                    })?,
            )?,
            expiration_seconds: crate::OptionableConvert::try_from_optioned(
                value.expiration_seconds,
            )?,
        })
    }
    fn merge(&mut self, other: TokenRequestAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.audience {
            crate::OptionableConvert::merge(&mut self.audience, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.expiration_seconds,
            other.expiration_seconds,
        )?;
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
