pub struct MutatingAdmissionPolicySpecAc {
    pub failure_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub match_conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1beta1::MatchCondition>,
    > as crate::Optionable>::Optioned,
    pub match_constraints: <Option<
        ::k8s_openapi::api::admissionregistration::v1beta1::MatchResources,
    > as crate::Optionable>::Optioned,
    pub mutations: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1beta1::Mutation>,
    > as crate::Optionable>::Optioned,
    pub param_kind: <Option<
        ::k8s_openapi::api::admissionregistration::v1beta1::ParamKind,
    > as crate::Optionable>::Optioned,
    pub reinvocation_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub variables: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1beta1::Variable>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec {
    type Optioned = MutatingAdmissionPolicySpecAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicySpecAc {
    type Optioned = MutatingAdmissionPolicySpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec {
    fn into_optioned(self) -> MutatingAdmissionPolicySpecAc {
        MutatingAdmissionPolicySpecAc {
            failure_policy: crate::OptionableConvert::into_optioned(self.failure_policy),
            match_conditions: crate::OptionableConvert::into_optioned(
                self.match_conditions,
            ),
            match_constraints: crate::OptionableConvert::into_optioned(
                self.match_constraints,
            ),
            mutations: crate::OptionableConvert::into_optioned(self.mutations),
            param_kind: crate::OptionableConvert::into_optioned(self.param_kind),
            reinvocation_policy: crate::OptionableConvert::into_optioned(
                self.reinvocation_policy,
            ),
            variables: crate::OptionableConvert::into_optioned(self.variables),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicySpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            failure_policy: crate::OptionableConvert::try_from_optioned(
                value.failure_policy,
            )?,
            match_conditions: crate::OptionableConvert::try_from_optioned(
                value.match_conditions,
            )?,
            match_constraints: crate::OptionableConvert::try_from_optioned(
                value.match_constraints,
            )?,
            mutations: crate::OptionableConvert::try_from_optioned(value.mutations)?,
            param_kind: crate::OptionableConvert::try_from_optioned(value.param_kind)?,
            reinvocation_policy: crate::OptionableConvert::try_from_optioned(
                value.reinvocation_policy,
            )?,
            variables: crate::OptionableConvert::try_from_optioned(value.variables)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicySpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.failure_policy, other.failure_policy)?;
        crate::OptionableConvert::merge(
            &mut self.match_conditions,
            other.match_conditions,
        )?;
        crate::OptionableConvert::merge(
            &mut self.match_constraints,
            other.match_constraints,
        )?;
        crate::OptionableConvert::merge(&mut self.mutations, other.mutations)?;
        crate::OptionableConvert::merge(&mut self.param_kind, other.param_kind)?;
        crate::OptionableConvert::merge(
            &mut self.reinvocation_policy,
            other.reinvocation_policy,
        )?;
        crate::OptionableConvert::merge(&mut self.variables, other.variables)?;
        Ok(())
    }
}
