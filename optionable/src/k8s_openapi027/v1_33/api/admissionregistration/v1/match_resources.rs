#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MatchResources decides whether to run the admission control policy on an object based on whether it meets the match criteria. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MatchResourcesAc {
    /// ExcludeResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy should not care about. The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_resource_rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1::NamedRuleWithOperations as crate::Optionable>::Optioned,
        >,
    >,
    /// matchPolicy defines how the "MatchResources" list is used to match incoming requests. Allowed values are "Exact" or "Equivalent".
    ///
    /// - Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the ValidatingAdmissionPolicy.
    ///
    /// - Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the ValidatingAdmissionPolicy.
    ///
    /// Defaults to "Equivalent"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<std::string::String>,
    /// NamespaceSelector decides whether to run the admission control policy on an object based on whether the namespace for that object matches the selector. If the object itself is a namespace, the matching is performed on object.metadata.labels. If the object is another cluster scoped resource, it never skips the policy.
    ///
    /// For example, to run the webhook on any objects whose namespace is not associated with "runlevel" of "0" or "1";  you will set the selector as follows: "namespaceSelector": {
    ///   "matchExpressions": \[
    ///     {
    ///       "key": "runlevel",
    ///       "operator": "NotIn",
    ///       "values": \[
    ///         "0",
    ///         "1"
    ///       \]
    ///     }
    ///   \]
    /// }
    ///
    /// If instead you want to only run the policy on any objects whose namespace is associated with the "environment" of "prod" or "staging"; you will set the selector as follows: "namespaceSelector": {
    ///   "matchExpressions": \[
    ///     {
    ///       "key": "environment",
    ///       "operator": "In",
    ///       "values": \[
    ///         "prod",
    ///         "staging"
    ///       \]
    ///     }
    ///   \]
    /// }
    ///
    /// See https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/ for more examples of label selectors.
    ///
    /// Default to the empty LabelSelector, which matches everything.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// ObjectSelector decides whether to run the validation based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the cel validation, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// ResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy matches. The policy cares about an operation if it matches _any_ Rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1::NamedRuleWithOperations as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::MatchResources {
    type Optioned = MatchResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for MatchResourcesAc {
    type Optioned = MatchResourcesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::MatchResources {
    fn into_optioned(self) -> MatchResourcesAc {
        MatchResourcesAc {
            exclude_resource_rules: crate::OptionableConvert::into_optioned(
                self.exclude_resource_rules,
            ),
            match_policy: self.match_policy,
            namespace_selector: crate::OptionableConvert::into_optioned(
                self.namespace_selector,
            ),
            object_selector: crate::OptionableConvert::into_optioned(
                self.object_selector,
            ),
            resource_rules: crate::OptionableConvert::into_optioned(self.resource_rules),
        }
    }
    fn try_from_optioned(value: MatchResourcesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            exclude_resource_rules: crate::OptionableConvert::try_from_optioned(
                value.exclude_resource_rules,
            )?,
            match_policy: value.match_policy,
            namespace_selector: crate::OptionableConvert::try_from_optioned(
                value.namespace_selector,
            )?,
            object_selector: crate::OptionableConvert::try_from_optioned(
                value.object_selector,
            )?,
            resource_rules: crate::OptionableConvert::try_from_optioned(
                value.resource_rules,
            )?,
        })
    }
    fn merge(&mut self, other: MatchResourcesAc) -> Result<(), crate::Error> {
        if self.exclude_resource_rules.is_none() {
            self.exclude_resource_rules = crate::OptionableConvert::try_from_optioned(
                other.exclude_resource_rules,
            )?;
        } else if let Some(self_value) = self.exclude_resource_rules.as_mut()
            && let Some(other_value) = other.exclude_resource_rules
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.match_policy.is_none() {
            self.match_policy = crate::OptionableConvert::try_from_optioned(
                other.match_policy,
            )?;
        } else if let Some(self_value) = self.match_policy.as_mut()
            && let Some(other_value) = other.match_policy
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.namespace_selector.is_none() {
            self.namespace_selector = crate::OptionableConvert::try_from_optioned(
                other.namespace_selector,
            )?;
        } else if let Some(self_value) = self.namespace_selector.as_mut()
            && let Some(other_value) = other.namespace_selector
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.object_selector.is_none() {
            self.object_selector = crate::OptionableConvert::try_from_optioned(
                other.object_selector,
            )?;
        } else if let Some(self_value) = self.object_selector.as_mut()
            && let Some(other_value) = other.object_selector
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.resource_rules.is_none() {
            self.resource_rules = crate::OptionableConvert::try_from_optioned(
                other.resource_rules,
            )?;
        } else if let Some(self_value) = self.resource_rules.as_mut()
            && let Some(other_value) = other.resource_rules
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::MatchResources,
> for MatchResourcesAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::MatchResources,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::MatchResources,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::MatchResources,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for MatchResourcesAc {
    fn merge_from(&mut self, other: Self) {
        self.exclude_resource_rules = other.exclude_resource_rules;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.match_policy,
            other.match_policy,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.namespace_selector,
            other.namespace_selector,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.object_selector,
            other.object_selector,
        );
        self.resource_rules = other.resource_rules;
    }
}
