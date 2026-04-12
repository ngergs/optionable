#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EnvVar represents an environment variable present in a Container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvVarAc {
    /// Name of the environment variable. May consist of any printable ASCII characters except '='.
    pub name: std::string::String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<std::string::String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<
        <::k8s_openapi027::api::core::v1::EnvVarSource as crate::Optionable>::Optioned,
    >,
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
            name: self.name,
            value: self.value,
            value_from: crate::OptionableConvert::into_optioned(self.value_from),
        }
    }
    fn try_from_optioned(value: EnvVarAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            value: value.value,
            value_from: crate::OptionableConvert::try_from_optioned(value.value_from)?,
        })
    }
    fn merge(&mut self, other: EnvVarAc) -> Result<(), crate::Error> {
        self.name = other.name;
        if self.value.is_none() {
            self.value = crate::OptionableConvert::try_from_optioned(other.value)?;
        } else {
            crate::OptionableConvert::merge(&mut self.value, other.value)?;
        }
        if self.value_from.is_none() {
            self.value_from = crate::OptionableConvert::try_from_optioned(
                other.value_from,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.value_from, other.value_from)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::EnvVar {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
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
