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
    pub kubelet_config_key: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource {
    type Optioned = ConfigMapNodeConfigSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapNodeConfigSourceAc {
    type Optioned = ConfigMapNodeConfigSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource {
    fn into_optioned(self) -> ConfigMapNodeConfigSourceAc {
        ConfigMapNodeConfigSourceAc {
            kubelet_config_key: Some(
                crate::OptionableConvert::into_optioned(self.kubelet_config_key),
            ),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            namespace: Some(crate::OptionableConvert::into_optioned(self.namespace)),
            resource_version: crate::OptionableConvert::into_optioned(
                self.resource_version,
            ),
            uid: crate::OptionableConvert::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: ConfigMapNodeConfigSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            kubelet_config_key: crate::OptionableConvert::try_from_optioned(
                value
                    .kubelet_config_key
                    .ok_or(crate::Error {
                        missing_field: "kubelet_config_key",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace: crate::OptionableConvert::try_from_optioned(
                value
                    .namespace
                    .ok_or(crate::Error {
                        missing_field: "namespace",
                    })?,
            )?,
            resource_version: crate::OptionableConvert::try_from_optioned(
                value.resource_version,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
        })
    }
    fn merge(&mut self, other: ConfigMapNodeConfigSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.kubelet_config_key {
            crate::OptionableConvert::merge(&mut self.kubelet_config_key, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.namespace {
            crate::OptionableConvert::merge(&mut self.namespace, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource>
for ConfigMapNodeConfigSourceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ConfigMapNodeConfigSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
