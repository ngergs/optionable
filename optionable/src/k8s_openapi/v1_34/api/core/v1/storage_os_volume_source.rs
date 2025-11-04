#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct StorageOSVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::StorageOSVolumeSource {
    type Optioned = StorageOSVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for StorageOSVolumeSourceAc {
    type Optioned = StorageOSVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::StorageOSVolumeSource {
    fn into_optioned(self) -> StorageOSVolumeSourceAc {
        StorageOSVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            volume_name: crate::OptionableConvert::into_optioned(self.volume_name),
            volume_namespace: crate::OptionableConvert::into_optioned(
                self.volume_namespace,
            ),
        }
    }
    fn try_from_optioned(
        value: StorageOSVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            volume_name: crate::OptionableConvert::try_from_optioned(value.volume_name)?,
            volume_namespace: crate::OptionableConvert::try_from_optioned(
                value.volume_namespace,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: StorageOSVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        crate::OptionableConvert::merge(&mut self.volume_name, other.volume_name)?;
        crate::OptionableConvert::merge(
            &mut self.volume_namespace,
            other.volume_namespace,
        )?;
        Ok(())
    }
}
