#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NFSVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NFSVolumeSource {
    type Optioned = NFSVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for NFSVolumeSourceAc {
    type Optioned = NFSVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NFSVolumeSource {
    fn into_optioned(self) -> NFSVolumeSourceAc {
        NFSVolumeSourceAc {
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            server: Some(crate::OptionableConvert::into_optioned(self.server)),
        }
    }
    fn try_from_optioned(value: NFSVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::Error {
                        missing_field: "path",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            server: crate::OptionableConvert::try_from_optioned(
                value
                    .server
                    .ok_or(crate::Error {
                        missing_field: "server",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NFSVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.server {
            crate::OptionableConvert::merge(&mut self.server, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NFSVolumeSource>
for NFSVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NFSVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NFSVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NFSVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
