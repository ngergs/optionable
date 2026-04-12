#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ConfigMapNodeConfigSource contains the information to reference a ConfigMap as a config source for the Node. This API is deprecated since 1.22: https://git.k8s.io/enhancements/keps/sig-node/281-dynamic-kubelet-configuration
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigMapNodeConfigSourceAc {
    /// KubeletConfigKey declares which key of the referenced ConfigMap corresponds to the KubeletConfiguration structure This field is required in all cases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_config_key: Option<std::string::String>,
    /// Name is the metadata.name of the referenced ConfigMap. This field is required in all cases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Namespace is the metadata.namespace of the referenced ConfigMap. This field is required in all cases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    /// ResourceVersion is the metadata.ResourceVersion of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
    /// UID is the metadata.UID of the referenced ConfigMap. This field is forbidden in Node.Spec, and required in Node.Status.
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
            self.kubelet_config_key = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.namespace {
            self.namespace = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.resource_version.is_none() {
            self.resource_version = crate::OptionableConvert::try_from_optioned(
                other.resource_version,
            )?;
        } else if let Some(self_value) = self.resource_version.as_mut()
            && let Some(other_value) = other.resource_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.uid.is_none() {
            self.uid = crate::OptionableConvert::try_from_optioned(other.uid)?;
        } else if let Some(self_value) = self.uid.as_mut()
            && let Some(other_value) = other.uid
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
