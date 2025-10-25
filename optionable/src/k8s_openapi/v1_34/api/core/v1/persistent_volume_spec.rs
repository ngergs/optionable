pub struct PersistentVolumeSpecOpt {
    pub access_modes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub aws_elastic_block_store: <Option<
        ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource,
    > as crate::Optionable>::Optioned,
    pub azure_disk: <Option<
        ::k8s_openapi::api::core::v1::AzureDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    pub azure_file: <Option<
        ::k8s_openapi::api::core::v1::AzureFilePersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub capacity: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub cephfs: <Option<
        ::k8s_openapi::api::core::v1::CephFSPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub cinder: <Option<
        ::k8s_openapi::api::core::v1::CinderPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub claim_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectReference,
    > as crate::Optionable>::Optioned,
    pub csi: <Option<
        ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub fc: <Option<
        ::k8s_openapi::api::core::v1::FCVolumeSource,
    > as crate::Optionable>::Optioned,
    pub flex_volume: <Option<
        ::k8s_openapi::api::core::v1::FlexPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub flocker: <Option<
        ::k8s_openapi::api::core::v1::FlockerVolumeSource,
    > as crate::Optionable>::Optioned,
    pub gce_persistent_disk: <Option<
        ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    pub glusterfs: <Option<
        ::k8s_openapi::api::core::v1::GlusterfsPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub host_path: <Option<
        ::k8s_openapi::api::core::v1::HostPathVolumeSource,
    > as crate::Optionable>::Optioned,
    pub iscsi: <Option<
        ::k8s_openapi::api::core::v1::ISCSIPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub local: <Option<
        ::k8s_openapi::api::core::v1::LocalVolumeSource,
    > as crate::Optionable>::Optioned,
    pub mount_options: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub nfs: <Option<
        ::k8s_openapi::api::core::v1::NFSVolumeSource,
    > as crate::Optionable>::Optioned,
    pub node_affinity: <Option<
        ::k8s_openapi::api::core::v1::VolumeNodeAffinity,
    > as crate::Optionable>::Optioned,
    pub persistent_volume_reclaim_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub photon_persistent_disk: <Option<
        ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    pub portworx_volume: <Option<
        ::k8s_openapi::api::core::v1::PortworxVolumeSource,
    > as crate::Optionable>::Optioned,
    pub quobyte: <Option<
        ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
    > as crate::Optionable>::Optioned,
    pub rbd: <Option<
        ::k8s_openapi::api::core::v1::RBDPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub scale_io: <Option<
        ::k8s_openapi::api::core::v1::ScaleIOPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub storage_class_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub storageos: <Option<
        ::k8s_openapi::api::core::v1::StorageOSPersistentVolumeSource,
    > as crate::Optionable>::Optioned,
    pub volume_attributes_class_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub volume_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub vsphere_volume: <Option<
        ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PersistentVolumeSpec {
    type Optioned = PersistentVolumeSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeSpecOpt {
    type Optioned = PersistentVolumeSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PersistentVolumeSpec {
    fn into_optioned(self) -> PersistentVolumeSpecOpt {
        PersistentVolumeSpecOpt {
            access_modes: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.access_modes),
            aws_elastic_block_store: <Option<
                ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.aws_elastic_block_store),
            azure_disk: <Option<
                ::k8s_openapi::api::core::v1::AzureDiskVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.azure_disk),
            azure_file: <Option<
                ::k8s_openapi::api::core::v1::AzureFilePersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.azure_file),
            capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.capacity),
            cephfs: <Option<
                ::k8s_openapi::api::core::v1::CephFSPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.cephfs),
            cinder: <Option<
                ::k8s_openapi::api::core::v1::CinderPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.cinder),
            claim_ref: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.claim_ref),
            csi: <Option<
                ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.csi),
            fc: <Option<
                ::k8s_openapi::api::core::v1::FCVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.fc),
            flex_volume: <Option<
                ::k8s_openapi::api::core::v1::FlexPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.flex_volume),
            flocker: <Option<
                ::k8s_openapi::api::core::v1::FlockerVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.flocker),
            gce_persistent_disk: <Option<
                ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.gce_persistent_disk),
            glusterfs: <Option<
                ::k8s_openapi::api::core::v1::GlusterfsPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.glusterfs),
            host_path: <Option<
                ::k8s_openapi::api::core::v1::HostPathVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.host_path),
            iscsi: <Option<
                ::k8s_openapi::api::core::v1::ISCSIPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.iscsi),
            local: <Option<
                ::k8s_openapi::api::core::v1::LocalVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.local),
            mount_options: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.mount_options),
            nfs: <Option<
                ::k8s_openapi::api::core::v1::NFSVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.nfs),
            node_affinity: <Option<
                ::k8s_openapi::api::core::v1::VolumeNodeAffinity,
            > as crate::OptionableConvert>::into_optioned(self.node_affinity),
            persistent_volume_reclaim_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(
                self.persistent_volume_reclaim_policy,
            ),
            photon_persistent_disk: <Option<
                ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.photon_persistent_disk),
            portworx_volume: <Option<
                ::k8s_openapi::api::core::v1::PortworxVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.portworx_volume),
            quobyte: <Option<
                ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.quobyte),
            rbd: <Option<
                ::k8s_openapi::api::core::v1::RBDPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.rbd),
            scale_io: <Option<
                ::k8s_openapi::api::core::v1::ScaleIOPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.scale_io),
            storage_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.storage_class_name),
            storageos: <Option<
                ::k8s_openapi::api::core::v1::StorageOSPersistentVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.storageos),
            volume_attributes_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(
                self.volume_attributes_class_name,
            ),
            volume_mode: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.volume_mode),
            vsphere_volume: <Option<
                ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.vsphere_volume),
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            access_modes: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.access_modes)?,
            aws_elastic_block_store: <Option<
                ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(
                value.aws_elastic_block_store,
            )?,
            azure_disk: <Option<
                ::k8s_openapi::api::core::v1::AzureDiskVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.azure_disk)?,
            azure_file: <Option<
                ::k8s_openapi::api::core::v1::AzureFilePersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.azure_file)?,
            capacity: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.capacity)?,
            cephfs: <Option<
                ::k8s_openapi::api::core::v1::CephFSPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.cephfs)?,
            cinder: <Option<
                ::k8s_openapi::api::core::v1::CinderPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.cinder)?,
            claim_ref: <Option<
                ::k8s_openapi::api::core::v1::ObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.claim_ref)?,
            csi: <Option<
                ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.csi)?,
            fc: <Option<
                ::k8s_openapi::api::core::v1::FCVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.fc)?,
            flex_volume: <Option<
                ::k8s_openapi::api::core::v1::FlexPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.flex_volume)?,
            flocker: <Option<
                ::k8s_openapi::api::core::v1::FlockerVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.flocker)?,
            gce_persistent_disk: <Option<
                ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(
                value.gce_persistent_disk,
            )?,
            glusterfs: <Option<
                ::k8s_openapi::api::core::v1::GlusterfsPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.glusterfs)?,
            host_path: <Option<
                ::k8s_openapi::api::core::v1::HostPathVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.host_path)?,
            iscsi: <Option<
                ::k8s_openapi::api::core::v1::ISCSIPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.iscsi)?,
            local: <Option<
                ::k8s_openapi::api::core::v1::LocalVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.local)?,
            mount_options: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.mount_options)?,
            nfs: <Option<
                ::k8s_openapi::api::core::v1::NFSVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.nfs)?,
            node_affinity: <Option<
                ::k8s_openapi::api::core::v1::VolumeNodeAffinity,
            > as crate::OptionableConvert>::try_from_optioned(value.node_affinity)?,
            persistent_volume_reclaim_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.persistent_volume_reclaim_policy,
            )?,
            photon_persistent_disk: <Option<
                ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(
                value.photon_persistent_disk,
            )?,
            portworx_volume: <Option<
                ::k8s_openapi::api::core::v1::PortworxVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.portworx_volume)?,
            quobyte: <Option<
                ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.quobyte)?,
            rbd: <Option<
                ::k8s_openapi::api::core::v1::RBDPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.rbd)?,
            scale_io: <Option<
                ::k8s_openapi::api::core::v1::ScaleIOPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.scale_io)?,
            storage_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.storage_class_name)?,
            storageos: <Option<
                ::k8s_openapi::api::core::v1::StorageOSPersistentVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.storageos)?,
            volume_attributes_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.volume_attributes_class_name,
            )?,
            volume_mode: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.volume_mode)?,
            vsphere_volume: <Option<
                ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.vsphere_volume)?,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.access_modes,
            other.access_modes,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource,
        > as crate::OptionableConvert>::merge(
            &mut self.aws_elastic_block_store,
            other.aws_elastic_block_store,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::AzureDiskVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.azure_disk, other.azure_disk)?;
        <Option<
            ::k8s_openapi::api::core::v1::AzureFilePersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.azure_file, other.azure_file)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.capacity, other.capacity)?;
        <Option<
            ::k8s_openapi::api::core::v1::CephFSPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.cephfs, other.cephfs)?;
        <Option<
            ::k8s_openapi::api::core::v1::CinderPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.cinder, other.cinder)?;
        <Option<
            ::k8s_openapi::api::core::v1::ObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.claim_ref, other.claim_ref)?;
        <Option<
            ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.csi, other.csi)?;
        <Option<
            ::k8s_openapi::api::core::v1::FCVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.fc, other.fc)?;
        <Option<
            ::k8s_openapi::api::core::v1::FlexPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.flex_volume, other.flex_volume)?;
        <Option<
            ::k8s_openapi::api::core::v1::FlockerVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.flocker, other.flocker)?;
        <Option<
            ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource,
        > as crate::OptionableConvert>::merge(
            &mut self.gce_persistent_disk,
            other.gce_persistent_disk,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::GlusterfsPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.glusterfs, other.glusterfs)?;
        <Option<
            ::k8s_openapi::api::core::v1::HostPathVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.host_path, other.host_path)?;
        <Option<
            ::k8s_openapi::api::core::v1::ISCSIPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.iscsi, other.iscsi)?;
        <Option<
            ::k8s_openapi::api::core::v1::LocalVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.local, other.local)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.mount_options,
            other.mount_options,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::NFSVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.nfs, other.nfs)?;
        <Option<
            ::k8s_openapi::api::core::v1::VolumeNodeAffinity,
        > as crate::OptionableConvert>::merge(
            &mut self.node_affinity,
            other.node_affinity,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.persistent_volume_reclaim_policy,
            other.persistent_volume_reclaim_policy,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource,
        > as crate::OptionableConvert>::merge(
            &mut self.photon_persistent_disk,
            other.photon_persistent_disk,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::PortworxVolumeSource,
        > as crate::OptionableConvert>::merge(
            &mut self.portworx_volume,
            other.portworx_volume,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.quobyte, other.quobyte)?;
        <Option<
            ::k8s_openapi::api::core::v1::RBDPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.rbd, other.rbd)?;
        <Option<
            ::k8s_openapi::api::core::v1::ScaleIOPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.scale_io, other.scale_io)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.storage_class_name,
            other.storage_class_name,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::StorageOSPersistentVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.storageos, other.storageos)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.volume_attributes_class_name,
            other.volume_attributes_class_name,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.volume_mode, other.volume_mode)?;
        <Option<
            ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
        > as crate::OptionableConvert>::merge(
            &mut self.vsphere_volume,
            other.vsphere_volume,
        )?;
        Ok(())
    }
}
