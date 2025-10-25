pub struct LocalVolumeSourceOpt {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::local_volume_source::LocalVolumeSource {
    type Optioned = LocalVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for LocalVolumeSourceOpt {
    type Optioned = LocalVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::local_volume_source::LocalVolumeSource {
    fn into_optioned(self) -> LocalVolumeSourceOpt {
        LocalVolumeSourceOpt {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.path,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: LocalVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: LocalVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        if let Some(other_value) = other.path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.path,
                other_value,
            )?;
        }
        Ok(())
    }
}
