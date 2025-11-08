#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct AzureFileVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::AzureFileVolumeSource {
    type Optioned = AzureFileVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for AzureFileVolumeSourceAc {
    type Optioned = AzureFileVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::AzureFileVolumeSource {
    fn into_optioned(self) -> AzureFileVolumeSourceAc {
        AzureFileVolumeSourceAc {
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_name: Some(crate::OptionableConvert::into_optioned(self.secret_name)),
            share_name: Some(crate::OptionableConvert::into_optioned(self.share_name)),
        }
    }
    fn try_from_optioned(value: AzureFileVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_name: crate::OptionableConvert::try_from_optioned(
                value
                    .secret_name
                    .ok_or(crate::Error {
                        missing_field: "secret_name",
                    })?,
            )?,
            share_name: crate::OptionableConvert::try_from_optioned(
                value
                    .share_name
                    .ok_or(crate::Error {
                        missing_field: "share_name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: AzureFileVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.secret_name {
            crate::OptionableConvert::merge(&mut self.secret_name, other_value)?;
        }
        if let Some(other_value) = other.share_name {
            crate::OptionableConvert::merge(&mut self.share_name, other_value)?;
        }
        Ok(())
    }
}
