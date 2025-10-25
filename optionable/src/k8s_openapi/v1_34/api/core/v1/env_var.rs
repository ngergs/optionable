pub struct EnvVarOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub value_from: <Option<
        ::k8s_openapi::api::core::v1::EnvVarSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::env_var::EnvVar {
    type Optioned = EnvVarOpt;
}
#[automatically_derived]
impl crate::Optionable for EnvVarOpt {
    type Optioned = EnvVarOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::env_var::EnvVar {
    fn into_optioned(self) -> EnvVarOpt {
        EnvVarOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            value: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.value),
            value_from: <Option<
                ::k8s_openapi::api::core::v1::EnvVarSource,
            > as crate::OptionableConvert>::into_optioned(self.value_from),
        }
    }
    fn try_from_optioned(value: EnvVarOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            value: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.value)?,
            value_from: <Option<
                ::k8s_openapi::api::core::v1::EnvVarSource,
            > as crate::OptionableConvert>::try_from_optioned(value.value_from)?,
        })
    }
    fn merge(&mut self, other: EnvVarOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.value, other.value)?;
        <Option<
            ::k8s_openapi::api::core::v1::EnvVarSource,
        > as crate::OptionableConvert>::merge(&mut self.value_from, other.value_from)?;
        Ok(())
    }
}
