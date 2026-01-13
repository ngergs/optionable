#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: <Option<
        std::collections::BTreeMap<std::string::String, ::k8s_openapi027::ByteString>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_data: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Secret {
    type Optioned = SecretAc;
}
#[automatically_derived]
impl crate::Optionable for SecretAc {
    type Optioned = SecretAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Secret {
    fn into_optioned(self) -> SecretAc {
        SecretAc {
            api_version: Default::default(),
            kind: Default::default(),
            data: crate::OptionableConvert::into_optioned(self.data),
            immutable: crate::OptionableConvert::into_optioned(self.immutable),
            metadata: self.metadata,
            string_data: crate::OptionableConvert::into_optioned(self.string_data),
            type_: crate::OptionableConvert::into_optioned(self.type_),
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Secret> for SecretAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Secret) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Secret, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Secret,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for SecretAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for SecretAc {
    type Ty = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_secretac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::core::v1::Secret>();
}
