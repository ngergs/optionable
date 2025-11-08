#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct EmptyDirVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EmptyDirVolumeSource {
    type Optioned = EmptyDirVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EmptyDirVolumeSourceAc {
    type Optioned = EmptyDirVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EmptyDirVolumeSource {
    fn into_optioned(self) -> EmptyDirVolumeSourceAc {
        EmptyDirVolumeSourceAc {
            medium: crate::OptionableConvert::into_optioned(self.medium),
            size_limit: crate::OptionableConvert::into_optioned(self.size_limit),
        }
    }
    fn try_from_optioned(value: EmptyDirVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            medium: crate::OptionableConvert::try_from_optioned(value.medium)?,
            size_limit: crate::OptionableConvert::try_from_optioned(value.size_limit)?,
        })
    }
    fn merge(&mut self, other: EmptyDirVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.medium, other.medium)?;
        crate::OptionableConvert::merge(&mut self.size_limit, other.size_limit)?;
        Ok(())
    }
}
