pub struct SecretOpt {
    pub data: <Option<
        std::collections::BTreeMap<std::string::String, ::k8s_openapi::ByteString>,
    > as crate::Optionable>::Optioned,
    pub immutable: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub string_data: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Secret {
    type Optioned = SecretOpt;
}
#[automatically_derived]
impl crate::Optionable for SecretOpt {
    type Optioned = SecretOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Secret {
    fn into_optioned(self) -> SecretOpt {
        SecretOpt {
            data: crate::OptionableConvert::into_optioned(self.data),
            immutable: crate::OptionableConvert::into_optioned(self.immutable),
            metadata: self.metadata,
            string_data: crate::OptionableConvert::into_optioned(self.string_data),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(value: SecretOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            immutable: crate::OptionableConvert::try_from_optioned(value.immutable)?,
            metadata: value.metadata,
            string_data: crate::OptionableConvert::try_from_optioned(value.string_data)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: SecretOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        crate::OptionableConvert::merge(&mut self.immutable, other.immutable)?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.string_data, other.string_data)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
