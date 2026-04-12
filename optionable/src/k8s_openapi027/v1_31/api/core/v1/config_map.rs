#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ConfigMap holds configuration data for pods to consume.
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
    /// BinaryData contains the binary data. Each key must consist of alphanumeric characters, '-', '_' or '.'. BinaryData can contain byte sequences that are not in the UTF-8 range. The keys stored in BinaryData must not overlap with the ones in the Data field, this is enforced during validation process. Using this field will require 1.10+ apiserver and kubelet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_data: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::ByteString as crate::Optionable>::Optioned,
        >,
    >,
    /// Data contains the configuration data. Each key must consist of alphanumeric characters, '-', '_' or '.'. Values with non-UTF-8 byte sequences must use the BinaryData field. The keys stored in Data must not overlap with the keys in the BinaryData field, this is enforced during validation process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// Immutable, if set to true, ensures that data stored in the ConfigMap cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
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
            data: self.data,
            immutable: self.immutable,
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(value: ConfigMapAc) -> Result<Self, crate::Error> {
        Ok(Self {
            binary_data: crate::OptionableConvert::try_from_optioned(value.binary_data)?,
            data: value.data,
            immutable: value.immutable,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: ConfigMapAc) -> Result<(), crate::Error> {
        if self.binary_data.is_none() {
            self.binary_data = other.binary_data;
        }
        if let Some(other_value) = other.binary_data {
            crate::OptionableConvert::merge(&mut self.binary_data, other_value)?;
        }
        if self.data.is_none() {
            self.data = other.data;
        }
        if let Some(other_value) = other.data {
            crate::OptionableConvert::merge(&mut self.data, other_value)?;
        }
        if self.immutable.is_none() {
            self.immutable = other.immutable;
        }
        if let Some(other_value) = other.immutable {
            crate::OptionableConvert::merge(&mut self.immutable, other_value)?;
        }
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
