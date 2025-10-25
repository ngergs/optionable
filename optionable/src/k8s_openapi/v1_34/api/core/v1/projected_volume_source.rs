pub struct ProjectedVolumeSourceOpt {
    pub default_mode: <Option<i32> as crate::Optionable>::Optioned,
    pub sources: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::VolumeProjection>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ProjectedVolumeSource {
    type Optioned = ProjectedVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for ProjectedVolumeSourceOpt {
    type Optioned = ProjectedVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ProjectedVolumeSource {
    fn into_optioned(self) -> ProjectedVolumeSourceOpt {
        ProjectedVolumeSourceOpt {
            default_mode: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.default_mode),
            sources: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::VolumeProjection>,
            > as crate::OptionableConvert>::into_optioned(self.sources),
        }
    }
    fn try_from_optioned(
        value: ProjectedVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            default_mode: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.default_mode)?,
            sources: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::VolumeProjection>,
            > as crate::OptionableConvert>::try_from_optioned(value.sources)?,
        })
    }
    fn merge(
        &mut self,
        other: ProjectedVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.default_mode,
            other.default_mode,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::VolumeProjection>,
        > as crate::OptionableConvert>::merge(&mut self.sources, other.sources)?;
        Ok(())
    }
}
