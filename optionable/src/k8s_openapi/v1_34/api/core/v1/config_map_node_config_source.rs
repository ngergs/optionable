pub struct ConfigMapNodeConfigSourceOpt {
    pub kubelet_config_key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub namespace: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::config_map_node_config_source::ConfigMapNodeConfigSource {
    type Optioned = ConfigMapNodeConfigSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapNodeConfigSourceOpt {
    type Optioned = ConfigMapNodeConfigSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::config_map_node_config_source::ConfigMapNodeConfigSource {
    fn into_optioned(self) -> ConfigMapNodeConfigSourceOpt {
        ConfigMapNodeConfigSourceOpt {
            kubelet_config_key: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kubelet_config_key,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            namespace: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.namespace,
                ),
            ),
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource_version),
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: ConfigMapNodeConfigSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            kubelet_config_key: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kubelet_config_key
                    .ok_or(crate::optionable::Error {
                        missing_field: "kubelet_config_key",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .namespace
                    .ok_or(crate::optionable::Error {
                        missing_field: "namespace",
                    })?,
            )?,
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_version)?,
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.uid)?,
        })
    }
    fn merge(
        &mut self,
        other: ConfigMapNodeConfigSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.kubelet_config_key {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kubelet_config_key,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.namespace {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.namespace,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
