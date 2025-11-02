#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AWSElasticBlockStoreVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource {
    type Optioned = AWSElasticBlockStoreVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for AWSElasticBlockStoreVolumeSourceAc {
    type Optioned = AWSElasticBlockStoreVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource {
    fn into_optioned(self) -> AWSElasticBlockStoreVolumeSourceAc {
        AWSElasticBlockStoreVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            partition: crate::OptionableConvert::into_optioned(self.partition),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            volume_id: Some(crate::OptionableConvert::into_optioned(self.volume_id)),
        }
    }
    fn try_from_optioned(
        value: AWSElasticBlockStoreVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            partition: crate::OptionableConvert::try_from_optioned(value.partition)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            volume_id: crate::OptionableConvert::try_from_optioned(
                value
                    .volume_id
                    .ok_or(crate::optionable::Error {
                        missing_field: "volume_id",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: AWSElasticBlockStoreVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.partition, other.partition)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.volume_id {
            crate::OptionableConvert::merge(&mut self.volume_id, other_value)?;
        }
        Ok(())
    }
}
