#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigMapNodeConfigSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_config_key: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ConfigMapNodeConfigSource {
    type Optioned = ConfigMapNodeConfigSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapNodeConfigSourceAc {
    type Optioned = ConfigMapNodeConfigSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ConfigMapNodeConfigSource {
    fn into_optioned(self) -> ConfigMapNodeConfigSourceAc {
        ConfigMapNodeConfigSourceAc {
            kubelet_config_key: Some(self.kubelet_config_key),
            name: Some(self.name),
            namespace: Some(self.namespace),
            resource_version: self.resource_version,
            uid: self.uid,
        }
    }
    fn try_from_optioned(
        value: ConfigMapNodeConfigSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            kubelet_config_key: value
                .kubelet_config_key
                .ok_or(crate::Error {
                    missing_field: "kubelet_config_key",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            namespace: value
                .namespace
                .ok_or(crate::Error {
                    missing_field: "namespace",
                })?,
            resource_version: value.resource_version,
            uid: value.uid,
        })
    }
    fn merge(&mut self, other: ConfigMapNodeConfigSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.kubelet_config_key {
            self.kubelet_config_key = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.namespace {
            self.namespace = other_value;
        }
        self.resource_version = other.resource_version;
        self.uid = other.uid;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ConfigMapNodeConfigSource>
for ConfigMapNodeConfigSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ConfigMapNodeConfigSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ConfigMapNodeConfigSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ConfigMapNodeConfigSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
