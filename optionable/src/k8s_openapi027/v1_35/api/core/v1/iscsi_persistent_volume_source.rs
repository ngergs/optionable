#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ISCSIPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_auth_discovery: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chap_auth_session: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iqn: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi_interface: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portals: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_portal: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ISCSIPersistentVolumeSource {
    type Optioned = ISCSIPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ISCSIPersistentVolumeSourceAc {
    type Optioned = ISCSIPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ISCSIPersistentVolumeSource {
    fn into_optioned(self) -> ISCSIPersistentVolumeSourceAc {
        ISCSIPersistentVolumeSourceAc {
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
    fn try_from_optioned(
        value: ISCSIPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
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
    fn merge(
        &mut self,
        other: ISCSIPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        self.chap_auth_discovery = other.chap_auth_discovery;
        self.chap_auth_session = other.chap_auth_session;
        self.fs_type = other.fs_type;
        self.initiator_name = other.initiator_name;
        if let Some(other_value) = other.iqn {
            self.iqn = other_value;
        }
        self.iscsi_interface = other.iscsi_interface;
        if let Some(other_value) = other.lun {
            self.lun = other_value;
        }
        self.portals = other.portals;
        self.read_only = other.read_only;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        if let Some(other_value) = other.target_portal {
            self.target_portal = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ISCSIPersistentVolumeSource>
for ISCSIPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ISCSIPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ISCSIPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ISCSIPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
