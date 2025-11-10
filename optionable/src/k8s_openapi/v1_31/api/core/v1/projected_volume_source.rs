#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::VolumeProjection>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ProjectedVolumeSource {
    type Optioned = ProjectedVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ProjectedVolumeSourceAc {
    type Optioned = ProjectedVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ProjectedVolumeSource {
    fn into_optioned(self) -> ProjectedVolumeSourceAc {
        ProjectedVolumeSourceAc {
            default_mode: crate::OptionableConvert::into_optioned(self.default_mode),
            sources: crate::OptionableConvert::into_optioned(self.sources),
        }
    }
    fn try_from_optioned(value: ProjectedVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            default_mode: crate::OptionableConvert::try_from_optioned(
                value.default_mode,
            )?,
            sources: crate::OptionableConvert::try_from_optioned(value.sources)?,
        })
    }
    fn merge(&mut self, other: ProjectedVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.default_mode, other.default_mode)?;
        crate::OptionableConvert::merge(&mut self.sources, other.sources)?;
        Ok(())
    }
}
