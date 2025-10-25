pub struct ConfigMapProjectionOpt {
    pub items: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::config_map_projection::ConfigMapProjection {
    type Optioned = ConfigMapProjectionOpt;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapProjectionOpt {
    type Optioned = ConfigMapProjectionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::config_map_projection::ConfigMapProjection {
    fn into_optioned(self) -> ConfigMapProjectionOpt {
        ConfigMapProjectionOpt {
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
        value: ConfigMapProjectionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
        other: ConfigMapProjectionOpt,
    ) -> Result<(), crate::optionable::Error> {
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
