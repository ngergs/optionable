pub struct GlusterfsPersistentVolumeSourceOpt {
    pub endpoints: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub endpoints_namespace: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::glusterfs_persistent_volume_source::GlusterfsPersistentVolumeSource {
    type Optioned = GlusterfsPersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for GlusterfsPersistentVolumeSourceOpt {
    type Optioned = GlusterfsPersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::glusterfs_persistent_volume_source::GlusterfsPersistentVolumeSource {
    fn into_optioned(self) -> GlusterfsPersistentVolumeSourceOpt {
        GlusterfsPersistentVolumeSourceOpt {
            endpoints: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.endpoints,
                ),
            ),
            endpoints_namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.endpoints_namespace),
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
        value: GlusterfsPersistentVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            endpoints: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .endpoints
                    .ok_or(crate::optionable::Error {
                        missing_field: "endpoints",
                    })?,
            )?,
            endpoints_namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.endpoints_namespace,
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
        other: GlusterfsPersistentVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.endpoints {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.endpoints,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.endpoints_namespace,
            other.endpoints_namespace,
        )?;
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
