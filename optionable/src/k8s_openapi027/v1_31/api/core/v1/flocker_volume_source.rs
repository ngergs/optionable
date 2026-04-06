#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlockerVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(rename = "datasetUUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_uuid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::FlockerVolumeSource {
    type Optioned = FlockerVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for FlockerVolumeSourceAc {
    type Optioned = FlockerVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::FlockerVolumeSource {
    fn into_optioned(self) -> FlockerVolumeSourceAc {
        FlockerVolumeSourceAc {
            dataset_name: crate::OptionableConvert::into_optioned(self.dataset_name),
            dataset_uuid: crate::OptionableConvert::into_optioned(self.dataset_uuid),
        }
    }
    fn try_from_optioned(value: FlockerVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            dataset_name: crate::OptionableConvert::try_from_optioned(
                value.dataset_name,
            )?,
            dataset_uuid: crate::OptionableConvert::try_from_optioned(
                value.dataset_uuid,
            )?,
        })
    }
    fn merge(&mut self, other: FlockerVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.dataset_name, other.dataset_name)?;
        crate::OptionableConvert::merge(&mut self.dataset_uuid, other.dataset_uuid)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::FlockerVolumeSource>
for FlockerVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::FlockerVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::FlockerVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::FlockerVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
