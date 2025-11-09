#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBindingSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_resources: <Option<
        ::k8s_openapi::api::admissionregistration::v1::MatchResources,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_ref: <Option<
        ::k8s_openapi::api::admissionregistration::v1::ParamRef,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_actions: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec {
    type Optioned = ValidatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyBindingSpecAc {
    type Optioned = ValidatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec {
    fn into_optioned(self) -> ValidatingAdmissionPolicyBindingSpecAc {
        ValidatingAdmissionPolicyBindingSpecAc {
            match_resources: crate::OptionableConvert::into_optioned(
                self.match_resources,
            ),
            param_ref: crate::OptionableConvert::into_optioned(self.param_ref),
            policy_name: crate::OptionableConvert::into_optioned(self.policy_name),
            validation_actions: crate::OptionableConvert::into_optioned(
                self.validation_actions,
            ),
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
            policy_name: crate::OptionableConvert::try_from_optioned(value.policy_name)?,
            validation_actions: crate::OptionableConvert::try_from_optioned(
                value.validation_actions,
            )?,
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
        crate::OptionableConvert::merge(&mut self.policy_name, other.policy_name)?;
        crate::OptionableConvert::merge(
            &mut self.validation_actions,
            other.validation_actions,
        )?;
        Ok(())
    }
}
