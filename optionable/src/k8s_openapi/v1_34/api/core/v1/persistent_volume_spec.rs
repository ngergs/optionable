#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elastic_block_store: <Option<
        ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_disk: <Option<
        ::k8s_openapi::api::core::v1::AzureDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_file: <Option<
        ::k8s_openapi::api::core::v1::AzureFilePersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cephfs: <Option<
        ::k8s_openapi::api::core::v1::CephFSPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinder: <Option<
        ::k8s_openapi::api::core::v1::CinderPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csi: <Option<
        ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc: <Option<
        ::k8s_openapi::api::core::v1::FCVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume: <Option<
        ::k8s_openapi::api::core::v1::FlexPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flocker: <Option<
        ::k8s_openapi::api::core::v1::FlockerVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gce_persistent_disk: <Option<
        ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glusterfs: <Option<
        ::k8s_openapi::api::core::v1::GlusterfsPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: <Option<
        ::k8s_openapi::api::core::v1::HostPathVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi: <Option<
        ::k8s_openapi::api::core::v1::ISCSIPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local: <Option<
        ::k8s_openapi::api::core::v1::LocalVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs: <Option<
        ::k8s_openapi::api::core::v1::NFSVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_affinity: <Option<
        ::k8s_openapi::api::core::v1::VolumeNodeAffinity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_reclaim_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photon_persistent_disk: <Option<
        ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portworx_volume: <Option<
        ::k8s_openapi::api::core::v1::PortworxVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quobyte: <Option<
        ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbd: <Option<
        ::k8s_openapi::api::core::v1::RBDPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_io: <Option<
        ::k8s_openapi::api::core::v1::ScaleIOPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageos: <Option<
        ::k8s_openapi::api::core::v1::StorageOSPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes_class_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: <Option<
        ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PersistentVolumeSpec {
    type Optioned = PersistentVolumeSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeSpecAc {
    type Optioned = PersistentVolumeSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PersistentVolumeSpec {
    fn into_optioned(self) -> PersistentVolumeSpecAc {
        PersistentVolumeSpecAc {
            access_modes: crate::OptionableConvert::into_optioned(self.access_modes),
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
            mount_options: crate::OptionableConvert::into_optioned(self.mount_options),
            nfs: crate::OptionableConvert::into_optioned(self.nfs),
            node_affinity: crate::OptionableConvert::into_optioned(self.node_affinity),
            persistent_volume_reclaim_policy: crate::OptionableConvert::into_optioned(
                self.persistent_volume_reclaim_policy,
            ),
            photon_persistent_disk: crate::OptionableConvert::into_optioned(
                self.photon_persistent_disk,
            ),
            portworx_volume: crate::OptionableConvert::into_optioned(
                self.portworx_volume,
            ),
            quobyte: crate::OptionableConvert::into_optioned(self.quobyte),
            rbd: crate::OptionableConvert::into_optioned(self.rbd),
            scale_io: crate::OptionableConvert::into_optioned(self.scale_io),
            storage_class_name: crate::OptionableConvert::into_optioned(
                self.storage_class_name,
            ),
            storageos: crate::OptionableConvert::into_optioned(self.storageos),
            volume_attributes_class_name: crate::OptionableConvert::into_optioned(
                self.volume_attributes_class_name,
            ),
            volume_mode: crate::OptionableConvert::into_optioned(self.volume_mode),
            vsphere_volume: crate::OptionableConvert::into_optioned(self.vsphere_volume),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            access_modes: crate::OptionableConvert::try_from_optioned(
                value.access_modes,
            )?,
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
            mount_options: crate::OptionableConvert::try_from_optioned(
                value.mount_options,
            )?,
            nfs: crate::OptionableConvert::try_from_optioned(value.nfs)?,
            node_affinity: crate::OptionableConvert::try_from_optioned(
                value.node_affinity,
            )?,
            persistent_volume_reclaim_policy: crate::OptionableConvert::try_from_optioned(
                value.persistent_volume_reclaim_policy,
            )?,
            photon_persistent_disk: crate::OptionableConvert::try_from_optioned(
                value.photon_persistent_disk,
            )?,
            portworx_volume: crate::OptionableConvert::try_from_optioned(
                value.portworx_volume,
            )?,
            quobyte: crate::OptionableConvert::try_from_optioned(value.quobyte)?,
            rbd: crate::OptionableConvert::try_from_optioned(value.rbd)?,
            scale_io: crate::OptionableConvert::try_from_optioned(value.scale_io)?,
            storage_class_name: crate::OptionableConvert::try_from_optioned(
                value.storage_class_name,
            )?,
            storageos: crate::OptionableConvert::try_from_optioned(value.storageos)?,
            volume_attributes_class_name: crate::OptionableConvert::try_from_optioned(
                value.volume_attributes_class_name,
            )?,
            volume_mode: crate::OptionableConvert::try_from_optioned(value.volume_mode)?,
            vsphere_volume: crate::OptionableConvert::try_from_optioned(
                value.vsphere_volume,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.access_modes, other.access_modes)?;
        crate::OptionableConvert::merge(
            &mut self.aws_elastic_block_store,
            other.aws_elastic_block_store,
        )?;
        crate::OptionableConvert::merge(&mut self.azure_disk, other.azure_disk)?;
        crate::OptionableConvert::merge(&mut self.azure_file, other.azure_file)?;
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(&mut self.cephfs, other.cephfs)?;
        crate::OptionableConvert::merge(&mut self.cinder, other.cinder)?;
        crate::OptionableConvert::merge(&mut self.claim_ref, other.claim_ref)?;
        crate::OptionableConvert::merge(&mut self.csi, other.csi)?;
        crate::OptionableConvert::merge(&mut self.fc, other.fc)?;
        crate::OptionableConvert::merge(&mut self.flex_volume, other.flex_volume)?;
        crate::OptionableConvert::merge(&mut self.flocker, other.flocker)?;
        crate::OptionableConvert::merge(
            &mut self.gce_persistent_disk,
            other.gce_persistent_disk,
        )?;
        crate::OptionableConvert::merge(&mut self.glusterfs, other.glusterfs)?;
        crate::OptionableConvert::merge(&mut self.host_path, other.host_path)?;
        crate::OptionableConvert::merge(&mut self.iscsi, other.iscsi)?;
        crate::OptionableConvert::merge(&mut self.local, other.local)?;
        crate::OptionableConvert::merge(&mut self.mount_options, other.mount_options)?;
        crate::OptionableConvert::merge(&mut self.nfs, other.nfs)?;
        crate::OptionableConvert::merge(&mut self.node_affinity, other.node_affinity)?;
        crate::OptionableConvert::merge(
            &mut self.persistent_volume_reclaim_policy,
            other.persistent_volume_reclaim_policy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.photon_persistent_disk,
            other.photon_persistent_disk,
        )?;
        crate::OptionableConvert::merge(
            &mut self.portworx_volume,
            other.portworx_volume,
        )?;
        crate::OptionableConvert::merge(&mut self.quobyte, other.quobyte)?;
        crate::OptionableConvert::merge(&mut self.rbd, other.rbd)?;
        crate::OptionableConvert::merge(&mut self.scale_io, other.scale_io)?;
        crate::OptionableConvert::merge(
            &mut self.storage_class_name,
            other.storage_class_name,
        )?;
        crate::OptionableConvert::merge(&mut self.storageos, other.storageos)?;
        crate::OptionableConvert::merge(
            &mut self.volume_attributes_class_name,
            other.volume_attributes_class_name,
        )?;
        crate::OptionableConvert::merge(&mut self.volume_mode, other.volume_mode)?;
        crate::OptionableConvert::merge(&mut self.vsphere_volume, other.vsphere_volume)?;
        Ok(())
    }
}
