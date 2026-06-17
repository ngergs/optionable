#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// RoleRef contains information that points to the role being used
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoleRefAc {
    /// APIGroup is the group for the resource being referenced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<std::string::String>,
    /// Kind is the type of resource being referenced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// Name is the name of resource being referenced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi028::api::rbac::v1::RoleRef {
    type Optioned = RoleRefAc;
}
#[automatically_derived]
impl crate::Optionable for RoleRefAc {
    type Optioned = RoleRefAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi028::api::rbac::v1::RoleRef {
    fn into_optioned(self) -> RoleRefAc {
        RoleRefAc {
            api_group: self.api_group,
            kind: Some(self.kind),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: RoleRefAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_group: value.api_group,
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
        if self.api_group.is_none() {
            self.api_group = crate::OptionableConvert::try_from_optioned(
                other.api_group,
            )?;
        } else if let Some(self_value) = self.api_group.as_mut()
            && let Some(other_value) = other.api_group
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.kind {
            self.kind = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi028::api::rbac::v1::RoleRef> for RoleRefAc {
    fn from_optionable(value: k8s_openapi028::api::rbac::v1::RoleRef) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi028::api::rbac::v1::RoleRef, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::rbac::v1::RoleRef,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for RoleRefAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.api_group, other.api_group);
        k8s_openapi028::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi028::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
