#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CSIVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::LocalObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::CSIVolumeSource {
    type Optioned = CSIVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CSIVolumeSourceAc {
    type Optioned = CSIVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::CSIVolumeSource {
    fn into_optioned(self) -> CSIVolumeSourceAc {
        CSIVolumeSourceAc {
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            node_publish_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_publish_secret_ref,
            ),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            volume_attributes: crate::OptionableConvert::into_optioned(
                self.volume_attributes,
            ),
        }
    }
    fn try_from_optioned(value: CSIVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            driver: crate::OptionableConvert::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::Error {
                        missing_field: "driver",
                    })?,
            )?,
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            node_publish_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_publish_secret_ref,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            volume_attributes: crate::OptionableConvert::try_from_optioned(
                value.volume_attributes,
            )?,
        })
    }
    fn merge(&mut self, other: CSIVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver {
            crate::OptionableConvert::merge(&mut self.driver, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(
            &mut self.node_publish_secret_ref,
            other.node_publish_secret_ref,
        )?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(
            &mut self.volume_attributes,
            other.volume_attributes,
        )?;
        Ok(())
    }
}
