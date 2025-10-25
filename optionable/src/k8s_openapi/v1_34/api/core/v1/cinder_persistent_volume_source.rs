pub struct CinderPersistentVolumeSourceOpt {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    pub volume_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::CinderPersistentVolumeSource {
    type Optioned = CinderPersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for CinderPersistentVolumeSourceOpt {
    type Optioned = CinderPersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::CinderPersistentVolumeSource {
    fn into_optioned(self) -> CinderPersistentVolumeSourceOpt {
        CinderPersistentVolumeSourceOpt {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::into_optioned(self.secret_ref),
            volume_id: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.volume_id,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: CinderPersistentVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretReference,
            > as crate::OptionableConvert>::try_from_optioned(value.secret_ref)?,
            volume_id: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
        other: CinderPersistentVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretReference,
        > as crate::OptionableConvert>::merge(&mut self.secret_ref, other.secret_ref)?;
        if let Some(other_value) = other.volume_id {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.volume_id,
                other_value,
            )?;
        }
        Ok(())
    }
}
