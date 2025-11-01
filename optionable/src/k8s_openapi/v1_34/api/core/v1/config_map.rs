pub struct ConfigMapOpt {
    pub binary_data: <Option<
        std::collections::BTreeMap<std::string::String, ::k8s_openapi::ByteString>,
    > as crate::Optionable>::Optioned,
    pub data: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub immutable: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ConfigMap {
    type Optioned = ConfigMapOpt;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapOpt {
    type Optioned = ConfigMapOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ConfigMap {
    fn into_optioned(self) -> ConfigMapOpt {
        ConfigMapOpt {
            binary_data: crate::OptionableConvert::into_optioned(self.binary_data),
            data: crate::OptionableConvert::into_optioned(self.data),
            immutable: crate::OptionableConvert::into_optioned(self.immutable),
            metadata: self.metadata,
        }
    }
    fn try_from_optioned(value: ConfigMapOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            binary_data: crate::OptionableConvert::try_from_optioned(value.binary_data)?,
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            immutable: crate::OptionableConvert::try_from_optioned(value.immutable)?,
            metadata: value.metadata,
        })
    }
    fn merge(&mut self, other: ConfigMapOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.binary_data, other.binary_data)?;
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        crate::OptionableConvert::merge(&mut self.immutable, other.immutable)?;
        self.metadata = other.metadata;
        Ok(())
    }
}
