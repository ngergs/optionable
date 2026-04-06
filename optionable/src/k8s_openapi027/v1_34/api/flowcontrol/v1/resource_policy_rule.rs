#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourcePolicyRule is a predicate that matches some resource requests, testing the request's verb and the target resource. A ResourcePolicyRule matches a resource request if and only if: (a) at least one member of verbs matches the request, (b) at least one member of apiGroups matches the request, (c) at least one member of resources matches the request, and (d) either (d1) the request does not specify a namespace (i.e., `Namespace==""`) and clusterScope is true or (d2) the request specifies a namespace and least one member of namespaces matches the request's namespace.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourcePolicyRuleAc {
    /// `apiGroups` is a list of matching API groups and may not be empty. "*" matches all API groups and, if present, must be the only entry. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_groups: Option<std::vec::Vec<std::string::String>>,
    /// `clusterScope` indicates whether to match requests that do not specify a namespace (which happens either because the resource is not namespaced or the request targets all namespaces). If this field is omitted or false then the `namespaces` field must contain a non-empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_scope: Option<bool>,
    /// `namespaces` is a list of target namespaces that restricts matches.  A request that specifies a target namespace matches only if either (a) this list contains that target namespace or (b) this list contains "*".  Note that "*" matches any specified namespace but does not match a request that _does not specify_ a namespace (see the `clusterScope` field for that). This list may be empty, but only if `clusterScope` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<std::vec::Vec<std::string::String>>,
    /// `resources` is a list of matching resources (i.e., lowercase and plural) with, if desired, subresource.  For example, \[ "services", "nodes/status" \].  This list may not be empty. "*" matches all resources and, if present, must be the only entry. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<std::vec::Vec<std::string::String>>,
    /// `verbs` is a list of matching verbs and may not be empty. "*" matches all verbs and, if present, must be the only entry. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::ResourcePolicyRule {
    type Optioned = ResourcePolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for ResourcePolicyRuleAc {
    type Optioned = ResourcePolicyRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::ResourcePolicyRule {
    fn into_optioned(self) -> ResourcePolicyRuleAc {
        ResourcePolicyRuleAc {
            api_groups: Some(self.api_groups),
            cluster_scope: self.cluster_scope,
            namespaces: self.namespaces,
            resources: Some(self.resources),
            verbs: Some(self.verbs),
        }
    }
    fn try_from_optioned(value: ResourcePolicyRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_groups: value
                .api_groups
                .ok_or(crate::Error {
                    missing_field: "api_groups",
                })?,
            cluster_scope: value.cluster_scope,
            namespaces: value.namespaces,
            resources: value
                .resources
                .ok_or(crate::Error {
                    missing_field: "resources",
                })?,
            verbs: value
                .verbs
                .ok_or(crate::Error {
                    missing_field: "verbs",
                })?,
        })
    }
    fn merge(&mut self, other: ResourcePolicyRuleAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.api_groups {
            self.api_groups = other_value;
        }
        self.cluster_scope = other.cluster_scope;
        self.namespaces = other.namespaces;
        if let Some(other_value) = other.resources {
            self.resources = other_value;
        }
        if let Some(other_value) = other.verbs {
            self.verbs = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::ResourcePolicyRule>
for ResourcePolicyRuleAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::ResourcePolicyRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::flowcontrol::v1::ResourcePolicyRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::ResourcePolicyRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
