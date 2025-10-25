pub struct TokenRequestSpecOpt {
    pub audiences: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub bound_object_ref: <Option<
        ::k8s_openapi::api::authentication::v1::BoundObjectReference,
    > as crate::Optionable>::Optioned,
    pub expiration_seconds: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::TokenRequestSpec {
    type Optioned = TokenRequestSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestSpecOpt {
    type Optioned = TokenRequestSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::TokenRequestSpec {
    fn into_optioned(self) -> TokenRequestSpecOpt {
        TokenRequestSpecOpt {
            audiences: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.audiences),
            ),
            bound_object_ref: <Option<
                ::k8s_openapi::api::authentication::v1::BoundObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.bound_object_ref),
            expiration_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.expiration_seconds),
        }
    }
    fn try_from_optioned(
        value: TokenRequestSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            audiences: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .audiences
                    .ok_or(crate::optionable::Error {
                        missing_field: "audiences",
                    })?,
            )?,
            bound_object_ref: <Option<
                ::k8s_openapi::api::authentication::v1::BoundObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.bound_object_ref)?,
            expiration_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.expiration_seconds)?,
        })
    }
    fn merge(
        &mut self,
        other: TokenRequestSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.audiences {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.audiences, other_value)?;
        }
        <Option<
            ::k8s_openapi::api::authentication::v1::BoundObjectReference,
        > as crate::OptionableConvert>::merge(
            &mut self.bound_object_ref,
            other.bound_object_ref,
        )?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.expiration_seconds,
            other.expiration_seconds,
        )?;
        Ok(())
    }
}
