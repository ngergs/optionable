#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Volume represents a named volume in a pod that may be accessed by any container in the pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeAc {
    /// awsElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: Option<
        <::k8s_openapi027::api::core::v1::AWSElasticBlockStoreVolumeSource as crate::Optionable>::Optioned,
    >,
    /// azureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<
        <::k8s_openapi027::api::core::v1::AzureDiskVolumeSource as crate::Optionable>::Optioned,
    >,
    /// azureFile represents an Azure File Service mount on the host and bind mount to the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<
        <::k8s_openapi027::api::core::v1::AzureFileVolumeSource as crate::Optionable>::Optioned,
    >,
    /// cephFS represents a Ceph FS mount on the host that shares a pod's lifetime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cephfs: Option<
        <::k8s_openapi027::api::core::v1::CephFSVolumeSource as crate::Optionable>::Optioned,
    >,
    /// cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinder: Option<
        <::k8s_openapi027::api::core::v1::CinderVolumeSource as crate::Optionable>::Optioned,
    >,
    /// configMap represents a configMap that should populate this volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: Option<
        <::k8s_openapi027::api::core::v1::ConfigMapVolumeSource as crate::Optionable>::Optioned,
    >,
    /// csi (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csi: Option<
        <::k8s_openapi027::api::core::v1::CSIVolumeSource as crate::Optionable>::Optioned,
    >,
    /// downwardAPI represents downward API about the pod that should populate this volume
    #[serde(rename = "downwardAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downward_api: Option<
        <::k8s_openapi027::api::core::v1::DownwardAPIVolumeSource as crate::Optionable>::Optioned,
    >,
    /// emptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<
        <::k8s_openapi027::api::core::v1::EmptyDirVolumeSource as crate::Optionable>::Optioned,
    >,
    /// ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.
    ///
    /// Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity
    ///    tracking are needed,
    /// c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through
    ///    a PersistentVolumeClaim (see EphemeralVolumeSource for more
    ///    information on the connection between this volume type
    ///    and PersistentVolumeClaim).
    ///
    /// Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.
    ///
    /// Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.
    ///
    /// A pod can use both types of ephemeral volumes and persistent volumes at the same time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<
        <::k8s_openapi027::api::core::v1::EphemeralVolumeSource as crate::Optionable>::Optioned,
    >,
    /// fc represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc: Option<
        <::k8s_openapi027::api::core::v1::FCVolumeSource as crate::Optionable>::Optioned,
    >,
    /// flexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume: Option<
        <::k8s_openapi027::api::core::v1::FlexVolumeSource as crate::Optionable>::Optioned,
    >,
    /// flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flocker: Option<
        <::k8s_openapi027::api::core::v1::FlockerVolumeSource as crate::Optionable>::Optioned,
    >,
    /// gcePersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: Option<
        <::k8s_openapi027::api::core::v1::GCEPersistentDiskVolumeSource as crate::Optionable>::Optioned,
    >,
    /// gitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<
        <::k8s_openapi027::api::core::v1::GitRepoVolumeSource as crate::Optionable>::Optioned,
    >,
    /// glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glusterfs: Option<
        <::k8s_openapi027::api::core::v1::GlusterfsVolumeSource as crate::Optionable>::Optioned,
    >,
    /// hostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<
        <::k8s_openapi027::api::core::v1::HostPathVolumeSource as crate::Optionable>::Optioned,
    >,
    /// image represents an OCI object (a container image or artifact) pulled and mounted on the kubelet's host machine. The volume is resolved at pod startup depending on which PullPolicy value is provided:
    ///
    /// - Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. - Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. - IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails.
    ///
    /// The volume gets re-resolved if the pod gets deleted and recreated, which means that new remote content will become available on pod recreation. A failure to resolve or pull the image during pod startup will block containers from starting and may add significant latency. Failures will be retried using normal volume backoff and will be reported on the pod reason and message. The types of objects that may be mounted by this volume are defined by the container runtime implementation on a host machine and at minimum must include all valid types supported by the container image field. The OCI object gets mounted in a single directory (spec.containers\[*\].volumeMounts.mountPath) by merging the manifest layers in the same way as for container images. The volume will be mounted read-only (ro) and non-executable files (noexec). Sub path mounts for containers are not supported (spec.containers\[*\].volumeMounts.subpath). The field spec.securityContext.fsGroupChangePolicy has no effect on this volume type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<
        <::k8s_openapi027::api::core::v1::ImageVolumeSource as crate::Optionable>::Optioned,
    >,
    /// iscsi represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<
        <::k8s_openapi027::api::core::v1::ISCSIVolumeSource as crate::Optionable>::Optioned,
    >,
    /// name of the volume. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub name: std::string::String,
    /// nfs represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs: Option<
        <::k8s_openapi027::api::core::v1::NFSVolumeSource as crate::Optionable>::Optioned,
    >,
    /// persistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: Option<
        <::k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource as crate::Optionable>::Optioned,
    >,
    /// photonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: Option<
        <::k8s_openapi027::api::core::v1::PhotonPersistentDiskVolumeSource as crate::Optionable>::Optioned,
    >,
    /// portworxVolume represents a portworx volume attached and mounted on kubelets host machine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portworx_volume: Option<
        <::k8s_openapi027::api::core::v1::PortworxVolumeSource as crate::Optionable>::Optioned,
    >,
    /// projected items for all in one resources secrets, configmaps, and downward API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected: Option<
        <::k8s_openapi027::api::core::v1::ProjectedVolumeSource as crate::Optionable>::Optioned,
    >,
    /// quobyte represents a Quobyte mount on the host that shares a pod's lifetime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quobyte: Option<
        <::k8s_openapi027::api::core::v1::QuobyteVolumeSource as crate::Optionable>::Optioned,
    >,
    /// rbd represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbd: Option<
        <::k8s_openapi027::api::core::v1::RBDVolumeSource as crate::Optionable>::Optioned,
    >,
    /// scaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
    #[serde(rename = "scaleIO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_io: Option<
        <::k8s_openapi027::api::core::v1::ScaleIOVolumeSource as crate::Optionable>::Optioned,
    >,
    /// secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<
        <::k8s_openapi027::api::core::v1::SecretVolumeSource as crate::Optionable>::Optioned,
    >,
    /// storageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageos: Option<
        <::k8s_openapi027::api::core::v1::StorageOSVolumeSource as crate::Optionable>::Optioned,
    >,
    /// vsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: Option<
        <::k8s_openapi027::api::core::v1::VsphereVirtualDiskVolumeSource as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Volume {
    type Optioned = VolumeAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAc {
    type Optioned = VolumeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Volume {
    fn into_optioned(self) -> VolumeAc {
        VolumeAc {
            aws_elastic_block_store: crate::OptionableConvert::into_optioned(
                self.aws_elastic_block_store,
            ),
            azure_disk: crate::OptionableConvert::into_optioned(self.azure_disk),
            azure_file: crate::OptionableConvert::into_optioned(self.azure_file),
            cephfs: crate::OptionableConvert::into_optioned(self.cephfs),
            cinder: crate::OptionableConvert::into_optioned(self.cinder),
            config_map: crate::OptionableConvert::into_optioned(self.config_map),
            csi: crate::OptionableConvert::into_optioned(self.csi),
            downward_api: crate::OptionableConvert::into_optioned(self.downward_api),
            empty_dir: crate::OptionableConvert::into_optioned(self.empty_dir),
            ephemeral: crate::OptionableConvert::into_optioned(self.ephemeral),
            fc: crate::OptionableConvert::into_optioned(self.fc),
            flex_volume: crate::OptionableConvert::into_optioned(self.flex_volume),
            flocker: crate::OptionableConvert::into_optioned(self.flocker),
            gce_persistent_disk: crate::OptionableConvert::into_optioned(
                self.gce_persistent_disk,
            ),
            git_repo: crate::OptionableConvert::into_optioned(self.git_repo),
            glusterfs: crate::OptionableConvert::into_optioned(self.glusterfs),
            host_path: crate::OptionableConvert::into_optioned(self.host_path),
            image: crate::OptionableConvert::into_optioned(self.image),
            iscsi: crate::OptionableConvert::into_optioned(self.iscsi),
            name: self.name,
            nfs: crate::OptionableConvert::into_optioned(self.nfs),
            persistent_volume_claim: crate::OptionableConvert::into_optioned(
                self.persistent_volume_claim,
            ),
            photon_persistent_disk: crate::OptionableConvert::into_optioned(
                self.photon_persistent_disk,
            ),
            portworx_volume: crate::OptionableConvert::into_optioned(
                self.portworx_volume,
            ),
            projected: crate::OptionableConvert::into_optioned(self.projected),
            quobyte: crate::OptionableConvert::into_optioned(self.quobyte),
            rbd: crate::OptionableConvert::into_optioned(self.rbd),
            scale_io: crate::OptionableConvert::into_optioned(self.scale_io),
            secret: crate::OptionableConvert::into_optioned(self.secret),
            storageos: crate::OptionableConvert::into_optioned(self.storageos),
            vsphere_volume: crate::OptionableConvert::into_optioned(self.vsphere_volume),
        }
    }
    fn try_from_optioned(value: VolumeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            aws_elastic_block_store: crate::OptionableConvert::try_from_optioned(
                value.aws_elastic_block_store,
            )?,
            azure_disk: crate::OptionableConvert::try_from_optioned(value.azure_disk)?,
            azure_file: crate::OptionableConvert::try_from_optioned(value.azure_file)?,
            cephfs: crate::OptionableConvert::try_from_optioned(value.cephfs)?,
            cinder: crate::OptionableConvert::try_from_optioned(value.cinder)?,
            config_map: crate::OptionableConvert::try_from_optioned(value.config_map)?,
            csi: crate::OptionableConvert::try_from_optioned(value.csi)?,
            downward_api: crate::OptionableConvert::try_from_optioned(
                value.downward_api,
            )?,
            empty_dir: crate::OptionableConvert::try_from_optioned(value.empty_dir)?,
            ephemeral: crate::OptionableConvert::try_from_optioned(value.ephemeral)?,
            fc: crate::OptionableConvert::try_from_optioned(value.fc)?,
            flex_volume: crate::OptionableConvert::try_from_optioned(value.flex_volume)?,
            flocker: crate::OptionableConvert::try_from_optioned(value.flocker)?,
            gce_persistent_disk: crate::OptionableConvert::try_from_optioned(
                value.gce_persistent_disk,
            )?,
            git_repo: crate::OptionableConvert::try_from_optioned(value.git_repo)?,
            glusterfs: crate::OptionableConvert::try_from_optioned(value.glusterfs)?,
            host_path: crate::OptionableConvert::try_from_optioned(value.host_path)?,
            image: crate::OptionableConvert::try_from_optioned(value.image)?,
            iscsi: crate::OptionableConvert::try_from_optioned(value.iscsi)?,
            name: value.name,
            nfs: crate::OptionableConvert::try_from_optioned(value.nfs)?,
            persistent_volume_claim: crate::OptionableConvert::try_from_optioned(
                value.persistent_volume_claim,
            )?,
            photon_persistent_disk: crate::OptionableConvert::try_from_optioned(
                value.photon_persistent_disk,
            )?,
            portworx_volume: crate::OptionableConvert::try_from_optioned(
                value.portworx_volume,
            )?,
            projected: crate::OptionableConvert::try_from_optioned(value.projected)?,
            quobyte: crate::OptionableConvert::try_from_optioned(value.quobyte)?,
            rbd: crate::OptionableConvert::try_from_optioned(value.rbd)?,
            scale_io: crate::OptionableConvert::try_from_optioned(value.scale_io)?,
            secret: crate::OptionableConvert::try_from_optioned(value.secret)?,
            storageos: crate::OptionableConvert::try_from_optioned(value.storageos)?,
            vsphere_volume: crate::OptionableConvert::try_from_optioned(
                value.vsphere_volume,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeAc) -> Result<(), crate::Error> {
        if self.aws_elastic_block_store.is_none() {
            self.aws_elastic_block_store = other.aws_elastic_block_store;
        }
        if let Some(other_value) = other.aws_elastic_block_store {
            crate::OptionableConvert::merge(
                &mut self.aws_elastic_block_store,
                other_value,
            )?;
        }
        if self.azure_disk.is_none() {
            self.azure_disk = other.azure_disk;
        }
        if let Some(other_value) = other.azure_disk {
            crate::OptionableConvert::merge(&mut self.azure_disk, other_value)?;
        }
        if self.azure_file.is_none() {
            self.azure_file = other.azure_file;
        }
        if let Some(other_value) = other.azure_file {
            crate::OptionableConvert::merge(&mut self.azure_file, other_value)?;
        }
        if self.cephfs.is_none() {
            self.cephfs = other.cephfs;
        }
        if let Some(other_value) = other.cephfs {
            crate::OptionableConvert::merge(&mut self.cephfs, other_value)?;
        }
        if self.cinder.is_none() {
            self.cinder = other.cinder;
        }
        if let Some(other_value) = other.cinder {
            crate::OptionableConvert::merge(&mut self.cinder, other_value)?;
        }
        if self.config_map.is_none() {
            self.config_map = other.config_map;
        }
        if let Some(other_value) = other.config_map {
            crate::OptionableConvert::merge(&mut self.config_map, other_value)?;
        }
        if self.csi.is_none() {
            self.csi = other.csi;
        }
        if let Some(other_value) = other.csi {
            crate::OptionableConvert::merge(&mut self.csi, other_value)?;
        }
        if self.downward_api.is_none() {
            self.downward_api = other.downward_api;
        }
        if let Some(other_value) = other.downward_api {
            crate::OptionableConvert::merge(&mut self.downward_api, other_value)?;
        }
        if self.empty_dir.is_none() {
            self.empty_dir = other.empty_dir;
        }
        if let Some(other_value) = other.empty_dir {
            crate::OptionableConvert::merge(&mut self.empty_dir, other_value)?;
        }
        if self.ephemeral.is_none() {
            self.ephemeral = other.ephemeral;
        }
        if let Some(other_value) = other.ephemeral {
            crate::OptionableConvert::merge(&mut self.ephemeral, other_value)?;
        }
        if self.fc.is_none() {
            self.fc = other.fc;
        }
        if let Some(other_value) = other.fc {
            crate::OptionableConvert::merge(&mut self.fc, other_value)?;
        }
        if self.flex_volume.is_none() {
            self.flex_volume = other.flex_volume;
        }
        if let Some(other_value) = other.flex_volume {
            crate::OptionableConvert::merge(&mut self.flex_volume, other_value)?;
        }
        if self.flocker.is_none() {
            self.flocker = other.flocker;
        }
        if let Some(other_value) = other.flocker {
            crate::OptionableConvert::merge(&mut self.flocker, other_value)?;
        }
        if self.gce_persistent_disk.is_none() {
            self.gce_persistent_disk = other.gce_persistent_disk;
        }
        if let Some(other_value) = other.gce_persistent_disk {
            crate::OptionableConvert::merge(&mut self.gce_persistent_disk, other_value)?;
        }
        if self.git_repo.is_none() {
            self.git_repo = other.git_repo;
        }
        if let Some(other_value) = other.git_repo {
            crate::OptionableConvert::merge(&mut self.git_repo, other_value)?;
        }
        if self.glusterfs.is_none() {
            self.glusterfs = other.glusterfs;
        }
        if let Some(other_value) = other.glusterfs {
            crate::OptionableConvert::merge(&mut self.glusterfs, other_value)?;
        }
        if self.host_path.is_none() {
            self.host_path = other.host_path;
        }
        if let Some(other_value) = other.host_path {
            crate::OptionableConvert::merge(&mut self.host_path, other_value)?;
        }
        if self.image.is_none() {
            self.image = other.image;
        }
        if let Some(other_value) = other.image {
            crate::OptionableConvert::merge(&mut self.image, other_value)?;
        }
        if self.iscsi.is_none() {
            self.iscsi = other.iscsi;
        }
        if let Some(other_value) = other.iscsi {
            crate::OptionableConvert::merge(&mut self.iscsi, other_value)?;
        }
        self.name = other.name;
        if self.nfs.is_none() {
            self.nfs = other.nfs;
        }
        if let Some(other_value) = other.nfs {
            crate::OptionableConvert::merge(&mut self.nfs, other_value)?;
        }
        if self.persistent_volume_claim.is_none() {
            self.persistent_volume_claim = other.persistent_volume_claim;
        }
        if let Some(other_value) = other.persistent_volume_claim {
            crate::OptionableConvert::merge(
                &mut self.persistent_volume_claim,
                other_value,
            )?;
        }
        if self.photon_persistent_disk.is_none() {
            self.photon_persistent_disk = other.photon_persistent_disk;
        }
        if let Some(other_value) = other.photon_persistent_disk {
            crate::OptionableConvert::merge(
                &mut self.photon_persistent_disk,
                other_value,
            )?;
        }
        if self.portworx_volume.is_none() {
            self.portworx_volume = other.portworx_volume;
        }
        if let Some(other_value) = other.portworx_volume {
            crate::OptionableConvert::merge(&mut self.portworx_volume, other_value)?;
        }
        if self.projected.is_none() {
            self.projected = other.projected;
        }
        if let Some(other_value) = other.projected {
            crate::OptionableConvert::merge(&mut self.projected, other_value)?;
        }
        if self.quobyte.is_none() {
            self.quobyte = other.quobyte;
        }
        if let Some(other_value) = other.quobyte {
            crate::OptionableConvert::merge(&mut self.quobyte, other_value)?;
        }
        if self.rbd.is_none() {
            self.rbd = other.rbd;
        }
        if let Some(other_value) = other.rbd {
            crate::OptionableConvert::merge(&mut self.rbd, other_value)?;
        }
        if self.scale_io.is_none() {
            self.scale_io = other.scale_io;
        }
        if let Some(other_value) = other.scale_io {
            crate::OptionableConvert::merge(&mut self.scale_io, other_value)?;
        }
        if self.secret.is_none() {
            self.secret = other.secret;
        }
        if let Some(other_value) = other.secret {
            crate::OptionableConvert::merge(&mut self.secret, other_value)?;
        }
        if self.storageos.is_none() {
            self.storageos = other.storageos;
        }
        if let Some(other_value) = other.storageos {
            crate::OptionableConvert::merge(&mut self.storageos, other_value)?;
        }
        if self.vsphere_volume.is_none() {
            self.vsphere_volume = other.vsphere_volume;
        }
        if let Some(other_value) = other.vsphere_volume {
            crate::OptionableConvert::merge(&mut self.vsphere_volume, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::Volume {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Volume> for VolumeAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Volume) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Volume, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Volume,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
