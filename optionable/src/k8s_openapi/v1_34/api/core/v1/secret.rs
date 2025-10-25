pub struct SecretOpt {
    pub data: <Option<
        std::collections::BTreeMap<std::string::String, ::k8s_openapi::ByteString>,
    > as crate::Optionable>::Optioned,
    pub immutable: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub string_data: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::secret::Secret {
    type Optioned = SecretOpt;
}
#[automatically_derived]
impl crate::Optionable for SecretOpt {
    type Optioned = SecretOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::secret::Secret {
    fn into_optioned(self) -> SecretOpt {
        SecretOpt {
            data: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::ByteString,
                >,
            > as crate::OptionableConvert>::into_optioned(self.data),
            immutable: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.immutable),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            string_data: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.string_data),
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(value: SecretOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            data: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::ByteString,
                >,
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
            string_data: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.string_data)?,
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: SecretOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::collections::BTreeMap<std::string::String, ::k8s_openapi::ByteString>,
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
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.string_data, other.string_data)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
