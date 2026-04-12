#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI volumes support ownership management and SELinux relabeling.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ISCSIVolumeSourceAc {
    /// chapAuthDiscovery defines whether support iSCSI Discovery CHAP authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_auth_discovery: Option<bool>,
    /// chapAuthSession defines whether support iSCSI Session CHAP authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_auth_session: Option<bool>,
    /// fsType is the filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    /// initiatorName is the custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface \<target portal\>:\<volume name\> will be created for the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<std::string::String>,
    /// iqn is the target iSCSI Qualified Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iqn: Option<std::string::String>,
    /// iscsiInterface is the interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi_interface: Option<std::string::String>,
    /// lun represents iSCSI Target Lun number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    /// portals is the iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portals: Option<std::vec::Vec<std::string::String>>,
    /// readOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretRef is the CHAP Secret for iSCSI target and initiator authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// targetPortal is iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_portal: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ISCSIVolumeSource {
    type Optioned = ISCSIVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ISCSIVolumeSourceAc {
    type Optioned = ISCSIVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ISCSIVolumeSource {
    fn into_optioned(self) -> ISCSIVolumeSourceAc {
        ISCSIVolumeSourceAc {
            chap_auth_discovery: self.chap_auth_discovery,
            chap_auth_session: self.chap_auth_session,
            fs_type: self.fs_type,
            initiator_name: self.initiator_name,
            iqn: Some(self.iqn),
            iscsi_interface: self.iscsi_interface,
            lun: Some(self.lun),
            portals: self.portals,
            read_only: self.read_only,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            target_portal: Some(self.target_portal),
        }
    }
    fn try_from_optioned(value: ISCSIVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            chap_auth_discovery: value.chap_auth_discovery,
            chap_auth_session: value.chap_auth_session,
            fs_type: value.fs_type,
            initiator_name: value.initiator_name,
            iqn: value
                .iqn
                .ok_or(crate::Error {
                    missing_field: "iqn",
                })?,
            iscsi_interface: value.iscsi_interface,
            lun: value
                .lun
                .ok_or(crate::Error {
                    missing_field: "lun",
                })?,
            portals: value.portals,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            target_portal: value
                .target_portal
                .ok_or(crate::Error {
                    missing_field: "target_portal",
                })?,
        })
    }
    fn merge(&mut self, other: ISCSIVolumeSourceAc) -> Result<(), crate::Error> {
        if other.chap_auth_discovery.is_some() {
            self.chap_auth_discovery = other.chap_auth_discovery;
        }
        if other.chap_auth_session.is_some() {
            self.chap_auth_session = other.chap_auth_session;
        }
        if other.fs_type.is_some() {
            self.fs_type = other.fs_type;
        }
        if other.initiator_name.is_some() {
            self.initiator_name = other.initiator_name;
        }
        if let Some(other_value) = other.iqn {
            self.iqn = other_value;
        }
        if other.iscsi_interface.is_some() {
            self.iscsi_interface = other.iscsi_interface;
        }
        if let Some(other_value) = other.lun {
            self.lun = other_value;
        }
        if other.portals.is_some() {
            self.portals = other.portals;
        }
        if other.read_only.is_some() {
            self.read_only = other.read_only;
        }
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        if let Some(other_value) = other.target_portal {
            self.target_portal = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ISCSIVolumeSource>
for ISCSIVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ISCSIVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ISCSIVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ISCSIVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
