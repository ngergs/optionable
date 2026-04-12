#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceAttributesAc {
    /// fieldSelector describes the limitation on access based on field.  It can only limit access, not broaden it.
    ///
    /// This field  is alpha-level. To use this field, you must enable the `AuthorizeWithSelectors` feature gate (disabled by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_selector: Option<
        <::k8s_openapi027::api::authorization::v1::FieldSelectorAttributes as crate::Optionable>::Optioned,
    >,
    /// Group is the API Group of the Resource.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    /// labelSelector describes the limitation on access based on labels.  It can only limit access, not broaden it.
    ///
    /// This field  is alpha-level. To use this field, you must enable the `AuthorizeWithSelectors` feature gate (disabled by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<
        <::k8s_openapi027::api::authorization::v1::LabelSelectorAttributes as crate::Optionable>::Optioned,
    >,
    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces "" (empty) is defaulted for LocalSubjectAccessReviews "" (empty) is empty for cluster-scoped resources "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    /// Resource is one of the existing resource types.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::string::String>,
    /// Subresource is one of the existing resource types.  "" means none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource: Option<std::string::String>,
    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<std::string::String>,
    /// Version is the API Version of the Resource.  "*" means all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::authorization::v1::ResourceAttributes {
    type Optioned = ResourceAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceAttributesAc {
    type Optioned = ResourceAttributesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::ResourceAttributes {
    fn into_optioned(self) -> ResourceAttributesAc {
        ResourceAttributesAc {
            field_selector: crate::OptionableConvert::into_optioned(self.field_selector),
            group: self.group,
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            name: self.name,
            namespace: self.namespace,
            resource: self.resource,
            subresource: self.subresource,
            verb: self.verb,
            version: self.version,
        }
    }
    fn try_from_optioned(value: ResourceAttributesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field_selector: crate::OptionableConvert::try_from_optioned(
                value.field_selector,
            )?,
            group: value.group,
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            name: value.name,
            namespace: value.namespace,
            resource: value.resource,
            subresource: value.subresource,
            verb: value.verb,
            version: value.version,
        })
    }
    fn merge(&mut self, other: ResourceAttributesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.field_selector, other.field_selector)?;
        if other.group.is_some() {
            self.group = other.group;
        }
        crate::OptionableConvert::merge(&mut self.label_selector, other.label_selector)?;
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.namespace.is_some() {
            self.namespace = other.namespace;
        }
        if other.resource.is_some() {
            self.resource = other.resource;
        }
        if other.subresource.is_some() {
            self.subresource = other.subresource;
        }
        if other.verb.is_some() {
            self.verb = other.verb;
        }
        if other.version.is_some() {
            self.version = other.version;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::authorization::v1::ResourceAttributes>
for ResourceAttributesAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::ResourceAttributes,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::ResourceAttributes,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::ResourceAttributes,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
