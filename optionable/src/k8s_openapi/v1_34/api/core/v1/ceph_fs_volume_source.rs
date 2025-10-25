pub struct CephFSVolumeSourceOpt {
    pub monitors: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_file: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::ceph_fs_volume_source::CephFSVolumeSource {
    type Optioned = CephFSVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for CephFSVolumeSourceOpt {
    type Optioned = CephFSVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ceph_fs_volume_source::CephFSVolumeSource {
    fn into_optioned(self) -> CephFSVolumeSourceOpt {
        CephFSVolumeSourceOpt {
            monitors: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.monitors),
            ),
            path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.path),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            secret_file: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.secret_file),
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::LocalObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.secret_ref),
            user: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: CephFSVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            monitors: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .monitors
                    .ok_or(crate::optionable::Error {
                        missing_field: "monitors",
                    })?,
            )?,
            path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.path)?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            secret_file: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.secret_file)?,
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
        other: CephFSVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.monitors {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.monitors, other_value)?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.path, other.path)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.secret_file, other.secret_file)?;
        <Option<
            ::k8s_openapi::api::core::v1::LocalObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.secret_ref, other.secret_ref)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
