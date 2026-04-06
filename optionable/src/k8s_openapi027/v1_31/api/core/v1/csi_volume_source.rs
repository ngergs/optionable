#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSIVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: Option<
        <::k8s_openapi027::api::core::v1::LocalObjectReference as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::CSIVolumeSource {
    type Optioned = CSIVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CSIVolumeSourceAc {
    type Optioned = CSIVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::CSIVolumeSource {
    fn into_optioned(self) -> CSIVolumeSourceAc {
        CSIVolumeSourceAc {
            driver: Some(self.driver),
            fs_type: self.fs_type,
            node_publish_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_publish_secret_ref,
            ),
            read_only: self.read_only,
            volume_attributes: self.volume_attributes,
        }
    }
    fn try_from_optioned(value: CSIVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            fs_type: value.fs_type,
            node_publish_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_publish_secret_ref,
            )?,
            read_only: value.read_only,
            volume_attributes: value.volume_attributes,
        })
    }
    fn merge(&mut self, other: CSIVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver {
            self.driver = other_value;
        }
        self.fs_type = other.fs_type;
        crate::OptionableConvert::merge(
            &mut self.node_publish_secret_ref,
            other.node_publish_secret_ref,
        )?;
        self.read_only = other.read_only;
        self.volume_attributes = other.volume_attributes;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::CSIVolumeSource>
for CSIVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::CSIVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::CSIVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::CSIVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
