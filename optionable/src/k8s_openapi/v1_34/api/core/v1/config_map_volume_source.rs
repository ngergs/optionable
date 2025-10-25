pub struct ConfigMapVolumeSourceOpt {
    pub default_mode: <Option<i32> as crate::Optionable>::Optioned,
    pub items: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::config_map_volume_source::ConfigMapVolumeSource {
    type Optioned = ConfigMapVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapVolumeSourceOpt {
    type Optioned = ConfigMapVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::config_map_volume_source::ConfigMapVolumeSource {
    fn into_optioned(self) -> ConfigMapVolumeSourceOpt {
        ConfigMapVolumeSourceOpt {
            default_mode: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.default_mode),
            items: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
            > as crate::OptionableConvert>::into_optioned(self.items),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            optional: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.optional),
        }
    }
    fn try_from_optioned(
        value: ConfigMapVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            default_mode: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.default_mode)?,
            items: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
            > as crate::OptionableConvert>::try_from_optioned(value.items)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            optional: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.optional)?,
        })
    }
    fn merge(
        &mut self,
        other: ConfigMapVolumeSourceOpt,
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
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.optional, other.optional)?;
        Ok(())
    }
}
