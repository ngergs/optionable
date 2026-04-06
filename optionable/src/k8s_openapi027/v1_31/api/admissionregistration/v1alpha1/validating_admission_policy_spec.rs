#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidatingAdmissionPolicySpecAc {
    /// auditAnnotations contains CEL expressions which are used to produce audit annotations for the audit event of the API request. validations and auditAnnotations may not both be empty; a least one of validations or auditAnnotations is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_annotations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1alpha1::AuditAnnotation as crate::Optionable>::Optioned,
        >,
    >,
    /// failurePolicy defines how to handle failures for the admission policy. Failures can occur from CEL expression parse errors, type check errors, runtime errors and invalid or mis-configured policy definitions or bindings.
    ///
    /// A policy is invalid if spec.paramKind refers to a non-existent Kind. A binding is invalid if spec.paramRef.name refers to a non-existent resource.
    ///
    /// failurePolicy does not define how validations that evaluate to false are handled.
    ///
    /// When failurePolicy is set to Fail, ValidatingAdmissionPolicyBinding validationActions define how failures are enforced.
    ///
    /// Allowed values are Ignore or Fail. Defaults to Fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_policy: Option<std::string::String>,
    /// MatchConditions is a list of conditions that must be met for a request to be validated. Match conditions filter requests that have already been matched by the rules, namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests. There are a maximum of 64 match conditions allowed.
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
            <::k8s_openapi027::api::admissionregistration::v1alpha1::MatchCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// MatchConstraints specifies what resources this policy is designed to validate. The AdmissionPolicy cares about a request if it matches _all_ Constraints. However, in order to prevent clusters from being put into an unstable state that cannot be recovered from via the API ValidatingAdmissionPolicy cannot match ValidatingAdmissionPolicy and ValidatingAdmissionPolicyBinding. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_constraints: Option<
        <::k8s_openapi027::api::admissionregistration::v1alpha1::MatchResources as crate::Optionable>::Optioned,
    >,
    /// ParamKind specifies the kind of resources used to parameterize this policy. If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions. If ParamKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied. If paramKind is specified but paramRef is unset in ValidatingAdmissionPolicyBinding, the params variable will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_kind: Option<
        <::k8s_openapi027::api::admissionregistration::v1alpha1::ParamKind as crate::Optionable>::Optioned,
    >,
    /// Validations contain CEL expressions which is used to apply the validation. Validations and AuditAnnotations may not both be empty; a minimum of one Validations or AuditAnnotations is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1alpha1::Validation as crate::Optionable>::Optioned,
        >,
    >,
    /// Variables contain definitions of variables that can be used in composition of other expressions. Each variable is defined as a named CEL expression. The variables defined here will be available under `variables` in other expressions of the policy except MatchConditions because MatchConditions are evaluated before the rest of the policy.
    ///
    /// The expression of a variable can refer to other variables defined earlier in the list but not those after. Thus, Variables must be sorted by the order of first appearance and acyclic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1alpha1::Variable as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicySpec {
    type Optioned = ValidatingAdmissionPolicySpecAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicySpecAc {
    type Optioned = ValidatingAdmissionPolicySpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicySpec {
    fn into_optioned(self) -> ValidatingAdmissionPolicySpecAc {
        ValidatingAdmissionPolicySpecAc {
            audit_annotations: crate::OptionableConvert::into_optioned(
                self.audit_annotations,
            ),
            failure_policy: self.failure_policy,
            match_conditions: crate::OptionableConvert::into_optioned(
                self.match_conditions,
            ),
            match_constraints: crate::OptionableConvert::into_optioned(
                self.match_constraints,
            ),
            param_kind: crate::OptionableConvert::into_optioned(self.param_kind),
            validations: crate::OptionableConvert::into_optioned(self.validations),
            variables: crate::OptionableConvert::into_optioned(self.variables),
        }
    }
    fn try_from_optioned(
        value: ValidatingAdmissionPolicySpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            audit_annotations: crate::OptionableConvert::try_from_optioned(
                value.audit_annotations,
            )?,
            failure_policy: value.failure_policy,
            match_conditions: crate::OptionableConvert::try_from_optioned(
                value.match_conditions,
            )?,
            match_constraints: crate::OptionableConvert::try_from_optioned(
                value.match_constraints,
            )?,
            param_kind: crate::OptionableConvert::try_from_optioned(value.param_kind)?,
            validations: crate::OptionableConvert::try_from_optioned(value.validations)?,
            variables: crate::OptionableConvert::try_from_optioned(value.variables)?,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingAdmissionPolicySpecAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.audit_annotations,
            other.audit_annotations,
        )?;
        self.failure_policy = other.failure_policy;
        crate::OptionableConvert::merge(
            &mut self.match_conditions,
            other.match_conditions,
        )?;
        crate::OptionableConvert::merge(
            &mut self.match_constraints,
            other.match_constraints,
        )?;
        crate::OptionableConvert::merge(&mut self.param_kind, other.param_kind)?;
        crate::OptionableConvert::merge(&mut self.validations, other.validations)?;
        crate::OptionableConvert::merge(&mut self.variables, other.variables)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicySpec,
> for ValidatingAdmissionPolicySpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicySpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicySpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicySpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
