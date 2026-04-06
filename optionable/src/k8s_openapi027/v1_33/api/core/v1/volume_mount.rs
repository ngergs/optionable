#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeMountAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path_expr: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::VolumeMount {
    type Optioned = VolumeMountAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeMountAc {
    type Optioned = VolumeMountAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::VolumeMount {
    fn into_optioned(self) -> VolumeMountAc {
        VolumeMountAc {
            mount_path: Some(self.mount_path),
            mount_propagation: self.mount_propagation,
            name: Some(self.name),
            read_only: self.read_only,
            recursive_read_only: self.recursive_read_only,
            sub_path: self.sub_path,
            sub_path_expr: self.sub_path_expr,
        }
    }
    fn try_from_optioned(value: VolumeMountAc) -> Result<Self, crate::Error> {
        Ok(Self {
            mount_path: value
                .mount_path
                .ok_or(crate::Error {
                    missing_field: "mount_path",
                })?,
            mount_propagation: value.mount_propagation,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            read_only: value.read_only,
            recursive_read_only: value.recursive_read_only,
            sub_path: value.sub_path,
            sub_path_expr: value.sub_path_expr,
        })
    }
    fn merge(&mut self, other: VolumeMountAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.mount_path {
            self.mount_path = other_value;
        }
        self.mount_propagation = other.mount_propagation;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.read_only = other.read_only;
        self.recursive_read_only = other.recursive_read_only;
        self.sub_path = other.sub_path;
        self.sub_path_expr = other.sub_path_expr;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::VolumeMount>
for VolumeMountAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::VolumeMount) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::VolumeMount, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::VolumeMount,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
