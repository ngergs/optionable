#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MutatingAdmissionPolicySpec is the specification of the desired behavior of the admission policy.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MutatingAdmissionPolicySpecAc {
    /// failurePolicy defines how to handle failures for the admission policy. Failures can occur from CEL expression parse errors, type check errors, runtime errors and invalid or mis-configured policy definitions or bindings.
    ///
    /// A policy is invalid if paramKind refers to a non-existent Kind. A binding is invalid if paramRef.name refers to a non-existent resource.
    ///
    /// failurePolicy does not define how validations that evaluate to false are handled.
    ///
    /// Allowed values are Ignore or Fail. Defaults to Fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<std::string::String>,
    /// matchConditions is a list of conditions that must be met for a request to be validated. Match conditions filter requests that have already been matched by the matchConstraints. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.
    ///
    /// If a parameter object is provided, it can be accessed via the `params` handle in the same manner as validation expressions.
    ///
    /// The exact matching logic is (in order):
    ///   1. If ANY matchCondition evaluates to FALSE, the policy is skipped.
    ///   2. If ALL matchConditions evaluate to TRUE, the policy is evaluated.
    ///   3. If any matchCondition evaluates to an error (but none are FALSE):
    ///      - If failurePolicy=Fail, reject the request
    ///      - If failurePolicy=Ignore, the policy is skipped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1beta1::MatchCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// matchConstraints specifies what resources this policy is designed to validate. The MutatingAdmissionPolicy cares about a request if it matches _all_ Constraints. However, in order to prevent clusters from being put into an unstable state that cannot be recovered from via the API MutatingAdmissionPolicy cannot match MutatingAdmissionPolicy and MutatingAdmissionPolicyBinding. The CREATE, UPDATE and CONNECT operations are allowed.  The DELETE operation may not be matched. '*' matches CREATE, UPDATE and CONNECT. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<
        <::k8s_openapi027::api::admissionregistration::v1beta1::MatchResources as crate::Optionable>::Optioned,
    >,
    /// mutations contain operations to perform on matching objects. mutations may not be empty; a minimum of one mutation is required. mutations are evaluated in order, and are reinvoked according to the reinvocationPolicy. The mutations of a policy are invoked for each binding of this policy and reinvocation of mutations occurs on a per binding basis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1beta1::Mutation as crate::Optionable>::Optioned,
        >,
    >,
    /// paramKind specifies the kind of resources used to parameterize this policy. If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions. If paramKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied. If paramKind is specified but paramRef is unset in MutatingAdmissionPolicyBinding, the params variable will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<
        <::k8s_openapi027::api::admissionregistration::v1beta1::ParamKind as crate::Optionable>::Optioned,
    >,
    /// reinvocationPolicy indicates whether mutations may be called multiple times per MutatingAdmissionPolicyBinding as part of a single admission evaluation. Allowed values are "Never" and "IfNeeded".
    ///
    /// Never: These mutations will not be called more than once per binding in a single admission evaluation.
    ///
    /// IfNeeded: These mutations may be invoked more than once per binding for a single admission request and there is no guarantee of order with respect to other admission plugins, admission webhooks, bindings of this policy and admission policies.  Mutations are only reinvoked when mutations change the object after this mutation is invoked. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reinvocation_policy: Option<std::string::String>,
    /// variables contain definitions of variables that can be used in composition of other expressions. Each variable is defined as a named CEL expression. The variables defined here will be available under `variables` in other expressions of the policy except matchConditions because matchConditions are evaluated before the rest of the policy.
    ///
    /// The expression of a variable can refer to other variables defined earlier in the list but not those after. Thus, variables must be sorted by the order of first appearance and acyclic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1beta1::Variable as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec {
    type Optioned = MutatingAdmissionPolicySpecAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicySpecAc {
    type Optioned = MutatingAdmissionPolicySpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec {
    fn into_optioned(self) -> MutatingAdmissionPolicySpecAc {
        MutatingAdmissionPolicySpecAc {
            failure_policy: self.failure_policy,
            match_conditions: crate::OptionableConvert::into_optioned(
                self.match_conditions,
            ),
            match_constraints: crate::OptionableConvert::into_optioned(
                self.match_constraints,
            ),
            mutations: crate::OptionableConvert::into_optioned(self.mutations),
            param_kind: crate::OptionableConvert::into_optioned(self.param_kind),
            reinvocation_policy: self.reinvocation_policy,
            variables: crate::OptionableConvert::into_optioned(self.variables),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicySpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            failure_policy: value.failure_policy,
            match_conditions: crate::OptionableConvert::try_from_optioned(
                value.match_conditions,
            )?,
            match_constraints: crate::OptionableConvert::try_from_optioned(
                value.match_constraints,
            )?,
            mutations: crate::OptionableConvert::try_from_optioned(value.mutations)?,
            param_kind: crate::OptionableConvert::try_from_optioned(value.param_kind)?,
            reinvocation_policy: value.reinvocation_policy,
            variables: crate::OptionableConvert::try_from_optioned(value.variables)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicySpecAc,
    ) -> Result<(), crate::Error> {
        if self.failure_policy.is_none() {
            self.failure_policy = other.failure_policy;
        }
        if let Some(other_value) = other.failure_policy {
            crate::OptionableConvert::merge(&mut self.failure_policy, other_value)?;
        }
        if self.match_conditions.is_none() {
            self.match_conditions = other.match_conditions;
        }
        if let Some(other_value) = other.match_conditions {
            crate::merge::try_merge_optioned_map(
                &mut self.match_conditions,
                other_value,
            )?;
        }
        if self.match_constraints.is_none() {
            self.match_constraints = other.match_constraints;
        }
        if let Some(other_value) = other.match_constraints {
            crate::OptionableConvert::merge(&mut self.match_constraints, other_value)?;
        }
        if self.mutations.is_none() {
            self.mutations = other.mutations;
        }
        if let Some(other_value) = other.mutations {
            self.mutations = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.param_kind.is_none() {
            self.param_kind = other.param_kind;
        }
        if let Some(other_value) = other.param_kind {
            crate::OptionableConvert::merge(&mut self.param_kind, other_value)?;
        }
        if self.reinvocation_policy.is_none() {
            self.reinvocation_policy = other.reinvocation_policy;
        }
        if let Some(other_value) = other.reinvocation_policy {
            crate::OptionableConvert::merge(&mut self.reinvocation_policy, other_value)?;
        }
        if self.variables.is_none() {
            self.variables = other.variables;
        }
        if let Some(other_value) = other.variables {
            self.variables = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec,
> for MutatingAdmissionPolicySpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
