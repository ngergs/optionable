pub struct ConfigMapEnvSourceOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ConfigMapEnvSource {
    type Optioned = ConfigMapEnvSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapEnvSourceOpt {
    type Optioned = ConfigMapEnvSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ConfigMapEnvSource {
    fn into_optioned(self) -> ConfigMapEnvSourceOpt {
        ConfigMapEnvSourceOpt {
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
        value: ConfigMapEnvSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
        other: ConfigMapEnvSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
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
