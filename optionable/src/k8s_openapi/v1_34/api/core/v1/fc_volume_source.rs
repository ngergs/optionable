pub struct FCVolumeSourceOpt {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub lun: <Option<i32> as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub target_wwns: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub wwids: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::FCVolumeSource {
    type Optioned = FCVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for FCVolumeSourceOpt {
    type Optioned = FCVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::FCVolumeSource {
    fn into_optioned(self) -> FCVolumeSourceOpt {
        FCVolumeSourceOpt {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            lun: crate::OptionableConvert::into_optioned(self.lun),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            target_wwns: crate::OptionableConvert::into_optioned(self.target_wwns),
            wwids: crate::OptionableConvert::into_optioned(self.wwids),
        }
    }
    fn try_from_optioned(
        value: FCVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            lun: crate::OptionableConvert::try_from_optioned(value.lun)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            target_wwns: crate::OptionableConvert::try_from_optioned(value.target_wwns)?,
            wwids: crate::OptionableConvert::try_from_optioned(value.wwids)?,
        })
    }
    fn merge(
        &mut self,
        other: FCVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.lun, other.lun)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.target_wwns, other.target_wwns)?;
        crate::OptionableConvert::merge(&mut self.wwids, other.wwids)?;
        Ok(())
    }
}
