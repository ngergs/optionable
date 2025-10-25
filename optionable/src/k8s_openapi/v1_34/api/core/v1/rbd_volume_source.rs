pub struct RBDVolumeSourceOpt {
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub image: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub keyring: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub monitors: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub pool: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::RBDVolumeSource {
    type Optioned = RBDVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for RBDVolumeSourceOpt {
    type Optioned = RBDVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::RBDVolumeSource {
    fn into_optioned(self) -> RBDVolumeSourceOpt {
        RBDVolumeSourceOpt {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_type),
            image: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.image,
                ),
            ),
            keyring: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.keyring),
            monitors: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.monitors),
            ),
            pool: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.pool),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::LocalObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.secret_ref),
            user: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: RBDVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_type)?,
            image: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .image
                    .ok_or(crate::optionable::Error {
                        missing_field: "image",
                    })?,
            )?,
            keyring: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.keyring)?,
            monitors: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .monitors
                    .ok_or(crate::optionable::Error {
                        missing_field: "monitors",
                    })?,
            )?,
            pool: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.pool)?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::LocalObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.secret_ref)?,
            user: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.user)?,
        })
    }
    fn merge(
        &mut self,
        other: RBDVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.fs_type, other.fs_type)?;
        if let Some(other_value) = other.image {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.image,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.keyring, other.keyring)?;
        if let Some(other_value) = other.monitors {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.monitors, other_value)?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.pool, other.pool)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        <Option<
            ::k8s_openapi::api::core::v1::LocalObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.secret_ref, other.secret_ref)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
