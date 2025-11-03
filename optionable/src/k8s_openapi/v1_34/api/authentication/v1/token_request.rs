#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenRequestAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::authentication::v1::TokenRequestSpec as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::authentication::v1::TokenRequestStatus,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<TokenRequestAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::TokenRequest {
    type Optioned = TokenRequestAc;
}
#[automatically_derived]
impl crate::Optionable for TokenRequestAc {
    type Optioned = TokenRequestAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::authentication::v1::TokenRequest {
    fn into_optioned(self) -> TokenRequestAc {
        TokenRequestAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
            status: crate::OptionableConvert::into_optioned(self.status),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(
        value: TokenRequestAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: TokenRequestAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for TokenRequestAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::authentication::v1::TokenRequest as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::authentication::v1::TokenRequest as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::authentication::v1::TokenRequest as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::authentication::v1::TokenRequest as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::authentication::v1::TokenRequest as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::authentication::v1::TokenRequest as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for TokenRequestAc {
    type Ty = <::k8s_openapi::api::authentication::v1::TokenRequest as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
