#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// RuleWithOperations is a tuple of Operations and Resources. It is recommended to make sure that all the tuple expansions are valid.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleWithOperationsAc {
    /// APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    /// APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_versions: Option<std::vec::Vec<std::string::String>>,
    /// Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or * for all of those operations and any future admission operations that are added. If '*' is present, the length of the slice must be one. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<std::vec::Vec<std::string::String>>,
    /// Resources is a list of resources this rule applies to.
    ///
    /// For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/*' means all subresources of pods. '*/scale' means all scale subresources. '*/*' means all resources and their subresources.
    ///
    /// If wildcard is present, the validation rule will ensure resources do not overlap with each other.
    ///
    /// Depending on the enclosing object, subresources might not be allowed. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    /// scope specifies the scope of this rule. Valid values are "Cluster", "Namespaced", and "*" "Cluster" means that only cluster-scoped resources will match this rule. Namespace API objects are cluster-scoped. "Namespaced" means that only namespaced resources will match this rule. "*" means that there are no scope restrictions. Subresources match the scope of their parent resource. Default is "*".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::RuleWithOperations {
    type Optioned = RuleWithOperationsAc;
}
#[automatically_derived]
impl crate::Optionable for RuleWithOperationsAc {
    type Optioned = RuleWithOperationsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::RuleWithOperations {
    fn into_optioned(self) -> RuleWithOperationsAc {
        RuleWithOperationsAc {
            api_groups: self.api_groups,
            api_versions: self.api_versions,
            operations: self.operations,
            resources: self.resources,
            scope: self.scope,
        }
    }
    fn try_from_optioned(value: RuleWithOperationsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value.api_groups,
            api_versions: value.api_versions,
            operations: value.operations,
            resources: value.resources,
            scope: value.scope,
        })
    }
    fn merge(&mut self, other: RuleWithOperationsAc) -> Result<(), crate::Error> {
        if self.api_groups.is_none() {
            self.api_groups = crate::OptionableConvert::try_from_optioned(
                other.api_groups,
            )?;
        } else if let Some(self_value) = self.api_groups.as_mut()
            && let Some(other_value) = other.api_groups
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.api_versions.is_none() {
            self.api_versions = crate::OptionableConvert::try_from_optioned(
                other.api_versions,
            )?;
        } else if let Some(self_value) = self.api_versions.as_mut()
            && let Some(other_value) = other.api_versions
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.operations.is_none() {
            self.operations = crate::OptionableConvert::try_from_optioned(
                other.operations,
            )?;
        } else if let Some(self_value) = self.operations.as_mut()
            && let Some(other_value) = other.operations
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.resources.is_none() {
            self.resources = crate::OptionableConvert::try_from_optioned(
                other.resources,
            )?;
        } else if let Some(self_value) = self.resources.as_mut()
            && let Some(other_value) = other.resources
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.scope.is_none() {
            self.scope = crate::OptionableConvert::try_from_optioned(other.scope)?;
        } else if let Some(self_value) = self.scope.as_mut()
            && let Some(other_value) = other.scope
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::RuleWithOperations,
> for RuleWithOperationsAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::RuleWithOperations,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::RuleWithOperations,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::RuleWithOperations,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for RuleWithOperationsAc {
    fn merge_from(&mut self, other: Self) {
        self.api_groups = other.api_groups;
        self.api_versions = other.api_versions;
        self.operations = other.operations;
        self.resources = other.resources;
        k8s_openapi027::DeepMerge::merge_from(&mut self.scope, other.scope);
    }
}
