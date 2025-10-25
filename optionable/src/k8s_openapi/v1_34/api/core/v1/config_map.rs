pub struct ConfigMapOpt {
    pub binary_data: <Option<
        std::collections::BTreeMap<std::string::String, ::k8s_openapi::ByteString>,
    > as crate::Optionable>::Optioned,
    pub data: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub immutable: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::config_map::ConfigMap {
    type Optioned = ConfigMapOpt;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapOpt {
    type Optioned = ConfigMapOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::config_map::ConfigMap {
    fn into_optioned(self) -> ConfigMapOpt {
        ConfigMapOpt {
            binary_data: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::ByteString,
                >,
            > as crate::OptionableConvert>::into_optioned(self.binary_data),
            data: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.data),
            immutable: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.immutable),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
        }
    }
    fn try_from_optioned(value: ConfigMapOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            binary_data: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::ByteString,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.binary_data)?,
            data: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.data)?,
            immutable: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.immutable)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ConfigMapOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::collections::BTreeMap<std::string::String, ::k8s_openapi::ByteString>,
        > as crate::OptionableConvert>::merge(&mut self.binary_data, other.binary_data)?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.data, other.data)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.immutable, other.immutable)?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        Ok(())
    }
}
