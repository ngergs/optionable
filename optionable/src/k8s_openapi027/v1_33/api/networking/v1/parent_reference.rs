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
impl crate::Optionable for k8s_openapi027::api::networking::v1::ParentReference {
    type Optioned = ParentReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ParentReferenceAc {
    type Optioned = ParentReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::ParentReference {
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
        if self.group.is_none() {
            self.group = crate::OptionableConvert::try_from_optioned(other.group)?;
        } else if let Some(self_value) = self.group.as_mut()
            && let Some(other_value) = other.group
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.namespace.is_none() {
            self.namespace = crate::OptionableConvert::try_from_optioned(
                other.namespace,
            )?;
        } else if let Some(self_value) = self.namespace.as_mut()
            && let Some(other_value) = other.namespace
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.resource {
            self.resource = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::ParentReference>
for ParentReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::ParentReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::ParentReference, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::ParentReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ParentReferenceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.group, other.group);
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.namespace, other.namespace);
        k8s_openapi027::DeepMerge::merge_from(&mut self.resource, other.resource);
    }
}
