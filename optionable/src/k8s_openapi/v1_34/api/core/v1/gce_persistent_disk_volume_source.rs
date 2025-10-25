pub struct GCEPersistentDiskVolumeSourceOpt {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub partition: <Option<i32> as crate::Optionable>::Optioned,
    pub pd_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource {
    type Optioned = GCEPersistentDiskVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for GCEPersistentDiskVolumeSourceOpt {
    type Optioned = GCEPersistentDiskVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource {
    fn into_optioned(self) -> GCEPersistentDiskVolumeSourceOpt {
        GCEPersistentDiskVolumeSourceOpt {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            partition: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.partition),
            pd_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.pd_name,
                ),
            ),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(
        value: GCEPersistentDiskVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            partition: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.partition)?,
            pd_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .pd_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "pd_name",
                    })?,
            )?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: GCEPersistentDiskVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.partition, other.partition)?;
        if let Some(other_value) = other.pd_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.pd_name,
                other_value,
            )?;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
