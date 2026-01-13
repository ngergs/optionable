#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigMapAc {
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
    pub binary_data: <Option<
        std::collections::BTreeMap<std::string::String, ::k8s_openapi027::ByteString>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ConfigMap {
    type Optioned = ConfigMapAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapAc {
    type Optioned = ConfigMapAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ConfigMap {
    fn into_optioned(self) -> ConfigMapAc {
        ConfigMapAc {
            api_version: Default::default(),
            kind: Default::default(),
            binary_data: crate::OptionableConvert::into_optioned(self.binary_data),
            data: crate::OptionableConvert::into_optioned(self.data),
            immutable: crate::OptionableConvert::into_optioned(self.immutable),
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(value: ConfigMapAc) -> Result<Self, crate::Error> {
        Ok(Self {
            binary_data: crate::OptionableConvert::try_from_optioned(value.binary_data)?,
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            immutable: crate::OptionableConvert::try_from_optioned(value.immutable)?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: ConfigMapAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.binary_data, other.binary_data)?;
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        crate::OptionableConvert::merge(&mut self.immutable, other.immutable)?;
        self.metadata = other.metadata;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ConfigMap> for ConfigMapAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ConfigMap) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ConfigMap, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ConfigMap,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for ConfigMapAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::core::v1::ConfigMap as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::core::v1::ConfigMap as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::core::v1::ConfigMap as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::core::v1::ConfigMap as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::core::v1::ConfigMap as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::core::v1::ConfigMap as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for ConfigMapAc {
    type Ty = <k8s_openapi027::api::core::v1::ConfigMap as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_configmapac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::core::v1::ConfigMap>();
}
