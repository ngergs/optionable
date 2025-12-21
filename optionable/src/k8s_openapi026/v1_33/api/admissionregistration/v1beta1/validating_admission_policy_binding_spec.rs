#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidatingAdmissionPolicyBindingSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_resources: <Option<
        ::k8s_openapi026::api::admissionregistration::v1beta1::MatchResources,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_ref: <Option<
        ::k8s_openapi026::api::admissionregistration::v1beta1::ParamRef,
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
for k8s_openapi026::api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBindingSpec {
    type Optioned = ValidatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyBindingSpecAc {
    type Optioned = ValidatingAdmissionPolicyBindingSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBindingSpec {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBindingSpec,
> for ValidatingAdmissionPolicyBindingSpecAc {
    fn from_optionable(
        value: k8s_openapi026::api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBindingSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBindingSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::admissionregistration::v1beta1::ValidatingAdmissionPolicyBindingSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
