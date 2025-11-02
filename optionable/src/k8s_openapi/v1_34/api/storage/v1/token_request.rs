#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TokenRequestAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::TokenRequest {
    type Optioned = TokenRequestAc;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestAc {
    type Optioned = TokenRequestAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::TokenRequest {
    fn into_optioned(self) -> TokenRequestAc {
        TokenRequestAc {
            audience: Some(crate::OptionableConvert::into_optioned(self.audience)),
            expiration_seconds: crate::OptionableConvert::into_optioned(
                self.expiration_seconds,
            ),
        }
    }
    fn try_from_optioned(
        value: TokenRequestAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            audience: crate::OptionableConvert::try_from_optioned(
                value
                    .audience
                    .ok_or(crate::optionable::Error {
                        missing_field: "audience",
                    })?,
            )?,
            expiration_seconds: crate::OptionableConvert::try_from_optioned(
                value.expiration_seconds,
            )?,
        })
    }
    fn merge(&mut self, other: TokenRequestAc) -> Result<(), crate::optionable::Error> {
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
