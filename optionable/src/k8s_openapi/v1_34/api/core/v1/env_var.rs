pub struct EnvVarAc {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub value_from: <Option<
        ::k8s_openapi::api::core::v1::EnvVarSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EnvVar {
    type Optioned = EnvVarAc;
}
#[automatically_derived]
impl crate::Optionable for EnvVarAc {
    type Optioned = EnvVarAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EnvVar {
    fn into_optioned(self) -> EnvVarAc {
        EnvVarAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            value: crate::OptionableConvert::into_optioned(self.value),
            value_from: crate::OptionableConvert::into_optioned(self.value_from),
        }
    }
    fn try_from_optioned(value: EnvVarAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
            value_from: crate::OptionableConvert::try_from_optioned(value.value_from)?,
        })
    }
    fn merge(&mut self, other: EnvVarAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        crate::OptionableConvert::merge(&mut self.value_from, other.value_from)?;
        Ok(())
    }
}
