#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct FCVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(rename = "targetWWNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_wwns: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wwids: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::FCVolumeSource {
    type Optioned = FCVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for FCVolumeSourceAc {
    type Optioned = FCVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::FCVolumeSource {
    fn into_optioned(self) -> FCVolumeSourceAc {
        FCVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            lun: crate::OptionableConvert::into_optioned(self.lun),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            target_wwns: crate::OptionableConvert::into_optioned(self.target_wwns),
            wwids: crate::OptionableConvert::into_optioned(self.wwids),
        }
    }
    fn try_from_optioned(value: FCVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            lun: crate::OptionableConvert::try_from_optioned(value.lun)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            target_wwns: crate::OptionableConvert::try_from_optioned(value.target_wwns)?,
            wwids: crate::OptionableConvert::try_from_optioned(value.wwids)?,
        })
    }
    fn merge(&mut self, other: FCVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.lun, other.lun)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.target_wwns, other.target_wwns)?;
        crate::OptionableConvert::merge(&mut self.wwids, other.wwids)?;
        Ok(())
    }
}
