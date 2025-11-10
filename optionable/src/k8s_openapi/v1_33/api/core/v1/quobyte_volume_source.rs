#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct QuobyteVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::QuobyteVolumeSource {
    type Optioned = QuobyteVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for QuobyteVolumeSourceAc {
    type Optioned = QuobyteVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::QuobyteVolumeSource {
    fn into_optioned(self) -> QuobyteVolumeSourceAc {
        QuobyteVolumeSourceAc {
            group: crate::OptionableConvert::into_optioned(self.group),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            registry: Some(crate::OptionableConvert::into_optioned(self.registry)),
            tenant: crate::OptionableConvert::into_optioned(self.tenant),
            user: crate::OptionableConvert::into_optioned(self.user),
            volume: Some(crate::OptionableConvert::into_optioned(self.volume)),
        }
    }
    fn try_from_optioned(value: QuobyteVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            registry: crate::OptionableConvert::try_from_optioned(
                value
                    .registry
                    .ok_or(crate::Error {
                        missing_field: "registry",
                    })?,
            )?,
            tenant: crate::OptionableConvert::try_from_optioned(value.tenant)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
            volume: crate::OptionableConvert::try_from_optioned(
                value
                    .volume
                    .ok_or(crate::Error {
                        missing_field: "volume",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: QuobyteVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.registry {
            crate::OptionableConvert::merge(&mut self.registry, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.tenant, other.tenant)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        if let Some(other_value) = other.volume {
            crate::OptionableConvert::merge(&mut self.volume, other_value)?;
        }
        Ok(())
    }
}
