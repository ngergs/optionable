#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PolicyRule holds information that describes a policy rule, but does not contain information about who the rule applies to or which namespace the rule applies to.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PolicyRuleAc {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed. "" represents the core API group and "*" represents all API groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as "pods" or "secrets") or non-resource URL paths (such as "/api"),  but not both.
    #[serde(rename = "nonResourceURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: Option<std::vec::Vec<std::string::String>>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<std::vec::Vec<std::string::String>>,
    /// Resources is a list of resources this rule applies to. '*' represents all resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule. '*' represents all verbs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::rbac::v1::PolicyRule {
    type Optioned = PolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for PolicyRuleAc {
    type Optioned = PolicyRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::rbac::v1::PolicyRule {
    fn into_optioned(self) -> PolicyRuleAc {
        PolicyRuleAc {
            api_groups: self.api_groups,
            non_resource_urls: self.non_resource_urls,
            resource_names: self.resource_names,
            resources: self.resources,
            verbs: Some(self.verbs),
        }
    }
    fn try_from_optioned(value: PolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value.api_groups,
            non_resource_urls: value.non_resource_urls,
            resource_names: value.resource_names,
            resources: value.resources,
            verbs: value
                .verbs
                .ok_or(crate::Error {
                    missing_field: "verbs",
                })?,
        })
    }
    fn merge(&mut self, other: PolicyRuleAc) -> Result<(), crate::Error> {
        if other.api_groups.is_some() {
            self.api_groups = other.api_groups;
        }
        if other.non_resource_urls.is_some() {
            self.non_resource_urls = other.non_resource_urls;
        }
        if other.resource_names.is_some() {
            self.resource_names = other.resource_names;
        }
        if other.resources.is_some() {
            self.resources = other.resources;
        }
        if let Some(other_value) = other.verbs {
            self.verbs = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::rbac::v1::PolicyRule> for PolicyRuleAc {
    fn from_optionable(value: k8s_openapi027::api::rbac::v1::PolicyRule) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::rbac::v1::PolicyRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::rbac::v1::PolicyRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
