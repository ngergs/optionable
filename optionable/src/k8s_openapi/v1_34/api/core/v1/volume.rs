pub struct VolumeOpt {
    pub aws_elastic_block_store: <Option<
        ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource,
    > as crate::Optionable>::Optioned,
    pub azure_disk: <Option<
        ::k8s_openapi::api::core::v1::AzureDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    pub azure_file: <Option<
        ::k8s_openapi::api::core::v1::AzureFileVolumeSource,
    > as crate::Optionable>::Optioned,
    pub cephfs: <Option<
        ::k8s_openapi::api::core::v1::CephFSVolumeSource,
    > as crate::Optionable>::Optioned,
    pub cinder: <Option<
        ::k8s_openapi::api::core::v1::CinderVolumeSource,
    > as crate::Optionable>::Optioned,
    pub config_map: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapVolumeSource,
    > as crate::Optionable>::Optioned,
    pub csi: <Option<
        ::k8s_openapi::api::core::v1::CSIVolumeSource,
    > as crate::Optionable>::Optioned,
    pub downward_api: <Option<
        ::k8s_openapi::api::core::v1::DownwardAPIVolumeSource,
    > as crate::Optionable>::Optioned,
    pub empty_dir: <Option<
        ::k8s_openapi::api::core::v1::EmptyDirVolumeSource,
    > as crate::Optionable>::Optioned,
    pub ephemeral: <Option<
        ::k8s_openapi::api::core::v1::EphemeralVolumeSource,
    > as crate::Optionable>::Optioned,
    pub fc: <Option<
        ::k8s_openapi::api::core::v1::FCVolumeSource,
    > as crate::Optionable>::Optioned,
    pub flex_volume: <Option<
        ::k8s_openapi::api::core::v1::FlexVolumeSource,
    > as crate::Optionable>::Optioned,
    pub flocker: <Option<
        ::k8s_openapi::api::core::v1::FlockerVolumeSource,
    > as crate::Optionable>::Optioned,
    pub gce_persistent_disk: <Option<
        ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    pub git_repo: <Option<
        ::k8s_openapi::api::core::v1::GitRepoVolumeSource,
    > as crate::Optionable>::Optioned,
    pub glusterfs: <Option<
        ::k8s_openapi::api::core::v1::GlusterfsVolumeSource,
    > as crate::Optionable>::Optioned,
    pub host_path: <Option<
        ::k8s_openapi::api::core::v1::HostPathVolumeSource,
    > as crate::Optionable>::Optioned,
    pub image: <Option<
        ::k8s_openapi::api::core::v1::ImageVolumeSource,
    > as crate::Optionable>::Optioned,
    pub iscsi: <Option<
        ::k8s_openapi::api::core::v1::ISCSIVolumeSource,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub nfs: <Option<
        ::k8s_openapi::api::core::v1::NFSVolumeSource,
    > as crate::Optionable>::Optioned,
    pub persistent_volume_claim: <Option<
        ::k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource,
    > as crate::Optionable>::Optioned,
    pub photon_persistent_disk: <Option<
        ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource,
    > as crate::Optionable>::Optioned,
    pub portworx_volume: <Option<
        ::k8s_openapi::api::core::v1::PortworxVolumeSource,
    > as crate::Optionable>::Optioned,
    pub projected: <Option<
        ::k8s_openapi::api::core::v1::ProjectedVolumeSource,
    > as crate::Optionable>::Optioned,
    pub quobyte: <Option<
        ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
    > as crate::Optionable>::Optioned,
    pub rbd: <Option<
        ::k8s_openapi::api::core::v1::RBDVolumeSource,
    > as crate::Optionable>::Optioned,
    pub scale_io: <Option<
        ::k8s_openapi::api::core::v1::ScaleIOVolumeSource,
    > as crate::Optionable>::Optioned,
    pub secret: <Option<
        ::k8s_openapi::api::core::v1::SecretVolumeSource,
    > as crate::Optionable>::Optioned,
    pub storageos: <Option<
        ::k8s_openapi::api::core::v1::StorageOSVolumeSource,
    > as crate::Optionable>::Optioned,
    pub vsphere_volume: <Option<
        ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Volume {
    type Optioned = VolumeOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeOpt {
    type Optioned = VolumeOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Volume {
    fn into_optioned(self) -> VolumeOpt {
        VolumeOpt {
            aws_elastic_block_store: <Option<
                ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.aws_elastic_block_store),
            azure_disk: <Option<
                ::k8s_openapi::api::core::v1::AzureDiskVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.azure_disk),
            azure_file: <Option<
                ::k8s_openapi::api::core::v1::AzureFileVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.azure_file),
            cephfs: <Option<
                ::k8s_openapi::api::core::v1::CephFSVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.cephfs),
            cinder: <Option<
                ::k8s_openapi::api::core::v1::CinderVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.cinder),
            config_map: <Option<
                ::k8s_openapi::api::core::v1::ConfigMapVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.config_map),
            csi: <Option<
                ::k8s_openapi::api::core::v1::CSIVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.csi),
            downward_api: <Option<
                ::k8s_openapi::api::core::v1::DownwardAPIVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.downward_api),
            empty_dir: <Option<
                ::k8s_openapi::api::core::v1::EmptyDirVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.empty_dir),
            ephemeral: <Option<
                ::k8s_openapi::api::core::v1::EphemeralVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.ephemeral),
            fc: <Option<
                ::k8s_openapi::api::core::v1::FCVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.fc),
            flex_volume: <Option<
                ::k8s_openapi::api::core::v1::FlexVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.flex_volume),
            flocker: <Option<
                ::k8s_openapi::api::core::v1::FlockerVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.flocker),
            gce_persistent_disk: <Option<
                ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.gce_persistent_disk),
            git_repo: <Option<
                ::k8s_openapi::api::core::v1::GitRepoVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.git_repo),
            glusterfs: <Option<
                ::k8s_openapi::api::core::v1::GlusterfsVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.glusterfs),
            host_path: <Option<
                ::k8s_openapi::api::core::v1::HostPathVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.host_path),
            image: <Option<
                ::k8s_openapi::api::core::v1::ImageVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.image),
            iscsi: <Option<
                ::k8s_openapi::api::core::v1::ISCSIVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.iscsi),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            nfs: <Option<
                ::k8s_openapi::api::core::v1::NFSVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.nfs),
            persistent_volume_claim: <Option<
                ::k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.persistent_volume_claim),
            photon_persistent_disk: <Option<
                ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.photon_persistent_disk),
            portworx_volume: <Option<
                ::k8s_openapi::api::core::v1::PortworxVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.portworx_volume),
            projected: <Option<
                ::k8s_openapi::api::core::v1::ProjectedVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.projected),
            quobyte: <Option<
                ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.quobyte),
            rbd: <Option<
                ::k8s_openapi::api::core::v1::RBDVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.rbd),
            scale_io: <Option<
                ::k8s_openapi::api::core::v1::ScaleIOVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.scale_io),
            secret: <Option<
                ::k8s_openapi::api::core::v1::SecretVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.secret),
            storageos: <Option<
                ::k8s_openapi::api::core::v1::StorageOSVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.storageos),
            vsphere_volume: <Option<
                ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
            > as crate::OptionableConvert>::into_optioned(self.vsphere_volume),
        }
    }
    fn try_from_optioned(value: VolumeOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            aws_elastic_block_store: <Option<
                ::k8s_openapi::api::core::v1::AWSElasticBlockStoreVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(
                value.aws_elastic_block_store,
            )?,
            azure_disk: <Option<
                ::k8s_openapi::api::core::v1::AzureDiskVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.azure_disk)?,
            azure_file: <Option<
                ::k8s_openapi::api::core::v1::AzureFileVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.azure_file)?,
            cephfs: <Option<
                ::k8s_openapi::api::core::v1::CephFSVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.cephfs)?,
            cinder: <Option<
                ::k8s_openapi::api::core::v1::CinderVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.cinder)?,
            config_map: <Option<
                ::k8s_openapi::api::core::v1::ConfigMapVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.config_map)?,
            csi: <Option<
                ::k8s_openapi::api::core::v1::CSIVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.csi)?,
            downward_api: <Option<
                ::k8s_openapi::api::core::v1::DownwardAPIVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.downward_api)?,
            empty_dir: <Option<
                ::k8s_openapi::api::core::v1::EmptyDirVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.empty_dir)?,
            ephemeral: <Option<
                ::k8s_openapi::api::core::v1::EphemeralVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.ephemeral)?,
            fc: <Option<
                ::k8s_openapi::api::core::v1::FCVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.fc)?,
            flex_volume: <Option<
                ::k8s_openapi::api::core::v1::FlexVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.flex_volume)?,
            flocker: <Option<
                ::k8s_openapi::api::core::v1::FlockerVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.flocker)?,
            gce_persistent_disk: <Option<
                ::k8s_openapi::api::core::v1::GCEPersistentDiskVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(
                value.gce_persistent_disk,
            )?,
            git_repo: <Option<
                ::k8s_openapi::api::core::v1::GitRepoVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.git_repo)?,
            glusterfs: <Option<
                ::k8s_openapi::api::core::v1::GlusterfsVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.glusterfs)?,
            host_path: <Option<
                ::k8s_openapi::api::core::v1::HostPathVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.host_path)?,
            image: <Option<
                ::k8s_openapi::api::core::v1::ImageVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.image)?,
            iscsi: <Option<
                ::k8s_openapi::api::core::v1::ISCSIVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.iscsi)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            nfs: <Option<
                ::k8s_openapi::api::core::v1::NFSVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.nfs)?,
            persistent_volume_claim: <Option<
                ::k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(
                value.persistent_volume_claim,
            )?,
            photon_persistent_disk: <Option<
                ::k8s_openapi::api::core::v1::PhotonPersistentDiskVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(
                value.photon_persistent_disk,
            )?,
            portworx_volume: <Option<
                ::k8s_openapi::api::core::v1::PortworxVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.portworx_volume)?,
            projected: <Option<
                ::k8s_openapi::api::core::v1::ProjectedVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.projected)?,
            quobyte: <Option<
                ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.quobyte)?,
            rbd: <Option<
                ::k8s_openapi::api::core::v1::RBDVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.rbd)?,
            scale_io: <Option<
                ::k8s_openapi::api::core::v1::ScaleIOVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.scale_io)?,
            secret: <Option<
                ::k8s_openapi::api::core::v1::SecretVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.secret)?,
            storageos: <Option<
                ::k8s_openapi::api::core::v1::StorageOSVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.storageos)?,
            vsphere_volume: <Option<
                ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
            > as crate::OptionableConvert>::try_from_optioned(value.vsphere_volume)?,
        })
    }
    fn merge(&mut self, other: VolumeOpt) -> Result<(), crate::optionable::Error> {
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
            ::k8s_openapi::api::core::v1::AzureFileVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.azure_file, other.azure_file)?;
        <Option<
            ::k8s_openapi::api::core::v1::CephFSVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.cephfs, other.cephfs)?;
        <Option<
            ::k8s_openapi::api::core::v1::CinderVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.cinder, other.cinder)?;
        <Option<
            ::k8s_openapi::api::core::v1::ConfigMapVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.config_map, other.config_map)?;
        <Option<
            ::k8s_openapi::api::core::v1::CSIVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.csi, other.csi)?;
        <Option<
            ::k8s_openapi::api::core::v1::DownwardAPIVolumeSource,
        > as crate::OptionableConvert>::merge(
            &mut self.downward_api,
            other.downward_api,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::EmptyDirVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.empty_dir, other.empty_dir)?;
        <Option<
            ::k8s_openapi::api::core::v1::EphemeralVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.ephemeral, other.ephemeral)?;
        <Option<
            ::k8s_openapi::api::core::v1::FCVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.fc, other.fc)?;
        <Option<
            ::k8s_openapi::api::core::v1::FlexVolumeSource,
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
            ::k8s_openapi::api::core::v1::GitRepoVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.git_repo, other.git_repo)?;
        <Option<
            ::k8s_openapi::api::core::v1::GlusterfsVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.glusterfs, other.glusterfs)?;
        <Option<
            ::k8s_openapi::api::core::v1::HostPathVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.host_path, other.host_path)?;
        <Option<
            ::k8s_openapi::api::core::v1::ImageVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.image, other.image)?;
        <Option<
            ::k8s_openapi::api::core::v1::ISCSIVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.iscsi, other.iscsi)?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::NFSVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.nfs, other.nfs)?;
        <Option<
            ::k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource,
        > as crate::OptionableConvert>::merge(
            &mut self.persistent_volume_claim,
            other.persistent_volume_claim,
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
            ::k8s_openapi::api::core::v1::ProjectedVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.projected, other.projected)?;
        <Option<
            ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.quobyte, other.quobyte)?;
        <Option<
            ::k8s_openapi::api::core::v1::RBDVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.rbd, other.rbd)?;
        <Option<
            ::k8s_openapi::api::core::v1::ScaleIOVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.scale_io, other.scale_io)?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.secret, other.secret)?;
        <Option<
            ::k8s_openapi::api::core::v1::StorageOSVolumeSource,
        > as crate::OptionableConvert>::merge(&mut self.storageos, other.storageos)?;
        <Option<
            ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
        > as crate::OptionableConvert>::merge(
            &mut self.vsphere_volume,
            other.vsphere_volume,
        )?;
        Ok(())
    }
}
