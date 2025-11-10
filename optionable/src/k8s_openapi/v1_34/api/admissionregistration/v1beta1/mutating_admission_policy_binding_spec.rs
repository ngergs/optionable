#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct MutatingAdmissionPolicyBindingSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_resources: <Option<
        ::k8s_openapi::api::admissionregistration::v1beta1::MatchResources,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_ref: <Option<
        ::k8s_openapi::api::admissionregistration::v1beta1::ParamRef,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingSpec {
    type Optioned = MutatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicyBindingSpecAc {
    type Optioned = MutatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingSpec {
    fn into_optioned(self) -> MutatingAdmissionPolicyBindingSpecAc {
        MutatingAdmissionPolicyBindingSpecAc {
            match_resources: crate::OptionableConvert::into_optioned(
                self.match_resources,
            ),
            param_ref: crate::OptionableConvert::into_optioned(self.param_ref),
            policy_name: crate::OptionableConvert::into_optioned(self.policy_name),
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
            policy_name: crate::OptionableConvert::try_from_optioned(value.policy_name)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicyBindingSpecAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_resources,
            other.match_resources,
        )?;
        crate::OptionableConvert::merge(&mut self.param_ref, other.param_ref)?;
        crate::OptionableConvert::merge(&mut self.policy_name, other.policy_name)?;
        Ok(())
    }
}
