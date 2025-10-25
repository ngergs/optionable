pub struct MutatingAdmissionPolicySpecOpt {
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
for ::k8s_openapi::api::admissionregistration::v1beta1::mutating_admission_policy_spec::MutatingAdmissionPolicySpec {
    type Optioned = MutatingAdmissionPolicySpecOpt;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicySpecOpt {
    type Optioned = MutatingAdmissionPolicySpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1beta1::mutating_admission_policy_spec::MutatingAdmissionPolicySpec {
    fn into_optioned(self) -> MutatingAdmissionPolicySpecOpt {
        MutatingAdmissionPolicySpecOpt {
            failure_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.failure_policy),
            match_conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1beta1::MatchCondition,
                >,
            > as crate::OptionableConvert>::into_optioned(self.match_conditions),
            match_constraints: <Option<
                ::k8s_openapi::api::admissionregistration::v1beta1::MatchResources,
            > as crate::OptionableConvert>::into_optioned(self.match_constraints),
            mutations: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1beta1::Mutation,
                >,
            > as crate::OptionableConvert>::into_optioned(self.mutations),
            param_kind: <Option<
                ::k8s_openapi::api::admissionregistration::v1beta1::ParamKind,
            > as crate::OptionableConvert>::into_optioned(self.param_kind),
            reinvocation_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reinvocation_policy),
            variables: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1beta1::Variable,
                >,
            > as crate::OptionableConvert>::into_optioned(self.variables),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicySpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            failure_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.failure_policy)?,
            match_conditions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1beta1::MatchCondition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.match_conditions)?,
            match_constraints: <Option<
                ::k8s_openapi::api::admissionregistration::v1beta1::MatchResources,
            > as crate::OptionableConvert>::try_from_optioned(value.match_constraints)?,
            mutations: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1beta1::Mutation,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.mutations)?,
            param_kind: <Option<
                ::k8s_openapi::api::admissionregistration::v1beta1::ParamKind,
            > as crate::OptionableConvert>::try_from_optioned(value.param_kind)?,
            reinvocation_policy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.reinvocation_policy,
            )?,
            variables: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1beta1::Variable,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.variables)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicySpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.failure_policy,
            other.failure_policy,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::admissionregistration::v1beta1::MatchCondition,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.match_conditions,
            other.match_conditions,
        )?;
        <Option<
            ::k8s_openapi::api::admissionregistration::v1beta1::MatchResources,
        > as crate::OptionableConvert>::merge(
            &mut self.match_constraints,
            other.match_constraints,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::admissionregistration::v1beta1::Mutation>,
        > as crate::OptionableConvert>::merge(&mut self.mutations, other.mutations)?;
        <Option<
            ::k8s_openapi::api::admissionregistration::v1beta1::ParamKind,
        > as crate::OptionableConvert>::merge(&mut self.param_kind, other.param_kind)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.reinvocation_policy,
            other.reinvocation_policy,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::admissionregistration::v1beta1::Variable>,
        > as crate::OptionableConvert>::merge(&mut self.variables, other.variables)?;
        Ok(())
    }
}
