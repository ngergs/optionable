#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ParentReference describes a reference to a parent object.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParentReferenceAc {
    /// Group is the group of the object being referenced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    /// Name is the name of the object being referenced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Namespace is the namespace of the object being referenced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    /// Resource is the resource of the object being referenced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1beta1::ParentReference {
    type Optioned = ParentReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ParentReferenceAc {
    type Optioned = ParentReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1beta1::ParentReference {
    fn into_optioned(self) -> ParentReferenceAc {
        ParentReferenceAc {
            group: self.group,
            name: Some(self.name),
            namespace: self.namespace,
            resource: Some(self.resource),
        }
    }
    fn try_from_optioned(value: ParentReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: value.group,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            namespace: value.namespace,
            resource: value
                .resource
                .ok_or(crate::Error {
                    missing_field: "resource",
                })?,
        })
    }
    fn merge(&mut self, other: ParentReferenceAc) -> Result<(), crate::Error> {
        if other.group.is_some() {
            self.group = other.group;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if other.namespace.is_some() {
            self.namespace = other.namespace;
        }
        if let Some(other_value) = other.resource {
            self.resource = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1beta1::ParentReference>
for ParentReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1beta1::ParentReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::networking::v1beta1::ParentReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1beta1::ParentReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
