pub struct SecretEnvSourceOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::SecretEnvSource {
    type Optioned = SecretEnvSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for SecretEnvSourceOpt {
    type Optioned = SecretEnvSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SecretEnvSource {
    fn into_optioned(self) -> SecretEnvSourceOpt {
        SecretEnvSourceOpt {
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
        value: SecretEnvSourceOpt,
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
        other: SecretEnvSourceOpt,
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
