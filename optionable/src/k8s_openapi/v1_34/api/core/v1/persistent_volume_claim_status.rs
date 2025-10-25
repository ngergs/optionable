pub struct PersistentVolumeClaimStatusOpt {
    pub access_modes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub allocated_resource_statuses: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub allocated_resources: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::PersistentVolumeClaimCondition>,
    > as crate::Optionable>::Optioned,
    pub current_volume_attributes_class_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub modify_volume_status: <Option<
        ::k8s_openapi::api::core::v1::ModifyVolumeStatus,
    > as crate::Optionable>::Optioned,
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PersistentVolumeClaimStatus {
    type Optioned = PersistentVolumeClaimStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimStatusOpt {
    type Optioned = PersistentVolumeClaimStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::PersistentVolumeClaimStatus {
    fn into_optioned(self) -> PersistentVolumeClaimStatusOpt {
        PersistentVolumeClaimStatusOpt {
            access_modes: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.access_modes),
            allocated_resource_statuses: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(
                self.allocated_resource_statuses,
            ),
            allocated_resources: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.allocated_resources),
            capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.capacity),
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::core::v1::PersistentVolumeClaimCondition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            current_volume_attributes_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(
                self.current_volume_attributes_class_name,
            ),
            modify_volume_status: <Option<
                ::k8s_openapi::api::core::v1::ModifyVolumeStatus,
            > as crate::OptionableConvert>::into_optioned(self.modify_volume_status),
            phase: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.phase),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            access_modes: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.access_modes)?,
            allocated_resource_statuses: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.allocated_resource_statuses,
            )?,
            allocated_resources: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(
                value.allocated_resources,
            )?,
            capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.capacity)?,
            conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::core::v1::PersistentVolumeClaimCondition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            current_volume_attributes_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.current_volume_attributes_class_name,
            )?,
            modify_volume_status: <Option<
                ::k8s_openapi::api::core::v1::ModifyVolumeStatus,
            > as crate::OptionableConvert>::try_from_optioned(
                value.modify_volume_status,
            )?,
            phase: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.phase)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.access_modes,
            other.access_modes,
        )?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.allocated_resource_statuses,
            other.allocated_resource_statuses,
        )?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.allocated_resources,
            other.allocated_resources,
        )?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.capacity, other.capacity)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::PersistentVolumeClaimCondition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.current_volume_attributes_class_name,
            other.current_volume_attributes_class_name,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::ModifyVolumeStatus,
        > as crate::OptionableConvert>::merge(
            &mut self.modify_volume_status,
            other.modify_volume_status,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.phase, other.phase)?;
        Ok(())
    }
}
