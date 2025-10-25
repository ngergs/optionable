pub struct ConfigMapKeySelectorOpt {
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ConfigMapKeySelector {
    type Optioned = ConfigMapKeySelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapKeySelectorOpt {
    type Optioned = ConfigMapKeySelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ConfigMapKeySelector {
    fn into_optioned(self) -> ConfigMapKeySelectorOpt {
        ConfigMapKeySelectorOpt {
            key: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.key,
                ),
            ),
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
        value: ConfigMapKeySelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            key: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .key
                    .ok_or(crate::optionable::Error {
                        missing_field: "key",
                    })?,
            )?,
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
        other: ConfigMapKeySelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.key {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.key,
                other_value,
            )?;
        }
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
