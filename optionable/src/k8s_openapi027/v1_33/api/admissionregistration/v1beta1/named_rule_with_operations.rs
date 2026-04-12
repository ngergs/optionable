#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamedRuleWithOperationsAc {
    /// APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    /// APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_versions: Option<std::vec::Vec<std::string::String>>,
    /// Operations is the operations the admission hook cares about - CREATE, UPDATE, DELETE, CONNECT or * for all of those operations and any future admission operations that are added. If '*' is present, the length of the slice must be one. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<std::vec::Vec<std::string::String>>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<std::vec::Vec<std::string::String>>,
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
for k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations {
    type Optioned = NamedRuleWithOperationsAc;
}
#[automatically_derived]
impl crate::Optionable for NamedRuleWithOperationsAc {
    type Optioned = NamedRuleWithOperationsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations {
    fn into_optioned(self) -> NamedRuleWithOperationsAc {
        NamedRuleWithOperationsAc {
            api_groups: self.api_groups,
            api_versions: self.api_versions,
            operations: self.operations,
            resource_names: self.resource_names,
            resources: self.resources,
            scope: self.scope,
        }
    }
    fn try_from_optioned(
        value: NamedRuleWithOperationsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value.api_groups,
            api_versions: value.api_versions,
            operations: value.operations,
            resource_names: value.resource_names,
            resources: value.resources,
            scope: value.scope,
        })
    }
    fn merge(&mut self, other: NamedRuleWithOperationsAc) -> Result<(), crate::Error> {
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
        if self.resource_names.is_none() {
            self.resource_names = crate::OptionableConvert::try_from_optioned(
                other.resource_names,
            )?;
        } else if let Some(self_value) = self.resource_names.as_mut()
            && let Some(other_value) = other.resource_names
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
    k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations,
> for NamedRuleWithOperationsAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1beta1::NamedRuleWithOperations,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
