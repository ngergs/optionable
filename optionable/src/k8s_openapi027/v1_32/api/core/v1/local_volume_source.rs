#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LocalVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::LocalVolumeSource {
    type Optioned = LocalVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for LocalVolumeSourceAc {
    type Optioned = LocalVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::LocalVolumeSource {
    fn into_optioned(self) -> LocalVolumeSourceAc {
        LocalVolumeSourceAc {
            fs_type: self.fs_type,
            path: Some(self.path),
        }
    }
    fn try_from_optioned(value: LocalVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            fs_type: value.fs_type,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
        })
    }
    fn merge(&mut self, other: LocalVolumeSourceAc) -> Result<(), crate::Error> {
        self.fs_type = other.fs_type;
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::LocalVolumeSource>
for LocalVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::LocalVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::LocalVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::LocalVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
