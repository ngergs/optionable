#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSIDriverSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_required: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_group_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_allocatable_update_period_seconds: <Option<
        i64,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_info_on_mount: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_republish: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_linux_mount: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_requests: <Option<
        std::vec::Vec<::k8s_openapi026::api::storage::v1::TokenRequest>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_lifecycle_modes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::storage::v1::CSIDriverSpec {
    type Optioned = CSIDriverSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CSIDriverSpecAc {
    type Optioned = CSIDriverSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::storage::v1::CSIDriverSpec {
    fn into_optioned(self) -> CSIDriverSpecAc {
        CSIDriverSpecAc {
            attach_required: crate::OptionableConvert::into_optioned(
                self.attach_required,
            ),
            fs_group_policy: crate::OptionableConvert::into_optioned(
                self.fs_group_policy,
            ),
            node_allocatable_update_period_seconds: crate::OptionableConvert::into_optioned(
                self.node_allocatable_update_period_seconds,
            ),
            pod_info_on_mount: crate::OptionableConvert::into_optioned(
                self.pod_info_on_mount,
            ),
            requires_republish: crate::OptionableConvert::into_optioned(
                self.requires_republish,
            ),
            se_linux_mount: crate::OptionableConvert::into_optioned(self.se_linux_mount),
            storage_capacity: crate::OptionableConvert::into_optioned(
                self.storage_capacity,
            ),
            token_requests: crate::OptionableConvert::into_optioned(self.token_requests),
            volume_lifecycle_modes: crate::OptionableConvert::into_optioned(
                self.volume_lifecycle_modes,
            ),
        }
    }
    fn try_from_optioned(value: CSIDriverSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            attach_required: crate::OptionableConvert::try_from_optioned(
                value.attach_required,
            )?,
            fs_group_policy: crate::OptionableConvert::try_from_optioned(
                value.fs_group_policy,
            )?,
            node_allocatable_update_period_seconds: crate::OptionableConvert::try_from_optioned(
                value.node_allocatable_update_period_seconds,
            )?,
            pod_info_on_mount: crate::OptionableConvert::try_from_optioned(
                value.pod_info_on_mount,
            )?,
            requires_republish: crate::OptionableConvert::try_from_optioned(
                value.requires_republish,
            )?,
            se_linux_mount: crate::OptionableConvert::try_from_optioned(
                value.se_linux_mount,
            )?,
            storage_capacity: crate::OptionableConvert::try_from_optioned(
                value.storage_capacity,
            )?,
            token_requests: crate::OptionableConvert::try_from_optioned(
                value.token_requests,
            )?,
            volume_lifecycle_modes: crate::OptionableConvert::try_from_optioned(
                value.volume_lifecycle_modes,
            )?,
        })
    }
    fn merge(&mut self, other: CSIDriverSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.attach_required,
            other.attach_required,
        )?;
        crate::OptionableConvert::merge(
            &mut self.fs_group_policy,
            other.fs_group_policy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.node_allocatable_update_period_seconds,
            other.node_allocatable_update_period_seconds,
        )?;
        crate::OptionableConvert::merge(
            &mut self.pod_info_on_mount,
            other.pod_info_on_mount,
        )?;
        crate::OptionableConvert::merge(
            &mut self.requires_republish,
            other.requires_republish,
        )?;
        crate::OptionableConvert::merge(&mut self.se_linux_mount, other.se_linux_mount)?;
        crate::OptionableConvert::merge(
            &mut self.storage_capacity,
            other.storage_capacity,
        )?;
        crate::OptionableConvert::merge(&mut self.token_requests, other.token_requests)?;
        crate::OptionableConvert::merge(
            &mut self.volume_lifecycle_modes,
            other.volume_lifecycle_modes,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::storage::v1::CSIDriverSpec>
for CSIDriverSpecAc {
    fn from_optionable(value: k8s_openapi026::api::storage::v1::CSIDriverSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::storage::v1::CSIDriverSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::storage::v1::CSIDriverSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
