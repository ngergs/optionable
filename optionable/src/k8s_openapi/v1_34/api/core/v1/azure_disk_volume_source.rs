pub struct AzureDiskVolumeSourceOpt {
    pub caching_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub disk_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub disk_uri: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::AzureDiskVolumeSource {
    type Optioned = AzureDiskVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for AzureDiskVolumeSourceOpt {
    type Optioned = AzureDiskVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::AzureDiskVolumeSource {
    fn into_optioned(self) -> AzureDiskVolumeSourceOpt {
        AzureDiskVolumeSourceOpt {
            caching_mode: crate::OptionableConvert::into_optioned(self.caching_mode),
            disk_name: Some(crate::OptionableConvert::into_optioned(self.disk_name)),
            disk_uri: Some(crate::OptionableConvert::into_optioned(self.disk_uri)),
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            kind: crate::OptionableConvert::into_optioned(self.kind),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(
        value: AzureDiskVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            caching_mode: crate::OptionableConvert::try_from_optioned(
                value.caching_mode,
            )?,
            disk_name: crate::OptionableConvert::try_from_optioned(
                value
                    .disk_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "disk_name",
                    })?,
            )?,
            disk_uri: crate::OptionableConvert::try_from_optioned(
                value
                    .disk_uri
                    .ok_or(crate::optionable::Error {
                        missing_field: "disk_uri",
                    })?,
            )?,
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: AzureDiskVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.caching_mode, other.caching_mode)?;
        if let Some(other_value) = other.disk_name {
            crate::OptionableConvert::merge(&mut self.disk_name, other_value)?;
        }
        if let Some(other_value) = other.disk_uri {
            crate::OptionableConvert::merge(&mut self.disk_uri, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
