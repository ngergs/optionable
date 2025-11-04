#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicySpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_annotations: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::AuditAnnotation>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::MatchCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_constraints: <Option<
        ::k8s_openapi::api::admissionregistration::v1::MatchResources,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_kind: <Option<
        ::k8s_openapi::api::admissionregistration::v1::ParamKind,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::Validation>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::Variable>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicySpec {
    type Optioned = ValidatingAdmissionPolicySpecAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicySpecAc {
    type Optioned = ValidatingAdmissionPolicySpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicySpec {
    fn into_optioned(self) -> ValidatingAdmissionPolicySpecAc {
        ValidatingAdmissionPolicySpecAc {
            audit_annotations: crate::OptionableConvert::into_optioned(
                self.audit_annotations,
            ),
            failure_policy: crate::OptionableConvert::into_optioned(self.failure_policy),
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
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            audit_annotations: crate::OptionableConvert::try_from_optioned(
                value.audit_annotations,
            )?,
            failure_policy: crate::OptionableConvert::try_from_optioned(
                value.failure_policy,
            )?,
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
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.audit_annotations,
            other.audit_annotations,
        )?;
        crate::OptionableConvert::merge(&mut self.failure_policy, other.failure_policy)?;
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
