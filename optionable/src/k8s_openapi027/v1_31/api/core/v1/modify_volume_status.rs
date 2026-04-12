#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ModifyVolumeStatus represents the status object of ControllerModifyVolume operation
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModifyVolumeStatusAc {
    /// status is the status of the ControllerModifyVolume operation. It can be in any of following states:
    ///  - Pending
    ///    Pending indicates that the PersistentVolumeClaim cannot be modified due to unmet requirements, such as
    ///    the specified VolumeAttributesClass not existing.
    ///  - InProgress
    ///    InProgress indicates that the volume is being modified.
    ///  - Infeasible
    ///   Infeasible indicates that the request has been rejected as invalid by the CSI driver. To
    ///       resolve the error, a valid VolumeAttributesClass needs to be specified.
    /// Note: New statuses can be added in the future. Consumers should check for unknown statuses and fail appropriately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// targetVolumeAttributesClassName is the name of the VolumeAttributesClass the PVC currently being reconciled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_volume_attributes_class_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ModifyVolumeStatus {
    type Optioned = ModifyVolumeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ModifyVolumeStatusAc {
    type Optioned = ModifyVolumeStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ModifyVolumeStatus {
    fn into_optioned(self) -> ModifyVolumeStatusAc {
        ModifyVolumeStatusAc {
            status: Some(self.status),
            target_volume_attributes_class_name: self.target_volume_attributes_class_name,
        }
    }
    fn try_from_optioned(value: ModifyVolumeStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            status: value
                .status
                .ok_or(crate::Error {
                    missing_field: "status",
                })?,
            target_volume_attributes_class_name: value
                .target_volume_attributes_class_name,
        })
    }
    fn merge(&mut self, other: ModifyVolumeStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.status {
            self.status = other_value;
        }
        if self.target_volume_attributes_class_name.is_none() {
            self.target_volume_attributes_class_name = other
                .target_volume_attributes_class_name;
        }
        if let Some(other_value) = other.target_volume_attributes_class_name {
            crate::OptionableConvert::merge(
                &mut self.target_volume_attributes_class_name,
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ModifyVolumeStatus>
for ModifyVolumeStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ModifyVolumeStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ModifyVolumeStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ModifyVolumeStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
