#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMapEnvSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ConfigMapEnvSource {
    type Optioned = ConfigMapEnvSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapEnvSourceAc {
    type Optioned = ConfigMapEnvSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ConfigMapEnvSource {
    fn into_optioned(self) -> ConfigMapEnvSourceAc {
        ConfigMapEnvSourceAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            optional: crate::OptionableConvert::into_optioned(self.optional),
        }
    }
    fn try_from_optioned(
        value: ConfigMapEnvSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            optional: crate::OptionableConvert::try_from_optioned(value.optional)?,
        })
    }
    fn merge(
        &mut self,
        other: ConfigMapEnvSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.optional, other.optional)?;
        Ok(())
    }
}
