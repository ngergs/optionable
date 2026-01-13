#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvVarAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: <Option<
        ::k8s_openapi027::api::core::v1::EnvVarSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EnvVar {
    type Optioned = EnvVarAc;
}
#[automatically_derived]
impl crate::Optionable for EnvVarAc {
    type Optioned = EnvVarAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EnvVar {
    fn into_optioned(self) -> EnvVarAc {
        EnvVarAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            value: crate::OptionableConvert::into_optioned(self.value),
            value_from: crate::OptionableConvert::into_optioned(self.value_from),
        }
    }
    fn try_from_optioned(value: EnvVarAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
            value_from: crate::OptionableConvert::try_from_optioned(value.value_from)?,
        })
    }
    fn merge(&mut self, other: EnvVarAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        crate::OptionableConvert::merge(&mut self.value_from, other.value_from)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EnvVar> for EnvVarAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::EnvVar) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EnvVar, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EnvVar,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
