pub struct PersistentVolumeClaimStatusAc {
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
    type Optioned = PersistentVolumeClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimStatusAc {
    type Optioned = PersistentVolumeClaimStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::PersistentVolumeClaimStatus {
    fn into_optioned(self) -> PersistentVolumeClaimStatusAc {
        PersistentVolumeClaimStatusAc {
            access_modes: crate::OptionableConvert::into_optioned(self.access_modes),
            allocated_resource_statuses: crate::OptionableConvert::into_optioned(
                self.allocated_resource_statuses,
            ),
            allocated_resources: crate::OptionableConvert::into_optioned(
                self.allocated_resources,
            ),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_volume_attributes_class_name: crate::OptionableConvert::into_optioned(
                self.current_volume_attributes_class_name,
            ),
            modify_volume_status: crate::OptionableConvert::into_optioned(
                self.modify_volume_status,
            ),
            phase: crate::OptionableConvert::into_optioned(self.phase),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            access_modes: crate::OptionableConvert::try_from_optioned(
                value.access_modes,
            )?,
            allocated_resource_statuses: crate::OptionableConvert::try_from_optioned(
                value.allocated_resource_statuses,
            )?,
            allocated_resources: crate::OptionableConvert::try_from_optioned(
                value.allocated_resources,
            )?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_volume_attributes_class_name: crate::OptionableConvert::try_from_optioned(
                value.current_volume_attributes_class_name,
            )?,
            modify_volume_status: crate::OptionableConvert::try_from_optioned(
                value.modify_volume_status,
            )?,
            phase: crate::OptionableConvert::try_from_optioned(value.phase)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.access_modes, other.access_modes)?;
        crate::OptionableConvert::merge(
            &mut self.allocated_resource_statuses,
            other.allocated_resource_statuses,
        )?;
        crate::OptionableConvert::merge(
            &mut self.allocated_resources,
            other.allocated_resources,
        )?;
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.current_volume_attributes_class_name,
            other.current_volume_attributes_class_name,
        )?;
        crate::OptionableConvert::merge(
            &mut self.modify_volume_status,
            other.modify_volume_status,
        )?;
        crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        Ok(())
    }
}
