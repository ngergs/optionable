#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MutatingAdmissionPolicyBindingSpec is the specification of the MutatingAdmissionPolicyBinding.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MutatingAdmissionPolicyBindingSpecAc {
    /// matchResources limits what resources match this binding and may be mutated by it. Note that if matchResources matches a resource, the resource must also match a policy's matchConstraints and matchConditions before the resource may be mutated. When matchResources is unset, it does not constrain resource matching, and only the policy's matchConstraints and matchConditions must match for the resource to be mutated. Additionally, matchResources.resourceRules are optional and do not constraint matching when unset. Note that this is differs from MutatingAdmissionPolicy matchConstraints, where resourceRules are required. The CREATE, UPDATE and CONNECT operations are allowed.  The DELETE operation may not be matched. '*' matches CREATE, UPDATE and CONNECT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_resources: Option<
        <::k8s_openapi027::api::admissionregistration::v1alpha1::MatchResources as crate::Optionable>::Optioned,
    >,
    /// paramRef specifies the parameter resource used to configure the admission control policy. It should point to a resource of the type specified in spec.ParamKind of the bound MutatingAdmissionPolicy. If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the MutatingAdmissionPolicy applied. If the policy does not specify a ParamKind then this field is ignored, and the rules are evaluated without a param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_ref: Option<
        <::k8s_openapi027::api::admissionregistration::v1alpha1::ParamRef as crate::Optionable>::Optioned,
    >,
    /// policyName references a MutatingAdmissionPolicy name which the MutatingAdmissionPolicyBinding binds to. If the referenced resource does not exist, this binding is considered invalid and will be ignored Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingSpec {
    type Optioned = MutatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicyBindingSpecAc {
    type Optioned = MutatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingSpec {
    fn into_optioned(self) -> MutatingAdmissionPolicyBindingSpecAc {
        MutatingAdmissionPolicyBindingSpecAc {
            match_resources: crate::OptionableConvert::into_optioned(
                self.match_resources,
            ),
            param_ref: crate::OptionableConvert::into_optioned(self.param_ref),
            policy_name: self.policy_name,
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicyBindingSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            match_resources: crate::OptionableConvert::try_from_optioned(
                value.match_resources,
            )?,
            param_ref: crate::OptionableConvert::try_from_optioned(value.param_ref)?,
            policy_name: value.policy_name,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicyBindingSpecAc,
    ) -> Result<(), crate::Error> {
        if self.match_resources.is_none() {
            self.match_resources = crate::OptionableConvert::try_from_optioned(
                other.match_resources,
            )?;
        } else if let Some(self_value) = self.match_resources.as_mut()
            && let Some(other_value) = other.match_resources
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.param_ref.is_none() {
            self.param_ref = crate::OptionableConvert::try_from_optioned(
                other.param_ref,
            )?;
        } else if let Some(self_value) = self.param_ref.as_mut()
            && let Some(other_value) = other.param_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.policy_name.is_none() {
            self.policy_name = crate::OptionableConvert::try_from_optioned(
                other.policy_name,
            )?;
        } else if let Some(self_value) = self.policy_name.as_mut()
            && let Some(other_value) = other.policy_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingSpec,
> for MutatingAdmissionPolicyBindingSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for MutatingAdmissionPolicyBindingSpecAc {
    fn merge_from(&mut self, other: Self) {
        self.match_resources = other.match_resources;
        k8s_openapi027::DeepMerge::merge_from(&mut self.param_ref, other.param_ref);
        k8s_openapi027::DeepMerge::merge_from(&mut self.policy_name, other.policy_name);
    }
}
