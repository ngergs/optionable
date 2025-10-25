pub struct FlexPersistentVolumeSourceOpt {
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub options: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::FlexPersistentVolumeSource {
    type Optioned = FlexPersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for FlexPersistentVolumeSourceOpt {
    type Optioned = FlexPersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::FlexPersistentVolumeSource {
    fn into_optioned(self) -> FlexPersistentVolumeSourceOpt {
        FlexPersistentVolumeSourceOpt {
            driver: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.driver,
                ),
            ),
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            options: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.options),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::into_optioned(self.secret_ref),
        }
    }
    fn try_from_optioned(
        value: FlexPersistentVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            driver: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::optionable::Error {
                        missing_field: "driver",
                    })?,
            )?,
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            options: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.options)?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::try_from_optioned(value.secret_ref)?,
        })
    }
    fn merge(
        &mut self,
        other: FlexPersistentVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.driver {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.driver,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.options, other.options)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretReference,
        > as crate::OptionableConvert>::merge(&mut self.secret_ref, other.secret_ref)?;
        Ok(())
    }
}
