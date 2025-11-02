#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_data: <Option<
        std::collections::BTreeMap<std::string::String, ::k8s_openapi::ByteString>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<ConfigMapAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ConfigMap {
    type Optioned = ConfigMapAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapAc {
    type Optioned = ConfigMapAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ConfigMap {
    fn into_optioned(self) -> ConfigMapAc {
        ConfigMapAc {
            binary_data: crate::OptionableConvert::into_optioned(self.binary_data),
            data: crate::OptionableConvert::into_optioned(self.data),
            immutable: crate::OptionableConvert::into_optioned(self.immutable),
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(value: ConfigMapAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            binary_data: crate::OptionableConvert::try_from_optioned(value.binary_data)?,
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            immutable: crate::OptionableConvert::try_from_optioned(value.immutable)?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: ConfigMapAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.binary_data, other.binary_data)?;
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        crate::OptionableConvert::merge(&mut self.immutable, other.immutable)?;
        self.metadata = other.metadata;
        Ok(())
    }
}
impl k8s_openapi::Resource for ConfigMapAc {
    const API_VERSION: &'static str = "v1";
    const GROUP: &'static str = "";
    const KIND: &'static str = "ConfigMap";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "configmaps";
    type Scope = k8s_openapi::NamespaceResourceScope;
}
impl k8s_openapi::Metadata for ConfigMapAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
