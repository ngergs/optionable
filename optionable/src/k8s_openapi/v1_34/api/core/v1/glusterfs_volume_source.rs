pub struct GlusterfsVolumeSourceOpt {
    pub endpoints: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::GlusterfsVolumeSource {
    type Optioned = GlusterfsVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for GlusterfsVolumeSourceOpt {
    type Optioned = GlusterfsVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::GlusterfsVolumeSource {
    fn into_optioned(self) -> GlusterfsVolumeSourceOpt {
        GlusterfsVolumeSourceOpt {
            endpoints: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.endpoints,
                ),
            ),
            path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.path,
                ),
            ),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(
        value: GlusterfsVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            endpoints: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .endpoints
                    .ok_or(crate::optionable::Error {
                        missing_field: "endpoints",
                    })?,
            )?,
            path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: GlusterfsVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.endpoints {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.endpoints,
                other_value,
            )?;
        }
        if let Some(other_value) = other.path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.path,
                other_value,
            )?;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
