#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostPathVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::HostPathVolumeSource {
    type Optioned = HostPathVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for HostPathVolumeSourceAc {
    type Optioned = HostPathVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::HostPathVolumeSource {
    fn into_optioned(self) -> HostPathVolumeSourceAc {
        HostPathVolumeSourceAc {
            path: Some(self.path),
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: HostPathVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: HostPathVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::HostPathVolumeSource>
for HostPathVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::HostPathVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::HostPathVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::HostPathVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
