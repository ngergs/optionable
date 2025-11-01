pub struct StorageOSPersistentVolumeSourceAc {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    pub volume_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub volume_namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::StorageOSPersistentVolumeSource {
    type Optioned = StorageOSPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for StorageOSPersistentVolumeSourceAc {
    type Optioned = StorageOSPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::StorageOSPersistentVolumeSource {
    fn into_optioned(self) -> StorageOSPersistentVolumeSourceAc {
        StorageOSPersistentVolumeSourceAc {
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
        value: StorageOSPersistentVolumeSourceAc,
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
        other: StorageOSPersistentVolumeSourceAc,
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
