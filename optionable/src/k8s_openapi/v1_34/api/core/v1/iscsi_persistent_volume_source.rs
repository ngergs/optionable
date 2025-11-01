pub struct ISCSIPersistentVolumeSourceAc {
    pub chap_auth_discovery: <Option<bool> as crate::Optionable>::Optioned,
    pub chap_auth_session: <Option<bool> as crate::Optionable>::Optioned,
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub initiator_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub iqn: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub iscsi_interface: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub lun: Option<i32>,
    pub portals: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    pub target_portal: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ISCSIPersistentVolumeSource {
    type Optioned = ISCSIPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ISCSIPersistentVolumeSourceAc {
    type Optioned = ISCSIPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ISCSIPersistentVolumeSource {
    fn into_optioned(self) -> ISCSIPersistentVolumeSourceAc {
        ISCSIPersistentVolumeSourceAc {
            chap_auth_discovery: crate::OptionableConvert::into_optioned(
                self.chap_auth_discovery,
            ),
            chap_auth_session: crate::OptionableConvert::into_optioned(
                self.chap_auth_session,
            ),
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            initiator_name: crate::OptionableConvert::into_optioned(self.initiator_name),
            iqn: Some(crate::OptionableConvert::into_optioned(self.iqn)),
            iscsi_interface: crate::OptionableConvert::into_optioned(
                self.iscsi_interface,
            ),
            lun: Some(self.lun),
            portals: crate::OptionableConvert::into_optioned(self.portals),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
            target_portal: Some(
                crate::OptionableConvert::into_optioned(self.target_portal),
            ),
        }
    }
    fn try_from_optioned(
        value: ISCSIPersistentVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            chap_auth_discovery: crate::OptionableConvert::try_from_optioned(
                value.chap_auth_discovery,
            )?,
            chap_auth_session: crate::OptionableConvert::try_from_optioned(
                value.chap_auth_session,
            )?,
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            initiator_name: crate::OptionableConvert::try_from_optioned(
                value.initiator_name,
            )?,
            iqn: crate::OptionableConvert::try_from_optioned(
                value
                    .iqn
                    .ok_or(crate::optionable::Error {
                        missing_field: "iqn",
                    })?,
            )?,
            iscsi_interface: crate::OptionableConvert::try_from_optioned(
                value.iscsi_interface,
            )?,
            lun: value
                .lun
                .ok_or(crate::optionable::Error {
                    missing_field: "lun",
                })?,
            portals: crate::OptionableConvert::try_from_optioned(value.portals)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
            target_portal: crate::OptionableConvert::try_from_optioned(
                value
                    .target_portal
                    .ok_or(crate::optionable::Error {
                        missing_field: "target_portal",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ISCSIPersistentVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.chap_auth_discovery,
            other.chap_auth_discovery,
        )?;
        crate::OptionableConvert::merge(
            &mut self.chap_auth_session,
            other.chap_auth_session,
        )?;
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(&mut self.initiator_name, other.initiator_name)?;
        if let Some(other_value) = other.iqn {
            crate::OptionableConvert::merge(&mut self.iqn, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.iscsi_interface,
            other.iscsi_interface,
        )?;
        if let Some(other_value) = other.lun {
            self.lun = other_value;
        }
        crate::OptionableConvert::merge(&mut self.portals, other.portals)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        if let Some(other_value) = other.target_portal {
            crate::OptionableConvert::merge(&mut self.target_portal, other_value)?;
        }
        Ok(())
    }
}
