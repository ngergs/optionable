#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// APIResource specifies the name of a resource and whether it is namespaced.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIResourceAc {
    /// categories is a list of the grouped resources this resource belongs to (e.g. 'all')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<std::vec::Vec<std::string::String>>,
    /// group is the preferred group of the resource.  Empty implies the group of the containing resource list. For subresources, this may have a different value, for example: Scale".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    /// kind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// name is the plural name of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// namespaced indicates if a resource is namespaced or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaced: Option<bool>,
    /// shortNames is a list of suggested short names of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_names: Option<std::vec::Vec<std::string::String>>,
    /// singularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely. The singularName is more correct for reporting status on a single item and both singular and plural are allowed from the kubectl CLI interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular_name: Option<std::string::String>,
    /// The hash value of the storage version, the version this resource is converted to when written to the data store. Value must be treated as opaque by clients. Only equality comparison on the value is valid. This is an alpha feature and may change or be removed in the future. The field is populated by the apiserver only if the StorageVersionHash feature gate is enabled. This field will remain optional even if it graduates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_version_hash: Option<std::string::String>,
    /// verbs is a list of supported kube verbs (this includes get, list, watch, create, update, patch, delete, deletecollection, and proxy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<std::vec::Vec<std::string::String>>,
    /// version is the preferred version of the resource.  Empty implies the version of the containing resource list For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIResource {
    type Optioned = APIResourceAc;
}
#[automatically_derived]
impl crate::Optionable for APIResourceAc {
    type Optioned = APIResourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIResource {
    fn into_optioned(self) -> APIResourceAc {
        APIResourceAc {
            categories: self.categories,
            group: self.group,
            kind: Some(self.kind),
            name: Some(self.name),
            namespaced: Some(self.namespaced),
            short_names: self.short_names,
            singular_name: Some(self.singular_name),
            storage_version_hash: self.storage_version_hash,
            verbs: Some(self.verbs),
            version: self.version,
        }
    }
    fn try_from_optioned(value: APIResourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            categories: value.categories,
            group: value.group,
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
            namespaced: value
                .namespaced
                .ok_or(crate::Error {
                    missing_field: "namespaced",
                })?,
            short_names: value.short_names,
            singular_name: value
                .singular_name
                .ok_or(crate::Error {
                    missing_field: "singular_name",
                })?,
            storage_version_hash: value.storage_version_hash,
            verbs: value
                .verbs
                .ok_or(crate::Error {
                    missing_field: "verbs",
                })?,
            version: value.version,
        })
    }
    fn merge(&mut self, other: APIResourceAc) -> Result<(), crate::Error> {
        self.categories = other.categories;
        self.group = other.group;
        if let Some(other_value) = other.kind {
            self.kind = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        if let Some(other_value) = other.namespaced {
            self.namespaced = other_value;
        }
        self.short_names = other.short_names;
        if let Some(other_value) = other.singular_name {
            self.singular_name = other_value;
        }
        self.storage_version_hash = other.storage_version_hash;
        if let Some(other_value) = other.verbs {
            self.verbs = other_value;
        }
        self.version = other.version;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIResource,
> for APIResourceAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIResource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIResource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::APIResource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
