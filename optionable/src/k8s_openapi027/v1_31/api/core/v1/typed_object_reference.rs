#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypedObjectReferenceAc {
    /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<std::string::String>,
    /// Kind is the type of resource being referenced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// Name is the name of resource being referenced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Namespace is the namespace of resource being referenced Note that when a namespace is specified, a gateway.networking.k8s.io/ReferenceGrant object is required in the referent namespace to allow that namespace's owner to accept the reference. See the ReferenceGrant documentation for details. (Alpha) This field requires the CrossNamespaceVolumeDataSource feature gate to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::TypedObjectReference {
    type Optioned = TypedObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for TypedObjectReferenceAc {
    type Optioned = TypedObjectReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::TypedObjectReference {
    fn into_optioned(self) -> TypedObjectReferenceAc {
        TypedObjectReferenceAc {
            api_group: self.api_group,
            kind: Some(self.kind),
            name: Some(self.name),
            namespace: self.namespace,
        }
    }
    fn try_from_optioned(value: TypedObjectReferenceAc) -> Result<Self, crate::Error> {
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
            namespace: value.namespace,
        })
    }
    fn merge(&mut self, other: TypedObjectReferenceAc) -> Result<(), crate::Error> {
        self.api_group = other.api_group;
        if let Some(other_value) = other.kind {
            self.kind = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.namespace = other.namespace;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::TypedObjectReference>
for TypedObjectReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::TypedObjectReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::TypedObjectReference, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::TypedObjectReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
