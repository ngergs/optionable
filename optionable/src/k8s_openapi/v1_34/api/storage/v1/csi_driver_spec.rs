pub struct CSIDriverSpecOpt {
    pub attach_required: <Option<bool> as crate::Optionable>::Optioned,
    pub fs_group_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_allocatable_update_period_seconds: <Option<
        i64,
    > as crate::Optionable>::Optioned,
    pub pod_info_on_mount: <Option<bool> as crate::Optionable>::Optioned,
    pub requires_republish: <Option<bool> as crate::Optionable>::Optioned,
    pub se_linux_mount: <Option<bool> as crate::Optionable>::Optioned,
    pub storage_capacity: <Option<bool> as crate::Optionable>::Optioned,
    pub token_requests: <Option<
        std::vec::Vec<::k8s_openapi::api::storage::v1::TokenRequest>,
    > as crate::Optionable>::Optioned,
    pub volume_lifecycle_modes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::CSIDriverSpec {
    type Optioned = CSIDriverSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for CSIDriverSpecOpt {
    type Optioned = CSIDriverSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSIDriverSpec {
    fn into_optioned(self) -> CSIDriverSpecOpt {
        CSIDriverSpecOpt {
            attach_required: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.attach_required),
            fs_group_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.fs_group_policy),
            node_allocatable_update_period_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(
                self.node_allocatable_update_period_seconds,
            ),
            pod_info_on_mount: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.pod_info_on_mount),
            requires_republish: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.requires_republish),
            se_linux_mount: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.se_linux_mount),
            storage_capacity: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.storage_capacity),
            token_requests: <Option<
                std::vec::Vec<::k8s_openapi::api::storage::v1::TokenRequest>,
            > as crate::OptionableConvert>::into_optioned(self.token_requests),
            volume_lifecycle_modes: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.volume_lifecycle_modes),
        }
    }
    fn try_from_optioned(
        value: CSIDriverSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            attach_required: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.attach_required)?,
            fs_group_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.fs_group_policy)?,
            node_allocatable_update_period_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.node_allocatable_update_period_seconds,
            )?,
            pod_info_on_mount: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.pod_info_on_mount)?,
            requires_republish: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.requires_republish)?,
            se_linux_mount: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.se_linux_mount)?,
            storage_capacity: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.storage_capacity)?,
            token_requests: <Option<
                std::vec::Vec<::k8s_openapi::api::storage::v1::TokenRequest>,
            > as crate::OptionableConvert>::try_from_optioned(value.token_requests)?,
            volume_lifecycle_modes: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.volume_lifecycle_modes,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CSIDriverSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.attach_required,
            other.attach_required,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.fs_group_policy,
            other.fs_group_policy,
        )?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.node_allocatable_update_period_seconds,
            other.node_allocatable_update_period_seconds,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.pod_info_on_mount,
            other.pod_info_on_mount,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.requires_republish,
            other.requires_republish,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.se_linux_mount,
            other.se_linux_mount,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.storage_capacity,
            other.storage_capacity,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::storage::v1::TokenRequest>,
        > as crate::OptionableConvert>::merge(
            &mut self.token_requests,
            other.token_requests,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.volume_lifecycle_modes,
            other.volume_lifecycle_modes,
        )?;
        Ok(())
    }
}
