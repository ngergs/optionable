#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigMapVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::KeyToPath>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ConfigMapVolumeSource {
    type Optioned = ConfigMapVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ConfigMapVolumeSourceAc {
    type Optioned = ConfigMapVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ConfigMapVolumeSource {
    fn into_optioned(self) -> ConfigMapVolumeSourceAc {
        ConfigMapVolumeSourceAc {
            default_mode: crate::OptionableConvert::into_optioned(self.default_mode),
            items: crate::OptionableConvert::into_optioned(self.items),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            optional: crate::OptionableConvert::into_optioned(self.optional),
        }
    }
    fn try_from_optioned(value: ConfigMapVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            default_mode: crate::OptionableConvert::try_from_optioned(
                value.default_mode,
            )?,
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            optional: crate::OptionableConvert::try_from_optioned(value.optional)?,
        })
    }
    fn merge(&mut self, other: ConfigMapVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.default_mode, other.default_mode)?;
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.optional, other.optional)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::ConfigMapVolumeSource>
for ConfigMapVolumeSourceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::ConfigMapVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::ConfigMapVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ConfigMapVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
