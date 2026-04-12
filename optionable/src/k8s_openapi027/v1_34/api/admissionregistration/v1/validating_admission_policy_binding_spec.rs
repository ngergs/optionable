#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidatingAdmissionPolicyBindingSpecAc {
    /// MatchResources declares what resources match this binding and will be validated by it. Note that this is intersected with the policy's matchConstraints, so only requests that are matched by the policy can be selected by this. If this is unset, all resources matched by the policy are validated by this binding When resourceRules is unset, it does not constrain resource matching. If a resource is matched by the other fields of this object, it will be validated. Note that this is differs from ValidatingAdmissionPolicy matchConstraints, where resourceRules are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_resources: Option<
        <::k8s_openapi027::api::admissionregistration::v1::MatchResources as crate::Optionable>::Optioned,
    >,
    /// paramRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in ParamKind of the bound ValidatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the ValidatingAdmissionPolicy applied. If the policy does not specify a ParamKind then this field is ignored, and the rules are evaluated without a param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_ref: Option<
        <::k8s_openapi027::api::admissionregistration::v1::ParamRef as crate::Optionable>::Optioned,
    >,
    /// PolicyName references a ValidatingAdmissionPolicy name which the ValidatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<std::string::String>,
    /// validationActions declares how Validations of the referenced ValidatingAdmissionPolicy are enforced. If a validation evaluates to false it is always enforced according to these actions.
    ///
    /// Failures defined by the ValidatingAdmissionPolicy's FailurePolicy are enforced according to these actions only if the FailurePolicy is set to Fail, otherwise the failures are ignored. This includes compilation errors, runtime errors and misconfigurations of the policy.
    ///
    /// validationActions is declared as a set of action values. Order does not matter. validationActions may not contain duplicates of the same action.
    ///
    /// The supported actions values are:
    ///
    /// "Deny" specifies that a validation failure results in a denied request.
    ///
    /// "Warn" specifies that a validation failure is reported to the request client in HTTP Warning headers, with a warning code of 299. Warnings can be sent both for allowed or denied admission responses.
    ///
    /// "Audit" specifies that a validation failure is included in the published audit event for the request. The audit event will contain a `validation.policy.admission.k8s.io/validation_failure` audit annotation with a value containing the details of the validation failures, formatted as a JSON list of objects, each with the following fields: - message: The validation failure message string - policy: The resource name of the ValidatingAdmissionPolicy - binding: The resource name of the ValidatingAdmissionPolicyBinding - expressionIndex: The index of the failed validations in the ValidatingAdmissionPolicy - validationActions: The enforcement actions enacted for the validation failure Example audit annotation: `"validation.policy.admission.k8s.io/validation_failure": "\[{\\"message\\": \\"Invalid value\\", {\\"policy\\": \\"policy.example.com\\", {\\"binding\\": \\"policybinding.example.com\\", {\\"expressionIndex\\": \\"1\\", {\\"validationActions\\": \[\\"Audit\\"\]}\]"`
    ///
    /// Clients should expect to handle additional values by ignoring any values not recognized.
    ///
    /// "Deny" and "Warn" may not be used together since this combination needlessly duplicates the validation failure both in the API response body and the HTTP warning headers.
    ///
    /// Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_actions: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec {
    type Optioned = ValidatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyBindingSpecAc {
    type Optioned = ValidatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec {
    fn into_optioned(self) -> ValidatingAdmissionPolicyBindingSpecAc {
        ValidatingAdmissionPolicyBindingSpecAc {
            match_resources: crate::OptionableConvert::into_optioned(
                self.match_resources,
            ),
            param_ref: crate::OptionableConvert::into_optioned(self.param_ref),
            policy_name: self.policy_name,
            validation_actions: self.validation_actions,
        }
    }
    fn try_from_optioned(
        value: ValidatingAdmissionPolicyBindingSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            match_resources: crate::OptionableConvert::try_from_optioned(
                value.match_resources,
            )?,
            param_ref: crate::OptionableConvert::try_from_optioned(value.param_ref)?,
            policy_name: value.policy_name,
            validation_actions: value.validation_actions,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingAdmissionPolicyBindingSpecAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_resources,
            other.match_resources,
        )?;
        crate::OptionableConvert::merge(&mut self.param_ref, other.param_ref)?;
        if other.policy_name.is_some() {
            self.policy_name = other.policy_name;
        }
        if other.validation_actions.is_some() {
            self.validation_actions = other.validation_actions;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec,
> for ValidatingAdmissionPolicyBindingSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
