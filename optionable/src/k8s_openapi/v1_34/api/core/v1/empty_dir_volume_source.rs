#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
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
#[cfg(feature = "k8s_openapi_convert")]
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::EmptyDirVolumeSource>
for EmptyDirVolumeSourceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::EmptyDirVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::EmptyDirVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::EmptyDirVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
