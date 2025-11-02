#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GCEPersistentDiskVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pd_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource {
    type Optioned = GCEPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GCEPersistentDiskVolumeSourceAc {
    type Optioned = GCEPersistentDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource {
    fn into_optioned(self) -> GCEPersistentDiskVolumeSourceAc {
        GCEPersistentDiskVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            partition: crate::OptionableConvert::into_optioned(self.partition),
            pd_name: Some(crate::OptionableConvert::into_optioned(self.pd_name)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(
        value: GCEPersistentDiskVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            partition: crate::OptionableConvert::try_from_optioned(value.partition)?,
            pd_name: crate::OptionableConvert::try_from_optioned(
                value
                    .pd_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "pd_name",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: GCEPersistentDiskVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.partition, other.partition)?;
        if let Some(other_value) = other.pd_name {
            crate::OptionableConvert::merge(&mut self.pd_name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
