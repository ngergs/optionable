#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TypedLocalObjectReference allows to reference typed object inside the same namespace.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypedLocalObjectReferenceAc {
    /// APIGroup is the group for the resource being referenced. If APIGroup is empty, the specified Kind must be in the core API group. For any other third-party types, setting APIGroup is required. It must be a DNS subdomain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<std::string::String>,
    /// Kind is the type of resource being referenced. It must be a path segment name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// Name is the name of resource being referenced. It must be a path segment name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::scheduling::v1alpha1::TypedLocalObjectReference {
    type Optioned = TypedLocalObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for TypedLocalObjectReferenceAc {
    type Optioned = TypedLocalObjectReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::scheduling::v1alpha1::TypedLocalObjectReference {
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
impl crate::OptionedConvert<
    k8s_openapi027::api::scheduling::v1alpha1::TypedLocalObjectReference,
> for TypedLocalObjectReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::scheduling::v1alpha1::TypedLocalObjectReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::scheduling::v1alpha1::TypedLocalObjectReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::scheduling::v1alpha1::TypedLocalObjectReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for TypedLocalObjectReferenceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_group, other.api_group);
        k8s_openapi027::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
