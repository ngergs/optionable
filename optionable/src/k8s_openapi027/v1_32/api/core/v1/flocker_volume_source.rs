#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName and datasetUUID should be set. Flocker volumes do not support ownership management or SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlockerVolumeSourceAc {
    /// datasetName is Name of the dataset stored as metadata -\> name on the dataset for Flocker should be considered as deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<std::string::String>,
    /// datasetUUID is the UUID of the dataset. This is unique identifier of a Flocker dataset
    #[serde(rename = "datasetUUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_uuid: Option<std::string::String>,
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
            dataset_name: self.dataset_name,
            dataset_uuid: self.dataset_uuid,
        }
    }
    fn try_from_optioned(value: FlockerVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            dataset_name: value.dataset_name,
            dataset_uuid: value.dataset_uuid,
        })
    }
    fn merge(&mut self, other: FlockerVolumeSourceAc) -> Result<(), crate::Error> {
        if self.dataset_name.is_none() {
            self.dataset_name = other.dataset_name;
        }
        if let Some(other_value) = other.dataset_name {
            crate::OptionableConvert::merge(&mut self.dataset_name, other_value)?;
        }
        if self.dataset_uuid.is_none() {
            self.dataset_uuid = other.dataset_uuid;
        }
        if let Some(other_value) = other.dataset_uuid {
            crate::OptionableConvert::merge(&mut self.dataset_uuid, other_value)?;
        }
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
