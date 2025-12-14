#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostPathVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::HostPathVolumeSource {
    type Optioned = HostPathVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for HostPathVolumeSourceAc {
    type Optioned = HostPathVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HostPathVolumeSource {
    fn into_optioned(self) -> HostPathVolumeSourceAc {
        HostPathVolumeSourceAc {
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(value: HostPathVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::Error {
                        missing_field: "path",
                    })?,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(&mut self, other: HostPathVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::HostPathVolumeSource>
for HostPathVolumeSourceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::HostPathVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::HostPathVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::HostPathVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
