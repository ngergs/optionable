#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlexPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::FlexPersistentVolumeSource {
    type Optioned = FlexPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for FlexPersistentVolumeSourceAc {
    type Optioned = FlexPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::FlexPersistentVolumeSource {
    fn into_optioned(self) -> FlexPersistentVolumeSourceAc {
        FlexPersistentVolumeSourceAc {
            driver: Some(self.driver),
            fs_type: self.fs_type,
            options: self.options,
            read_only: self.read_only,
            secret_ref: crate::OptionableConvert::into_optioned(self.secret_ref),
        }
    }
    fn try_from_optioned(
        value: FlexPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            fs_type: value.fs_type,
            options: value.options,
            read_only: value.read_only,
            secret_ref: crate::OptionableConvert::try_from_optioned(value.secret_ref)?,
        })
    }
    fn merge(
        &mut self,
        other: FlexPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver {
            self.driver = other_value;
        }
        self.fs_type = other.fs_type;
        self.options = other.options;
        self.read_only = other.read_only;
        crate::OptionableConvert::merge(&mut self.secret_ref, other.secret_ref)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::FlexPersistentVolumeSource>
for FlexPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::FlexPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::FlexPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::FlexPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
