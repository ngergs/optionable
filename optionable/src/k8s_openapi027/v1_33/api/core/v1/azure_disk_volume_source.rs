#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AzureDiskVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_mode: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<std::string::String>,
    #[serde(rename = "diskURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_uri: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::AzureDiskVolumeSource {
    type Optioned = AzureDiskVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for AzureDiskVolumeSourceAc {
    type Optioned = AzureDiskVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::AzureDiskVolumeSource {
    fn into_optioned(self) -> AzureDiskVolumeSourceAc {
        AzureDiskVolumeSourceAc {
            caching_mode: self.caching_mode,
            disk_name: Some(self.disk_name),
            disk_uri: Some(self.disk_uri),
            fs_type: self.fs_type,
            kind: self.kind,
            read_only: self.read_only,
        }
    }
    fn try_from_optioned(value: AzureDiskVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            caching_mode: value.caching_mode,
            disk_name: value
                .disk_name
                .ok_or(crate::Error {
                    missing_field: "disk_name",
                })?,
            disk_uri: value
                .disk_uri
                .ok_or(crate::Error {
                    missing_field: "disk_uri",
                })?,
            fs_type: value.fs_type,
            kind: value.kind,
            read_only: value.read_only,
        })
    }
    fn merge(&mut self, other: AzureDiskVolumeSourceAc) -> Result<(), crate::Error> {
        self.caching_mode = other.caching_mode;
        if let Some(other_value) = other.disk_name {
            self.disk_name = other_value;
        }
        if let Some(other_value) = other.disk_uri {
            self.disk_uri = other_value;
        }
        self.fs_type = other.fs_type;
        self.kind = other.kind;
        self.read_only = other.read_only;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::AzureDiskVolumeSource>
for AzureDiskVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::AzureDiskVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::AzureDiskVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::AzureDiskVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
