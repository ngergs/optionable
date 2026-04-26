#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PersistentVolumeSpec is the specification of a persistent volume.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistentVolumeSpecAc {
    /// accessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modes: Option<std::vec::Vec<std::string::String>>,
    /// awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Deprecated: AWSElasticBlockStore is deprecated. All operations for the in-tree awsElasticBlockStore type are redirected to the ebs.csi.aws.com CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<
        <::k8s_openapi027::api::core::v1::AWSElasticBlockStoreVolumeSource as crate::Optionable>::Optioned,
    >,
    /// azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod. Deprecated: AzureDisk is deprecated. All operations for the in-tree azureDisk type are redirected to the disk.csi.azure.com CSI driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<
        <::k8s_openapi027::api::core::v1::AzureDiskVolumeSource as crate::Optionable>::Optioned,
    >,
    /// azureFile represents an Azure File Service mount on the host and bind mount to the pod. Deprecated: AzureFile is deprecated. All operations for the in-tree azureFile type are redirected to the file.csi.azure.com CSI driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<
        <::k8s_openapi027::api::core::v1::AzureFilePersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// capacity is the description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// cephFS represents a Ceph FS mount on the host that shares a pod's lifetime. Deprecated: CephFS is deprecated and the in-tree cephfs type is no longer supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<
        <::k8s_openapi027::api::core::v1::CephFSPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// cinder represents a cinder volume attached and mounted on kubelets host machine. Deprecated: Cinder is deprecated. All operations for the in-tree cinder type are redirected to the cinder.csi.openstack.org CSI driver. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinder: Option<
        <::k8s_openapi027::api::core::v1::CinderPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// claimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_ref: Option<
        <::k8s_openapi027::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    /// csi represents storage that is handled by an external CSI driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csi: Option<
        <::k8s_openapi027::api::core::v1::CSIPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc: Option<
        <::k8s_openapi027::api::core::v1::FCVolumeSource as crate::Optionable>::Optioned,
    >,
    /// flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. Deprecated: FlexVolume is deprecated. Consider using a CSIDriver instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<
        <::k8s_openapi027::api::core::v1::FlexPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running. Deprecated: Flocker is deprecated and the in-tree flocker type is no longer supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flocker: Option<
        <::k8s_openapi027::api::core::v1::FlockerVolumeSource as crate::Optionable>::Optioned,
    >,
    /// gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. Deprecated: GCEPersistentDisk is deprecated. All operations for the in-tree gcePersistentDisk type are redirected to the pd.csi.storage.gke.io CSI driver. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<
        <::k8s_openapi027::api::core::v1::GCEPersistentDiskVolumeSource as crate::Optionable>::Optioned,
    >,
    /// glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. Deprecated: Glusterfs is deprecated and the in-tree glusterfs type is no longer supported. More info: https://examples.k8s.io/volumes/glusterfs/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<
        <::k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// hostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<
        <::k8s_openapi027::api::core::v1::HostPathVolumeSource as crate::Optionable>::Optioned,
    >,
    /// iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<
        <::k8s_openapi027::api::core::v1::ISCSIPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// local represents directly-attached storage with node affinity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local: Option<
        <::k8s_openapi027::api::core::v1::LocalVolumeSource as crate::Optionable>::Optioned,
    >,
    /// mountOptions is the list of mount options, e.g. \["ro", "soft"\]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<std::vec::Vec<std::string::String>>,
    /// nfs represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs: Option<
        <::k8s_openapi027::api::core::v1::NFSVolumeSource as crate::Optionable>::Optioned,
    >,
    /// nodeAffinity defines constraints that limit what nodes this volume can be accessed from. This field influences the scheduling of pods that use this volume. This field is mutable if MutablePVNodeAffinity feature gate is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_affinity: Option<
        <::k8s_openapi027::api::core::v1::VolumeNodeAffinity as crate::Optionable>::Optioned,
    >,
    /// persistentVolumeReclaimPolicy defines what happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_reclaim_policy: Option<std::string::String>,
    /// photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine. Deprecated: PhotonPersistentDisk is deprecated and the in-tree photonPersistentDisk type is no longer supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<
        <::k8s_openapi027::api::core::v1::PhotonPersistentDiskVolumeSource as crate::Optionable>::Optioned,
    >,
    /// portworxVolume represents a portworx volume attached and mounted on kubelets host machine. Deprecated: PortworxVolume is deprecated. All operations for the in-tree portworxVolume type are redirected to the pxd.portworx.com CSI driver when the CSIMigrationPortworx feature-gate is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<
        <::k8s_openapi027::api::core::v1::PortworxVolumeSource as crate::Optionable>::Optioned,
    >,
    /// quobyte represents a Quobyte mount on the host that shares a pod's lifetime. Deprecated: Quobyte is deprecated and the in-tree quobyte type is no longer supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<
        <::k8s_openapi027::api::core::v1::QuobyteVolumeSource as crate::Optionable>::Optioned,
    >,
    /// rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. Deprecated: RBD is deprecated and the in-tree rbd type is no longer supported. More info: https://examples.k8s.io/volumes/rbd/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbd: Option<
        <::k8s_openapi027::api::core::v1::RBDPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes. Deprecated: ScaleIO is deprecated and the in-tree scaleIO type is no longer supported.
    #[serde(rename = "scaleIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<
        <::k8s_openapi027::api::core::v1::ScaleIOPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// storageClassName is the name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_name: Option<std::string::String>,
    /// storageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod. Deprecated: StorageOS is deprecated and the in-tree storageos type is no longer supported. More info: https://examples.k8s.io/volumes/storageos/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageos: Option<
        <::k8s_openapi027::api::core::v1::StorageOSPersistentVolumeSource as crate::Optionable>::Optioned,
    >,
    /// Name of VolumeAttributesClass to which this persistent volume belongs. Empty value is not allowed. When this field is not set, it indicates that this volume does not belong to any VolumeAttributesClass. This field is mutable and can be changed by the CSI driver after a volume has been updated successfully to a new class. For an unbound PersistentVolume, the volumeAttributesClassName will be matched with unbound PersistentVolumeClaims during the binding process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes_class_name: Option<std::string::String>,
    /// volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<std::string::String>,
    /// vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine. Deprecated: VsphereVolume is deprecated. All operations for the in-tree vsphereVolume type are redirected to the csi.vsphere.vmware.com CSI driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<
        <::k8s_openapi027::api::core::v1::VsphereVirtualDiskVolumeSource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PersistentVolumeSpec {
    type Optioned = PersistentVolumeSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeSpecAc {
    type Optioned = PersistentVolumeSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PersistentVolumeSpec {
    fn into_optioned(self) -> PersistentVolumeSpecAc {
        PersistentVolumeSpecAc {
            access_modes: self.access_modes,
            aws_elastic_block_store: crate::OptionableConvert::into_optioned(
                self.aws_elastic_block_store,
            ),
            azure_disk: crate::OptionableConvert::into_optioned(self.azure_disk),
            azure_file: crate::OptionableConvert::into_optioned(self.azure_file),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            cephfs: crate::OptionableConvert::into_optioned(self.cephfs),
            cinder: crate::OptionableConvert::into_optioned(self.cinder),
            claim_ref: crate::OptionableConvert::into_optioned(self.claim_ref),
            csi: crate::OptionableConvert::into_optioned(self.csi),
            fc: crate::OptionableConvert::into_optioned(self.fc),
            flex_volume: crate::OptionableConvert::into_optioned(self.flex_volume),
            flocker: crate::OptionableConvert::into_optioned(self.flocker),
            gce_persistent_disk: crate::OptionableConvert::into_optioned(
                self.gce_persistent_disk,
            ),
            glusterfs: crate::OptionableConvert::into_optioned(self.glusterfs),
            host_path: crate::OptionableConvert::into_optioned(self.host_path),
            iscsi: crate::OptionableConvert::into_optioned(self.iscsi),
            local: crate::OptionableConvert::into_optioned(self.local),
            mount_options: self.mount_options,
            nfs: crate::OptionableConvert::into_optioned(self.nfs),
            node_affinity: crate::OptionableConvert::into_optioned(self.node_affinity),
            persistent_volume_reclaim_policy: self.persistent_volume_reclaim_policy,
            photon_persistent_disk: crate::OptionableConvert::into_optioned(
                self.photon_persistent_disk,
            ),
            portworx_volume: crate::OptionableConvert::into_optioned(
                self.portworx_volume,
            ),
            quobyte: crate::OptionableConvert::into_optioned(self.quobyte),
            rbd: crate::OptionableConvert::into_optioned(self.rbd),
            scale_io: crate::OptionableConvert::into_optioned(self.scale_io),
            storage_class_name: self.storage_class_name,
            storageos: crate::OptionableConvert::into_optioned(self.storageos),
            volume_attributes_class_name: self.volume_attributes_class_name,
            volume_mode: self.volume_mode,
            vsphere_volume: crate::OptionableConvert::into_optioned(self.vsphere_volume),
        }
    }
    fn try_from_optioned(value: PersistentVolumeSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            access_modes: value.access_modes,
            aws_elastic_block_store: crate::OptionableConvert::try_from_optioned(
                value.aws_elastic_block_store,
            )?,
            azure_disk: crate::OptionableConvert::try_from_optioned(value.azure_disk)?,
            azure_file: crate::OptionableConvert::try_from_optioned(value.azure_file)?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
            cephfs: crate::OptionableConvert::try_from_optioned(value.cephfs)?,
            cinder: crate::OptionableConvert::try_from_optioned(value.cinder)?,
            claim_ref: crate::OptionableConvert::try_from_optioned(value.claim_ref)?,
            csi: crate::OptionableConvert::try_from_optioned(value.csi)?,
            fc: crate::OptionableConvert::try_from_optioned(value.fc)?,
            flex_volume: crate::OptionableConvert::try_from_optioned(value.flex_volume)?,
            flocker: crate::OptionableConvert::try_from_optioned(value.flocker)?,
            gce_persistent_disk: crate::OptionableConvert::try_from_optioned(
                value.gce_persistent_disk,
            )?,
            glusterfs: crate::OptionableConvert::try_from_optioned(value.glusterfs)?,
            host_path: crate::OptionableConvert::try_from_optioned(value.host_path)?,
            iscsi: crate::OptionableConvert::try_from_optioned(value.iscsi)?,
            local: crate::OptionableConvert::try_from_optioned(value.local)?,
            mount_options: value.mount_options,
            nfs: crate::OptionableConvert::try_from_optioned(value.nfs)?,
            node_affinity: crate::OptionableConvert::try_from_optioned(
                value.node_affinity,
            )?,
            persistent_volume_reclaim_policy: value.persistent_volume_reclaim_policy,
            photon_persistent_disk: crate::OptionableConvert::try_from_optioned(
                value.photon_persistent_disk,
            )?,
            portworx_volume: crate::OptionableConvert::try_from_optioned(
                value.portworx_volume,
            )?,
            quobyte: crate::OptionableConvert::try_from_optioned(value.quobyte)?,
            rbd: crate::OptionableConvert::try_from_optioned(value.rbd)?,
            scale_io: crate::OptionableConvert::try_from_optioned(value.scale_io)?,
            storage_class_name: value.storage_class_name,
            storageos: crate::OptionableConvert::try_from_optioned(value.storageos)?,
            volume_attributes_class_name: value.volume_attributes_class_name,
            volume_mode: value.volume_mode,
            vsphere_volume: crate::OptionableConvert::try_from_optioned(
                value.vsphere_volume,
            )?,
        })
    }
    fn merge(&mut self, other: PersistentVolumeSpecAc) -> Result<(), crate::Error> {
        if self.access_modes.is_none() {
            self.access_modes = crate::OptionableConvert::try_from_optioned(
                other.access_modes,
            )?;
        } else if let Some(self_value) = self.access_modes.as_mut()
            && let Some(other_value) = other.access_modes
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.aws_elastic_block_store.is_none() {
            self.aws_elastic_block_store = crate::OptionableConvert::try_from_optioned(
                other.aws_elastic_block_store,
            )?;
        } else if let Some(self_value) = self.aws_elastic_block_store.as_mut()
            && let Some(other_value) = other.aws_elastic_block_store
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.azure_disk.is_none() {
            self.azure_disk = crate::OptionableConvert::try_from_optioned(
                other.azure_disk,
            )?;
        } else if let Some(self_value) = self.azure_disk.as_mut()
            && let Some(other_value) = other.azure_disk
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.azure_file.is_none() {
            self.azure_file = crate::OptionableConvert::try_from_optioned(
                other.azure_file,
            )?;
        } else if let Some(self_value) = self.azure_file.as_mut()
            && let Some(other_value) = other.azure_file
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.capacity.is_none() {
            self.capacity = crate::OptionableConvert::try_from_optioned(other.capacity)?;
        } else if let Some(self_value) = self.capacity.as_mut()
            && let Some(other_value) = other.capacity
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.cephfs.is_none() {
            self.cephfs = crate::OptionableConvert::try_from_optioned(other.cephfs)?;
        } else if let Some(self_value) = self.cephfs.as_mut()
            && let Some(other_value) = other.cephfs
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.cinder.is_none() {
            self.cinder = crate::OptionableConvert::try_from_optioned(other.cinder)?;
        } else if let Some(self_value) = self.cinder.as_mut()
            && let Some(other_value) = other.cinder
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.claim_ref.is_none() {
            self.claim_ref = crate::OptionableConvert::try_from_optioned(
                other.claim_ref,
            )?;
        } else if let Some(self_value) = self.claim_ref.as_mut()
            && let Some(other_value) = other.claim_ref
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.csi.is_none() {
            self.csi = crate::OptionableConvert::try_from_optioned(other.csi)?;
        } else if let Some(self_value) = self.csi.as_mut()
            && let Some(other_value) = other.csi
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.fc.is_none() {
            self.fc = crate::OptionableConvert::try_from_optioned(other.fc)?;
        } else if let Some(self_value) = self.fc.as_mut()
            && let Some(other_value) = other.fc
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.flex_volume.is_none() {
            self.flex_volume = crate::OptionableConvert::try_from_optioned(
                other.flex_volume,
            )?;
        } else if let Some(self_value) = self.flex_volume.as_mut()
            && let Some(other_value) = other.flex_volume
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.flocker.is_none() {
            self.flocker = crate::OptionableConvert::try_from_optioned(other.flocker)?;
        } else if let Some(self_value) = self.flocker.as_mut()
            && let Some(other_value) = other.flocker
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.gce_persistent_disk.is_none() {
            self.gce_persistent_disk = crate::OptionableConvert::try_from_optioned(
                other.gce_persistent_disk,
            )?;
        } else if let Some(self_value) = self.gce_persistent_disk.as_mut()
            && let Some(other_value) = other.gce_persistent_disk
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.glusterfs.is_none() {
            self.glusterfs = crate::OptionableConvert::try_from_optioned(
                other.glusterfs,
            )?;
        } else if let Some(self_value) = self.glusterfs.as_mut()
            && let Some(other_value) = other.glusterfs
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.host_path.is_none() {
            self.host_path = crate::OptionableConvert::try_from_optioned(
                other.host_path,
            )?;
        } else if let Some(self_value) = self.host_path.as_mut()
            && let Some(other_value) = other.host_path
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.iscsi.is_none() {
            self.iscsi = crate::OptionableConvert::try_from_optioned(other.iscsi)?;
        } else if let Some(self_value) = self.iscsi.as_mut()
            && let Some(other_value) = other.iscsi
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.local.is_none() {
            self.local = crate::OptionableConvert::try_from_optioned(other.local)?;
        } else if let Some(self_value) = self.local.as_mut()
            && let Some(other_value) = other.local
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.mount_options.is_none() {
            self.mount_options = crate::OptionableConvert::try_from_optioned(
                other.mount_options,
            )?;
        } else if let Some(self_value) = self.mount_options.as_mut()
            && let Some(other_value) = other.mount_options
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.nfs.is_none() {
            self.nfs = crate::OptionableConvert::try_from_optioned(other.nfs)?;
        } else if let Some(self_value) = self.nfs.as_mut()
            && let Some(other_value) = other.nfs
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.node_affinity.is_none() {
            self.node_affinity = crate::OptionableConvert::try_from_optioned(
                other.node_affinity,
            )?;
        } else if let Some(self_value) = self.node_affinity.as_mut()
            && let Some(other_value) = other.node_affinity
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.persistent_volume_reclaim_policy.is_none() {
            self.persistent_volume_reclaim_policy = crate::OptionableConvert::try_from_optioned(
                other.persistent_volume_reclaim_policy,
            )?;
        } else if let Some(self_value) = self.persistent_volume_reclaim_policy.as_mut()
            && let Some(other_value) = other.persistent_volume_reclaim_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.photon_persistent_disk.is_none() {
            self.photon_persistent_disk = crate::OptionableConvert::try_from_optioned(
                other.photon_persistent_disk,
            )?;
        } else if let Some(self_value) = self.photon_persistent_disk.as_mut()
            && let Some(other_value) = other.photon_persistent_disk
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.portworx_volume.is_none() {
            self.portworx_volume = crate::OptionableConvert::try_from_optioned(
                other.portworx_volume,
            )?;
        } else if let Some(self_value) = self.portworx_volume.as_mut()
            && let Some(other_value) = other.portworx_volume
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.quobyte.is_none() {
            self.quobyte = crate::OptionableConvert::try_from_optioned(other.quobyte)?;
        } else if let Some(self_value) = self.quobyte.as_mut()
            && let Some(other_value) = other.quobyte
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.rbd.is_none() {
            self.rbd = crate::OptionableConvert::try_from_optioned(other.rbd)?;
        } else if let Some(self_value) = self.rbd.as_mut()
            && let Some(other_value) = other.rbd
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.scale_io.is_none() {
            self.scale_io = crate::OptionableConvert::try_from_optioned(other.scale_io)?;
        } else if let Some(self_value) = self.scale_io.as_mut()
            && let Some(other_value) = other.scale_io
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
        if self.storageos.is_none() {
            self.storageos = crate::OptionableConvert::try_from_optioned(
                other.storageos,
            )?;
        } else if let Some(self_value) = self.storageos.as_mut()
            && let Some(other_value) = other.storageos
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
        if self.vsphere_volume.is_none() {
            self.vsphere_volume = crate::OptionableConvert::try_from_optioned(
                other.vsphere_volume,
            )?;
        } else if let Some(self_value) = self.vsphere_volume.as_mut()
            && let Some(other_value) = other.vsphere_volume
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PersistentVolumeSpec>
for PersistentVolumeSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PersistentVolumeSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PersistentVolumeSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PersistentVolumeSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for PersistentVolumeSpecAc {
    fn merge_from(&mut self, other: Self) {
        self.access_modes = other.access_modes;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.aws_elastic_block_store,
            other.aws_elastic_block_store,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.azure_disk, other.azure_disk);
        k8s_openapi027::DeepMerge::merge_from(&mut self.azure_file, other.azure_file);
        k8s_openapi027::DeepMerge::merge_from(&mut self.capacity, other.capacity);
        k8s_openapi027::DeepMerge::merge_from(&mut self.cephfs, other.cephfs);
        k8s_openapi027::DeepMerge::merge_from(&mut self.cinder, other.cinder);
        k8s_openapi027::DeepMerge::merge_from(&mut self.claim_ref, other.claim_ref);
        k8s_openapi027::DeepMerge::merge_from(&mut self.csi, other.csi);
        k8s_openapi027::DeepMerge::merge_from(&mut self.fc, other.fc);
        k8s_openapi027::DeepMerge::merge_from(&mut self.flex_volume, other.flex_volume);
        k8s_openapi027::DeepMerge::merge_from(&mut self.flocker, other.flocker);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.gce_persistent_disk,
            other.gce_persistent_disk,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.glusterfs, other.glusterfs);
        k8s_openapi027::DeepMerge::merge_from(&mut self.host_path, other.host_path);
        k8s_openapi027::DeepMerge::merge_from(&mut self.iscsi, other.iscsi);
        k8s_openapi027::DeepMerge::merge_from(&mut self.local, other.local);
        self.mount_options = other.mount_options;
        k8s_openapi027::DeepMerge::merge_from(&mut self.nfs, other.nfs);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.node_affinity,
            other.node_affinity,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.persistent_volume_reclaim_policy,
            other.persistent_volume_reclaim_policy,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.photon_persistent_disk,
            other.photon_persistent_disk,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.portworx_volume,
            other.portworx_volume,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.quobyte, other.quobyte);
        k8s_openapi027::DeepMerge::merge_from(&mut self.rbd, other.rbd);
        k8s_openapi027::DeepMerge::merge_from(&mut self.scale_io, other.scale_io);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.storage_class_name,
            other.storage_class_name,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.storageos, other.storageos);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.volume_attributes_class_name,
            other.volume_attributes_class_name,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.volume_mode, other.volume_mode);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.vsphere_volume,
            other.vsphere_volume,
        );
    }
}
