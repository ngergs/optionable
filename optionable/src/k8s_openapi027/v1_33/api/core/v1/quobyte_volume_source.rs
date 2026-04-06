#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QuobyteVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::QuobyteVolumeSource {
    type Optioned = QuobyteVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for QuobyteVolumeSourceAc {
    type Optioned = QuobyteVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::QuobyteVolumeSource {
    fn into_optioned(self) -> QuobyteVolumeSourceAc {
        QuobyteVolumeSourceAc {
            group: self.group,
            read_only: self.read_only,
            registry: Some(self.registry),
            tenant: self.tenant,
            user: self.user,
            volume: Some(self.volume),
        }
    }
    fn try_from_optioned(value: QuobyteVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: value.group,
            read_only: value.read_only,
            registry: value
                .registry
                .ok_or(crate::Error {
                    missing_field: "registry",
                })?,
            tenant: value.tenant,
            user: value.user,
            volume: value
                .volume
                .ok_or(crate::Error {
                    missing_field: "volume",
                })?,
        })
    }
    fn merge(&mut self, other: QuobyteVolumeSourceAc) -> Result<(), crate::Error> {
        self.group = other.group;
        self.read_only = other.read_only;
        if let Some(other_value) = other.registry {
            self.registry = other_value;
        }
        self.tenant = other.tenant;
        self.user = other.user;
        if let Some(other_value) = other.volume {
            self.volume = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::QuobyteVolumeSource>
for QuobyteVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::QuobyteVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::QuobyteVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::QuobyteVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
