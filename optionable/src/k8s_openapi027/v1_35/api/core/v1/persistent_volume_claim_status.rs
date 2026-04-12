#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PersistentVolumeClaimStatus is the current status of a persistent volume claim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistentVolumeClaimStatusAc {
    /// accessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modes: Option<std::vec::Vec<std::string::String>>,
    /// allocatedResourceStatuses stores status of resource being resized for the given PVC. Key names follow standard Kubernetes label syntax. Valid values are either:
    ///     * Un-prefixed keys:
    ///         - storage - the capacity of the volume.
    ///     * Custom resources must use implementation-defined prefixed names such as "example.com/my-custom-resource"
    /// Apart from above values - keys that are unprefixed or have kubernetes.io prefix are considered reserved and hence may not be used.
    ///
    /// ClaimResourceStatus can be in any of following states:
    ///     - ControllerResizeInProgress:
    ///         State set when resize controller starts resizing the volume in control-plane.
    ///     - ControllerResizeFailed:
    ///         State set when resize has failed in resize controller with a terminal error.
    ///     - NodeResizePending:
    ///         State set when resize controller has finished resizing the volume but further resizing of
    ///         volume is needed on the node.
    ///     - NodeResizeInProgress:
    ///         State set when kubelet starts resizing the volume.
    ///     - NodeResizeFailed:
    ///         State set when resizing has failed in kubelet with a terminal error. Transient errors don't set
    ///         NodeResizeFailed.
    /// For example: if expanding a PVC for more capacity - this field can be one of the following states:
    ///     - pvc.status.allocatedResourceStatus\['storage'\] = "ControllerResizeInProgress"
    ///      - pvc.status.allocatedResourceStatus\['storage'\] = "ControllerResizeFailed"
    ///      - pvc.status.allocatedResourceStatus\['storage'\] = "NodeResizePending"
    ///      - pvc.status.allocatedResourceStatus\['storage'\] = "NodeResizeInProgress"
    ///      - pvc.status.allocatedResourceStatus\['storage'\] = "NodeResizeFailed"
    /// When this field is not set, it means that no resize operation is in progress for the given PVC.
    ///
    /// A controller that receives PVC update with previously unknown resourceName or ClaimResourceStatus should ignore the update for the purpose it was designed. For example - a controller that only is responsible for resizing capacity of the volume, should ignore PVC updates that change other valid resources associated with PVC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_resource_statuses: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// allocatedResources tracks the resources allocated to a PVC including its capacity. Key names follow standard Kubernetes label syntax. Valid values are either:
    ///     * Un-prefixed keys:
    ///         - storage - the capacity of the volume.
    ///     * Custom resources must use implementation-defined prefixed names such as "example.com/my-custom-resource"
    /// Apart from above values - keys that are unprefixed or have kubernetes.io prefix are considered reserved and hence may not be used.
    ///
    /// Capacity reported here may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity.
    ///
    /// A controller that receives PVC update with previously unknown resourceName should ignore the update for the purpose it was designed. For example - a controller that only is responsible for resizing capacity of the volume, should ignore PVC updates that change other valid resources associated with PVC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_resources: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// capacity represents the actual resources of the underlying volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// conditions is the current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'Resizing'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PersistentVolumeClaimCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// currentVolumeAttributesClassName is the current name of the VolumeAttributesClass the PVC is using. When unset, there is no VolumeAttributeClass applied to this PersistentVolumeClaim
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_volume_attributes_class_name: Option<std::string::String>,
    /// ModifyVolumeStatus represents the status object of ControllerModifyVolume operation. When this is unset, there is no ModifyVolume operation being attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_volume_status: Option<
        <::k8s_openapi027::api::core::v1::ModifyVolumeStatus as crate::Optionable>::Optioned,
    >,
    /// phase represents the current phase of PersistentVolumeClaim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PersistentVolumeClaimStatus {
    type Optioned = PersistentVolumeClaimStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimStatusAc {
    type Optioned = PersistentVolumeClaimStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PersistentVolumeClaimStatus {
    fn into_optioned(self) -> PersistentVolumeClaimStatusAc {
        PersistentVolumeClaimStatusAc {
            access_modes: self.access_modes,
            allocated_resource_statuses: self.allocated_resource_statuses,
            allocated_resources: crate::OptionableConvert::into_optioned(
                self.allocated_resources,
            ),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            current_volume_attributes_class_name: self
                .current_volume_attributes_class_name,
            modify_volume_status: crate::OptionableConvert::into_optioned(
                self.modify_volume_status,
            ),
            phase: self.phase,
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            access_modes: value.access_modes,
            allocated_resource_statuses: value.allocated_resource_statuses,
            allocated_resources: crate::OptionableConvert::try_from_optioned(
                value.allocated_resources,
            )?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            current_volume_attributes_class_name: value
                .current_volume_attributes_class_name,
            modify_volume_status: crate::OptionableConvert::try_from_optioned(
                value.modify_volume_status,
            )?,
            phase: value.phase,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimStatusAc,
    ) -> Result<(), crate::Error> {
        if self.access_modes.is_none() {
            self.access_modes = crate::OptionableConvert::try_from_optioned(
                other.access_modes,
            )?;
        } else {
            self.access_modes = crate::OptionableConvert::try_from_optioned(
                other.access_modes,
            )?;
        }
        if self.allocated_resource_statuses.is_none() {
            self.allocated_resource_statuses = crate::OptionableConvert::try_from_optioned(
                other.allocated_resource_statuses,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.allocated_resource_statuses,
                other.allocated_resource_statuses,
            )?;
        }
        if self.allocated_resources.is_none() {
            self.allocated_resources = crate::OptionableConvert::try_from_optioned(
                other.allocated_resources,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.allocated_resources,
                other.allocated_resources,
            )?;
        }
        if self.capacity.is_none() {
            self.capacity = crate::OptionableConvert::try_from_optioned(other.capacity)?;
        } else {
            crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        }
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else {
            crate::merge::try_merge_optioned_map(
                &mut self.conditions,
                other.conditions,
            )?;
        }
        if self.current_volume_attributes_class_name.is_none() {
            self.current_volume_attributes_class_name = crate::OptionableConvert::try_from_optioned(
                other.current_volume_attributes_class_name,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.current_volume_attributes_class_name,
                other.current_volume_attributes_class_name,
            )?;
        }
        if self.modify_volume_status.is_none() {
            self.modify_volume_status = crate::OptionableConvert::try_from_optioned(
                other.modify_volume_status,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.modify_volume_status,
                other.modify_volume_status,
            )?;
        }
        if self.phase.is_none() {
            self.phase = crate::OptionableConvert::try_from_optioned(other.phase)?;
        } else {
            crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PersistentVolumeClaimStatus>
for PersistentVolumeClaimStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PersistentVolumeClaimStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::PersistentVolumeClaimStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PersistentVolumeClaimStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
