#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct SecretAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: <Option<
        std::collections::BTreeMap<std::string::String, ::k8s_openapi::ByteString>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_data: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<SecretAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Secret {
    type Optioned = SecretAc;
}
#[automatically_derived]
impl crate::Optionable for SecretAc {
    type Optioned = SecretAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Secret {
    fn into_optioned(self) -> SecretAc {
        SecretAc {
            data: crate::OptionableConvert::into_optioned(self.data),
            immutable: crate::OptionableConvert::into_optioned(self.immutable),
            metadata: self.metadata,
            string_data: crate::OptionableConvert::into_optioned(self.string_data),
            type_: crate::OptionableConvert::into_optioned(self.type_),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: SecretAc) -> Result<Self, crate::Error> {
        Ok(Self {
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            immutable: crate::OptionableConvert::try_from_optioned(value.immutable)?,
            metadata: value.metadata,
            string_data: crate::OptionableConvert::try_from_optioned(value.string_data)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: SecretAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        crate::OptionableConvert::merge(&mut self.immutable, other.immutable)?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.string_data, other.string_data)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for SecretAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::core::v1::Secret as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::core::v1::Secret as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::core::v1::Secret as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::core::v1::Secret as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::core::v1::Secret as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::core::v1::Secret as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for SecretAc {
    type Ty = <::k8s_openapi::api::core::v1::Secret as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
