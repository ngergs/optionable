#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleIOVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_domain: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_enabled: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_pool: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ScaleIOVolumeSource {
    type Optioned = ScaleIOVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ScaleIOVolumeSourceAc {
    type Optioned = ScaleIOVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ScaleIOVolumeSource {
    fn into_optioned(self) -> ScaleIOVolumeSourceAc {
        ScaleIOVolumeSourceAc {
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            gateway: Some(crate::OptionableConvert::into_optioned(self.gateway)),
            protection_domain: crate::OptionableConvert::into_optioned(
                self.protection_domain,
            ),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_ref: Some(crate::OptionableConvert::into_optioned(self.secret_ref)),
            ssl_enabled: crate::OptionableConvert::into_optioned(self.ssl_enabled),
            storage_mode: crate::OptionableConvert::into_optioned(self.storage_mode),
            storage_pool: crate::OptionableConvert::into_optioned(self.storage_pool),
            system: Some(crate::OptionableConvert::into_optioned(self.system)),
            volume_name: crate::OptionableConvert::into_optioned(self.volume_name),
        }
    }
    fn try_from_optioned(
        value: ScaleIOVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            gateway: crate::OptionableConvert::try_from_optioned(
                value
                    .gateway
                    .ok_or(crate::optionable::Error {
                        missing_field: "gateway",
                    })?,
            )?,
            protection_domain: crate::OptionableConvert::try_from_optioned(
                value.protection_domain,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_ref: crate::OptionableConvert::try_from_optioned(
                value
                    .secret_ref
                    .ok_or(crate::optionable::Error {
                        missing_field: "secret_ref",
                    })?,
            )?,
            ssl_enabled: crate::OptionableConvert::try_from_optioned(value.ssl_enabled)?,
            storage_mode: crate::OptionableConvert::try_from_optioned(
                value.storage_mode,
            )?,
            storage_pool: crate::OptionableConvert::try_from_optioned(
                value.storage_pool,
            )?,
            system: crate::OptionableConvert::try_from_optioned(
                value
                    .system
                    .ok_or(crate::optionable::Error {
                        missing_field: "system",
                    })?,
            )?,
            volume_name: crate::OptionableConvert::try_from_optioned(value.volume_name)?,
        })
    }
    fn merge(
        &mut self,
        other: ScaleIOVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        if let Some(other_value) = other.gateway {
            crate::OptionableConvert::merge(&mut self.gateway, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.protection_domain,
            other.protection_domain,
        )?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.secret_ref {
            crate::OptionableConvert::merge(&mut self.secret_ref, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.ssl_enabled, other.ssl_enabled)?;
        crate::OptionableConvert::merge(&mut self.storage_mode, other.storage_mode)?;
        crate::OptionableConvert::merge(&mut self.storage_pool, other.storage_pool)?;
        if let Some(other_value) = other.system {
            crate::OptionableConvert::merge(&mut self.system, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.volume_name, other.volume_name)?;
        Ok(())
    }
}
