pub struct StorageOSVolumeSourceOpt {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    pub volume_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub volume_namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::StorageOSVolumeSource {
    type Optioned = StorageOSVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for StorageOSVolumeSourceOpt {
    type Optioned = StorageOSVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::StorageOSVolumeSource {
    fn into_optioned(self) -> StorageOSVolumeSourceOpt {
        StorageOSVolumeSourceOpt {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::LocalObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.secret_ref),
            volume_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.volume_name),
            volume_namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.volume_namespace),
        }
    }
    fn try_from_optioned(
        value: StorageOSVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::LocalObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.secret_ref)?,
            volume_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.volume_name)?,
            volume_namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.volume_namespace)?,
        })
    }
    fn merge(
        &mut self,
        other: StorageOSVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        <Option<
            ::k8s_openapi::api::core::v1::LocalObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.secret_ref, other.secret_ref)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.volume_name, other.volume_name)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.volume_namespace,
            other.volume_namespace,
        )?;
        Ok(())
    }
}
