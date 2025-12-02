#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeAc {
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
        ::k8s_openapi::api::core::v1::AzureFileVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cephfs: <Option<
        ::k8s_openapi::api::core::v1::CephFSVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cinder: <Option<
        ::k8s_openapi::api::core::v1::CinderVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csi: <Option<
        ::k8s_openapi::api::core::v1::CSIVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downward_api: <Option<
        ::k8s_openapi::api::core::v1::DownwardAPIVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir: <Option<
        ::k8s_openapi::api::core::v1::EmptyDirVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: <Option<
        ::k8s_openapi::api::core::v1::EphemeralVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc: <Option<
        ::k8s_openapi::api::core::v1::FCVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_volume: <Option<
        ::k8s_openapi::api::core::v1::FlexVolumeSource,
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
    pub git_repo: <Option<
        ::k8s_openapi::api::core::v1::GitRepoVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glusterfs: <Option<
        ::k8s_openapi::api::core::v1::GlusterfsVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: <Option<
        ::k8s_openapi::api::core::v1::HostPathVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: <Option<
        ::k8s_openapi::api::core::v1::ImageVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi: <Option<
        ::k8s_openapi::api::core::v1::ISCSIVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs: <Option<
        ::k8s_openapi::api::core::v1::NFSVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: <Option<
        ::k8s_openapi::api::core::v1::PersistentVolumeClaimVolumeSource,
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
    pub projected: <Option<
        ::k8s_openapi::api::core::v1::ProjectedVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quobyte: <Option<
        ::k8s_openapi::api::core::v1::QuobyteVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbd: <Option<
        ::k8s_openapi::api::core::v1::RBDVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_io: <Option<
        ::k8s_openapi::api::core::v1::ScaleIOVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: <Option<
        ::k8s_openapi::api::core::v1::SecretVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storageos: <Option<
        ::k8s_openapi::api::core::v1::StorageOSVolumeSource,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsphere_volume: <Option<
        ::k8s_openapi::api::core::v1::VsphereVirtualDiskVolumeSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Volume {
    type Optioned = VolumeAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAc {
    type Optioned = VolumeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Volume {
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
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
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
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
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
        crate::OptionableConvert::merge(
            &mut self.aws_elastic_block_store,
            other.aws_elastic_block_store,
        )?;
        crate::OptionableConvert::merge(&mut self.azure_disk, other.azure_disk)?;
        crate::OptionableConvert::merge(&mut self.azure_file, other.azure_file)?;
        crate::OptionableConvert::merge(&mut self.cephfs, other.cephfs)?;
        crate::OptionableConvert::merge(&mut self.cinder, other.cinder)?;
        crate::OptionableConvert::merge(&mut self.config_map, other.config_map)?;
        crate::OptionableConvert::merge(&mut self.csi, other.csi)?;
        crate::OptionableConvert::merge(&mut self.downward_api, other.downward_api)?;
        crate::OptionableConvert::merge(&mut self.empty_dir, other.empty_dir)?;
        crate::OptionableConvert::merge(&mut self.ephemeral, other.ephemeral)?;
        crate::OptionableConvert::merge(&mut self.fc, other.fc)?;
        crate::OptionableConvert::merge(&mut self.flex_volume, other.flex_volume)?;
        crate::OptionableConvert::merge(&mut self.flocker, other.flocker)?;
        crate::OptionableConvert::merge(
            &mut self.gce_persistent_disk,
            other.gce_persistent_disk,
        )?;
        crate::OptionableConvert::merge(&mut self.git_repo, other.git_repo)?;
        crate::OptionableConvert::merge(&mut self.glusterfs, other.glusterfs)?;
        crate::OptionableConvert::merge(&mut self.host_path, other.host_path)?;
        crate::OptionableConvert::merge(&mut self.image, other.image)?;
        crate::OptionableConvert::merge(&mut self.iscsi, other.iscsi)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.nfs, other.nfs)?;
        crate::OptionableConvert::merge(
            &mut self.persistent_volume_claim,
            other.persistent_volume_claim,
        )?;
        crate::OptionableConvert::merge(
            &mut self.photon_persistent_disk,
            other.photon_persistent_disk,
        )?;
        crate::OptionableConvert::merge(
            &mut self.portworx_volume,
            other.portworx_volume,
        )?;
        crate::OptionableConvert::merge(&mut self.projected, other.projected)?;
        crate::OptionableConvert::merge(&mut self.quobyte, other.quobyte)?;
        crate::OptionableConvert::merge(&mut self.rbd, other.rbd)?;
        crate::OptionableConvert::merge(&mut self.scale_io, other.scale_io)?;
        crate::OptionableConvert::merge(&mut self.secret, other.secret)?;
        crate::OptionableConvert::merge(&mut self.storageos, other.storageos)?;
        crate::OptionableConvert::merge(&mut self.vsphere_volume, other.vsphere_volume)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::Volume> for VolumeAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::Volume) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::Volume, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::Volume,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
