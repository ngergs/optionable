pub struct SecretVolumeSourceOpt {
    pub default_mode: <Option<i32> as crate::Optionable>::Optioned,
    pub items: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
    > as crate::Optionable>::Optioned,
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::secret_volume_source::SecretVolumeSource {
    type Optioned = SecretVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for SecretVolumeSourceOpt {
    type Optioned = SecretVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::secret_volume_source::SecretVolumeSource {
    fn into_optioned(self) -> SecretVolumeSourceOpt {
        SecretVolumeSourceOpt {
            default_mode: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.default_mode),
            items: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
            > as crate::OptionableConvert>::into_optioned(self.items),
            optional: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.optional),
            secret_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.secret_name),
        }
    }
    fn try_from_optioned(
        value: SecretVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            default_mode: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.default_mode)?,
            items: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
            > as crate::OptionableConvert>::try_from_optioned(value.items)?,
            optional: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.optional)?,
            secret_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.secret_name)?,
        })
    }
    fn merge(
        &mut self,
        other: SecretVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.default_mode,
            other.default_mode,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
        > as crate::OptionableConvert>::merge(&mut self.items, other.items)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.optional, other.optional)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.secret_name, other.secret_name)?;
        Ok(())
    }
}
