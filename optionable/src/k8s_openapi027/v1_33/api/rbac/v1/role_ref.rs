#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoleRefAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::rbac::v1::RoleRef {
    type Optioned = RoleRefAc;
}
#[automatically_derived]
impl crate::Optionable for RoleRefAc {
    type Optioned = RoleRefAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::rbac::v1::RoleRef {
    fn into_optioned(self) -> RoleRefAc {
        RoleRefAc {
            api_group: Some(self.api_group),
            kind: Some(self.kind),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: RoleRefAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_group: value
                .api_group
                .ok_or(crate::Error {
                    missing_field: "api_group",
                })?,
            kind: value
                .kind
                .ok_or(crate::Error {
                    missing_field: "kind",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(&mut self, other: RoleRefAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.api_group {
            self.api_group = other_value;
        }
        if let Some(other_value) = other.kind {
            self.kind = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::rbac::v1::RoleRef> for RoleRefAc {
    fn from_optionable(value: k8s_openapi027::api::rbac::v1::RoleRef) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::rbac::v1::RoleRef, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::rbac::v1::RoleRef,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
