#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TokenRequestSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_object_ref: <Option<
        ::k8s_openapi::api::authentication::v1::BoundObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_seconds: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::TokenRequestSpec {
    type Optioned = TokenRequestSpecAc;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestSpecAc {
    type Optioned = TokenRequestSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::TokenRequestSpec {
    fn into_optioned(self) -> TokenRequestSpecAc {
        TokenRequestSpecAc {
            audiences: Some(crate::OptionableConvert::into_optioned(self.audiences)),
            bound_object_ref: crate::OptionableConvert::into_optioned(
                self.bound_object_ref,
            ),
            expiration_seconds: crate::OptionableConvert::into_optioned(
                self.expiration_seconds,
            ),
        }
    }
    fn try_from_optioned(value: TokenRequestSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            audiences: crate::OptionableConvert::try_from_optioned(
                value
                    .audiences
                    .ok_or(crate::Error {
                        missing_field: "audiences",
                    })?,
            )?,
            bound_object_ref: crate::OptionableConvert::try_from_optioned(
                value.bound_object_ref,
            )?,
            expiration_seconds: crate::OptionableConvert::try_from_optioned(
                value.expiration_seconds,
            )?,
        })
    }
    fn merge(&mut self, other: TokenRequestSpecAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.audiences {
            crate::OptionableConvert::merge(&mut self.audiences, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.bound_object_ref,
            other.bound_object_ref,
        )?;
        crate::OptionableConvert::merge(
            &mut self.expiration_seconds,
            other.expiration_seconds,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::authentication::v1::TokenRequestSpec>
for TokenRequestSpecAc {
    fn from_optionable(
        value: ::k8s_openapi::api::authentication::v1::TokenRequestSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::authentication::v1::TokenRequestSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::authentication::v1::TokenRequestSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
