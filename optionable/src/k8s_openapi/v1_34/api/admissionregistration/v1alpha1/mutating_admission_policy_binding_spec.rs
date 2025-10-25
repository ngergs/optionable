pub struct MutatingAdmissionPolicyBindingSpecOpt {
    pub match_resources: <Option<
        ::k8s_openapi::api::admissionregistration::v1alpha1::MatchResources,
    > as crate::Optionable>::Optioned,
    pub param_ref: <Option<
        ::k8s_openapi::api::admissionregistration::v1alpha1::ParamRef,
    > as crate::Optionable>::Optioned,
    pub policy_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingSpec {
    type Optioned = MutatingAdmissionPolicyBindingSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicyBindingSpecOpt {
    type Optioned = MutatingAdmissionPolicyBindingSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicyBindingSpec {
    fn into_optioned(self) -> MutatingAdmissionPolicyBindingSpecOpt {
        MutatingAdmissionPolicyBindingSpecOpt {
            match_resources: <Option<
                ::k8s_openapi::api::admissionregistration::v1alpha1::MatchResources,
            > as crate::OptionableConvert>::into_optioned(self.match_resources),
            param_ref: <Option<
                ::k8s_openapi::api::admissionregistration::v1alpha1::ParamRef,
            > as crate::OptionableConvert>::into_optioned(self.param_ref),
            policy_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.policy_name),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicyBindingSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            match_resources: <Option<
                ::k8s_openapi::api::admissionregistration::v1alpha1::MatchResources,
            > as crate::OptionableConvert>::try_from_optioned(value.match_resources)?,
            param_ref: <Option<
                ::k8s_openapi::api::admissionregistration::v1alpha1::ParamRef,
            > as crate::OptionableConvert>::try_from_optioned(value.param_ref)?,
            policy_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.policy_name)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicyBindingSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::admissionregistration::v1alpha1::MatchResources,
        > as crate::OptionableConvert>::merge(
            &mut self.match_resources,
            other.match_resources,
        )?;
        <Option<
            ::k8s_openapi::api::admissionregistration::v1alpha1::ParamRef,
        > as crate::OptionableConvert>::merge(&mut self.param_ref, other.param_ref)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.policy_name, other.policy_name)?;
        Ok(())
    }
}
