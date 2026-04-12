#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TypedLocalObjectReference contains enough information to let you locate the typed referenced object inside the same namespace.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypedLocalObjectReferenceAc {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
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
impl crate::Optionable for k8s_openapi027::api::core::v1::TypedLocalObjectReference {
    type Optioned = TypedLocalObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for TypedLocalObjectReferenceAc {
    type Optioned = TypedLocalObjectReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::TypedLocalObjectReference {
    fn into_optioned(self) -> TypedLocalObjectReferenceAc {
        TypedLocalObjectReferenceAc {
            api_group: self.api_group,
            kind: Some(self.kind),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(
        value: TypedLocalObjectReferenceAc,
    ) -> Result<Self, crate::Error> {
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
    fn merge(&mut self, other: TypedLocalObjectReferenceAc) -> Result<(), crate::Error> {
        if self.api_group.is_none() {
            self.api_group = other.api_group;
        }
        if let Some(other_value) = other.api_group {
            crate::OptionableConvert::merge(&mut self.api_group, other_value)?;
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
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::TypedLocalObjectReference>
for TypedLocalObjectReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::TypedLocalObjectReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::TypedLocalObjectReference, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::TypedLocalObjectReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
