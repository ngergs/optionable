#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GlusterfsPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints_namespace: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource {
    type Optioned = GlusterfsPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GlusterfsPersistentVolumeSourceAc {
    type Optioned = GlusterfsPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource {
    fn into_optioned(self) -> GlusterfsPersistentVolumeSourceAc {
        GlusterfsPersistentVolumeSourceAc {
            endpoints: Some(self.endpoints),
            endpoints_namespace: self.endpoints_namespace,
            path: Some(self.path),
            read_only: self.read_only,
        }
    }
    fn try_from_optioned(
        value: GlusterfsPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            endpoints: value
                .endpoints
                .ok_or(crate::Error {
                    missing_field: "endpoints",
                })?,
            endpoints_namespace: value.endpoints_namespace,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            read_only: value.read_only,
        })
    }
    fn merge(
        &mut self,
        other: GlusterfsPersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.endpoints {
            self.endpoints = other_value;
        }
        self.endpoints_namespace = other.endpoints_namespace;
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        self.read_only = other.read_only;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource,
> for GlusterfsPersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::GlusterfsPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
