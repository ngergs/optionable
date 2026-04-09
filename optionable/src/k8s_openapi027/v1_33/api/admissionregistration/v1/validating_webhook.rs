#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ValidatingWebhook describes an admission webhook and the resources and operations it applies to.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidatingWebhookAc {
    /// AdmissionReviewVersions is an ordered list of preferred `AdmissionReview` versions the Webhook expects. API server will try to use first version in the list which it supports. If none of the versions specified in this list supported by API server, validation will fail for this object. If a persisted webhook configuration specifies allowed versions and does not include any versions known to the API Server, calls to the webhook will fail and be subject to the failure policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admission_review_versions: Option<std::vec::Vec<std::string::String>>,
    /// ClientConfig defines how to communicate with the hook. Required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_config: Option<
        <::k8s_openapi027::api::admissionregistration::v1::WebhookClientConfig as crate::Optionable>::Optioned,
    >,
    /// FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<std::string::String>,
    /// MatchConditions is a list of conditions that must be met for a request to be sent to this webhook. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.
    ///
    /// The exact matching logic is (in order):
    ///   1. If ANY matchCondition evaluates to FALSE, the webhook is skipped.
    ///   2. If ALL matchConditions evaluate to TRUE, the webhook is called.
    ///   3. If any matchCondition evaluates to an error (but none are FALSE):
    ///      - If failurePolicy=Fail, reject the request
    ///      - If failurePolicy=Ignore, the error is ignored and the webhook is skipped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1::MatchCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// matchPolicy defines how the "rules" list is used to match incoming requests. Allowed values are "Exact" or "Equivalent".
    ///
    /// - Exact: match a request only if it exactly matches a specified rule. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, but "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`, a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the webhook.
    ///
    /// - Equivalent: match a request if modifies a resource listed in rules, even via another API group or version. For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1, and "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`, a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the webhook.
    ///
    /// Defaults to "Equivalent"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_policy: Option<std::string::String>,
    /// The name of the admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where "imagepolicy" is the name of the webhook, and kubernetes.io is the name of the organization. Required.
    pub name: std::string::String,
    /// NamespaceSelector decides whether to run the webhook on an object based on whether the namespace for that object matches the selector. If the object itself is a namespace, the matching is performed on object.metadata.labels. If the object is another cluster scoped resource, it never skips the webhook.
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
    /// If instead you want to only run the webhook on any objects whose namespace is associated with the "environment" of "prod" or "staging"; you will set the selector as follows: "namespaceSelector": {
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
    /// See https://kubernetes.io/docs/concepts/overview/working-with-objects/labels for more examples of label selectors.
    ///
    /// Default to the empty LabelSelector, which matches everything.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// ObjectSelector decides whether to run the webhook based on if the object has matching labels. objectSelector is evaluated against both the oldObject and newObject that would be sent to the webhook, and is considered to match if either object matches the selector. A null object (oldObject in the case of create, or newObject in the case of delete) or an object that cannot have labels (like a DeploymentRollback or a PodProxyOptions object) is not considered to match. Use the object selector only if the webhook is opt-in, because end users may skip the admission webhook by setting the labels. Default to the empty LabelSelector, which matches everything.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. However, in order to prevent ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks from putting the cluster in a state which cannot be recovered from without completely disabling the plugin, ValidatingAdmissionWebhooks and MutatingAdmissionWebhooks are never called on admission requests for ValidatingWebhookConfiguration and MutatingWebhookConfiguration objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1::RuleWithOperations as crate::Optionable>::Optioned,
        >,
    >,
    /// SideEffects states whether this webhook has side effects. Acceptable values are: None, NoneOnDryRun (webhooks created via v1beta1 may also specify Some or Unknown). Webhooks with side effects MUST implement a reconciliation system, since a request may be rejected by a future step in the admission chain and the side effects therefore need to be undone. Requests with the dryRun attribute will be auto-rejected if they match a webhook with sideEffects == Unknown or Some.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_effects: Option<std::string::String>,
    /// TimeoutSeconds specifies the timeout for this webhook. After the timeout passes, the webhook call will be ignored or the API call will fail based on the failure policy. The timeout value must be between 1 and 30 seconds. Default to 10 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::ValidatingWebhook {
    type Optioned = ValidatingWebhookAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingWebhookAc {
    type Optioned = ValidatingWebhookAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::ValidatingWebhook {
    fn into_optioned(self) -> ValidatingWebhookAc {
        ValidatingWebhookAc {
            admission_review_versions: Some(self.admission_review_versions),
            client_config: Some(
                crate::OptionableConvert::into_optioned(self.client_config),
            ),
            failure_policy: self.failure_policy,
            match_conditions: crate::OptionableConvert::into_optioned(
                self.match_conditions,
            ),
            match_policy: self.match_policy,
            name: self.name,
            namespace_selector: crate::OptionableConvert::into_optioned(
                self.namespace_selector,
            ),
            object_selector: crate::OptionableConvert::into_optioned(
                self.object_selector,
            ),
            rules: crate::OptionableConvert::into_optioned(self.rules),
            side_effects: Some(self.side_effects),
            timeout_seconds: self.timeout_seconds,
        }
    }
    fn try_from_optioned(value: ValidatingWebhookAc) -> Result<Self, crate::Error> {
        Ok(Self {
            admission_review_versions: value
                .admission_review_versions
                .ok_or(crate::Error {
                    missing_field: "admission_review_versions",
                })?,
            client_config: crate::OptionableConvert::try_from_optioned(
                value
                    .client_config
                    .ok_or(crate::Error {
                        missing_field: "client_config",
                    })?,
            )?,
            failure_policy: value.failure_policy,
            match_conditions: crate::OptionableConvert::try_from_optioned(
                value.match_conditions,
            )?,
            match_policy: value.match_policy,
            name: value.name,
            namespace_selector: crate::OptionableConvert::try_from_optioned(
                value.namespace_selector,
            )?,
            object_selector: crate::OptionableConvert::try_from_optioned(
                value.object_selector,
            )?,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
            side_effects: value
                .side_effects
                .ok_or(crate::Error {
                    missing_field: "side_effects",
                })?,
            timeout_seconds: value.timeout_seconds,
        })
    }
    fn merge(&mut self, other: ValidatingWebhookAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.admission_review_versions {
            self.admission_review_versions = other_value;
        }
        if let Some(other_value) = other.client_config {
            crate::OptionableConvert::merge(&mut self.client_config, other_value)?;
        }
        self.failure_policy = other.failure_policy;
        crate::OptionableConvert::merge(
            &mut self.match_conditions,
            other.match_conditions,
        )?;
        self.match_policy = other.match_policy;
        self.name = other.name;
        crate::OptionableConvert::merge(
            &mut self.namespace_selector,
            other.namespace_selector,
        )?;
        crate::OptionableConvert::merge(
            &mut self.object_selector,
            other.object_selector,
        )?;
        crate::OptionableConvert::merge(&mut self.rules, other.rules)?;
        if let Some(other_value) = other.side_effects {
            self.side_effects = other_value;
        }
        self.timeout_seconds = other.timeout_seconds;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::ValidatingWebhook,
> for ValidatingWebhookAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::ValidatingWebhook,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::ValidatingWebhook,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::ValidatingWebhook,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
