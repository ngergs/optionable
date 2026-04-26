#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PersistentVolumeClaimSpec describes the common attributes of storage devices and allows a Source for provider-specific attributes
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistentVolumeClaimSpecAc {
    /// accessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modes: Option<std::vec::Vec<std::string::String>>,
    /// dataSource field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. When the AnyVolumeDataSource feature gate is enabled, dataSource contents will be copied to dataSourceRef, and dataSourceRef contents will be copied to dataSource when dataSourceRef.namespace is not specified. If the namespace is specified, then dataSourceRef will not be copied to dataSource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<
        <::k8s_openapi027::api::core::v1::TypedLocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// dataSourceRef specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the dataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, when namespace isn't specified in dataSourceRef, both fields (dataSource and dataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. When namespace is specified in dataSourceRef, dataSource isn't set to the same value and must be empty. There are three important differences between dataSource and dataSourceRef: * While dataSource only allows two specific types of objects, dataSourceRef
    ///   allows any non-core object, as well as PersistentVolumeClaim objects.
    /// * While dataSource ignores disallowed values (dropping them), dataSourceRef
    ///   preserves all values, and generates an error if a disallowed value is
    ///   specified.
    /// * While dataSource only allows local objects, dataSourceRef allows objects
    ///   in any namespaces.
    /// (Beta) Using this field requires the AnyVolumeDataSource feature gate to be enabled. (Alpha) Using the namespace field of dataSourceRef requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_ref: Option<
        <::k8s_openapi027::api::core::v1::TypedObjectReference as crate::Optionable>::Optioned,
    >,
    /// resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<
        <::k8s_openapi027::api::core::v1::VolumeResourceRequirements as crate::Optionable>::Optioned,
    >,
    /// selector is a label query over volumes to consider for binding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// storageClassName is the name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<std::string::String>,
    /// volumeAttributesClassName may be used to set the VolumeAttributesClass used by this claim. If specified, the CSI driver will create or update the volume with the attributes defined in the corresponding VolumeAttributesClass. This has a different purpose than storageClassName, it can be changed after the claim is created. An empty string or nil value indicates that no VolumeAttributesClass will be applied to the claim. If the claim enters an Infeasible error state, this field can be reset to its previous value (including nil) to cancel the modification. If the resource referred to by volumeAttributesClass does not exist, this PersistentVolumeClaim will be set to a Pending state, as reflected by the modifyVolumeStatus field, until such as a resource exists. More info: https://kubernetes.io/docs/concepts/storage/volume-attributes-classes/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes_class_name: Option<std::string::String>,
    /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<std::string::String>,
    /// volumeName is the binding reference to the PersistentVolume backing this claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PersistentVolumeClaimSpec {
    type Optioned = PersistentVolumeClaimSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimSpecAc {
    type Optioned = PersistentVolumeClaimSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PersistentVolumeClaimSpec {
    fn into_optioned(self) -> PersistentVolumeClaimSpecAc {
        PersistentVolumeClaimSpecAc {
            access_modes: self.access_modes,
            data_source: crate::OptionableConvert::into_optioned(self.data_source),
            data_source_ref: crate::OptionableConvert::into_optioned(
                self.data_source_ref,
            ),
            resources: crate::OptionableConvert::into_optioned(self.resources),
            selector: crate::OptionableConvert::into_optioned(self.selector),
            storage_class_name: self.storage_class_name,
            volume_attributes_class_name: self.volume_attributes_class_name,
            volume_mode: self.volume_mode,
            volume_name: self.volume_name,
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            access_modes: value.access_modes,
            data_source: crate::OptionableConvert::try_from_optioned(value.data_source)?,
            data_source_ref: crate::OptionableConvert::try_from_optioned(
                value.data_source_ref,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
            storage_class_name: value.storage_class_name,
            volume_attributes_class_name: value.volume_attributes_class_name,
            volume_mode: value.volume_mode,
            volume_name: value.volume_name,
        })
    }
    fn merge(&mut self, other: PersistentVolumeClaimSpecAc) -> Result<(), crate::Error> {
        if self.access_modes.is_none() {
            self.access_modes = crate::OptionableConvert::try_from_optioned(
                other.access_modes,
            )?;
        } else if let Some(self_value) = self.access_modes.as_mut()
            && let Some(other_value) = other.access_modes
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.data_source.is_none() {
            self.data_source = crate::OptionableConvert::try_from_optioned(
                other.data_source,
            )?;
        } else if let Some(self_value) = self.data_source.as_mut()
            && let Some(other_value) = other.data_source
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.data_source_ref.is_none() {
            self.data_source_ref = crate::OptionableConvert::try_from_optioned(
                other.data_source_ref,
            )?;
        } else if let Some(self_value) = self.data_source_ref.as_mut()
            && let Some(other_value) = other.data_source_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resources.is_none() {
            self.resources = crate::OptionableConvert::try_from_optioned(
                other.resources,
            )?;
        } else if let Some(self_value) = self.resources.as_mut()
            && let Some(other_value) = other.resources
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.selector.is_none() {
            self.selector = crate::OptionableConvert::try_from_optioned(other.selector)?;
        } else if let Some(self_value) = self.selector.as_mut()
            && let Some(other_value) = other.selector
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.storage_class_name.is_none() {
            self.storage_class_name = crate::OptionableConvert::try_from_optioned(
                other.storage_class_name,
            )?;
        } else if let Some(self_value) = self.storage_class_name.as_mut()
            && let Some(other_value) = other.storage_class_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.volume_attributes_class_name.is_none() {
            self.volume_attributes_class_name = crate::OptionableConvert::try_from_optioned(
                other.volume_attributes_class_name,
            )?;
        } else if let Some(self_value) = self.volume_attributes_class_name.as_mut()
            && let Some(other_value) = other.volume_attributes_class_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.volume_mode.is_none() {
            self.volume_mode = crate::OptionableConvert::try_from_optioned(
                other.volume_mode,
            )?;
        } else if let Some(self_value) = self.volume_mode.as_mut()
            && let Some(other_value) = other.volume_mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.volume_name.is_none() {
            self.volume_name = crate::OptionableConvert::try_from_optioned(
                other.volume_name,
            )?;
        } else if let Some(self_value) = self.volume_name.as_mut()
            && let Some(other_value) = other.volume_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PersistentVolumeClaimSpec>
for PersistentVolumeClaimSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PersistentVolumeClaimSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PersistentVolumeClaimSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PersistentVolumeClaimSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PersistentVolumeClaimSpecAc {
    fn merge_from(&mut self, other: Self) {
        self.access_modes = other.access_modes;
        k8s_openapi027::DeepMerge::merge_from(&mut self.data_source, other.data_source);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.data_source_ref,
            other.data_source_ref,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.resources, other.resources);
        k8s_openapi027::DeepMerge::merge_from(&mut self.selector, other.selector);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.storage_class_name,
            other.storage_class_name,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.volume_attributes_class_name,
            other.volume_attributes_class_name,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.volume_mode, other.volume_mode);
        k8s_openapi027::DeepMerge::merge_from(&mut self.volume_name, other.volume_name);
    }
}
