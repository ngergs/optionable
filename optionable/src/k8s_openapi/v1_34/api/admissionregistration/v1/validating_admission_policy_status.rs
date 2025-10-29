pub struct ValidatingAdmissionPolicyStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
    > as crate::Optionable>::Optioned,
    pub observed_generation: <Option<i64> as crate::Optionable>::Optioned,
    pub type_checking: <Option<
        ::k8s_openapi::api::admissionregistration::v1::TypeChecking,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyStatus {
    type Optioned = ValidatingAdmissionPolicyStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyStatusOpt {
    type Optioned = ValidatingAdmissionPolicyStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyStatus {
    fn into_optioned(self) -> ValidatingAdmissionPolicyStatusOpt {
        ValidatingAdmissionPolicyStatusOpt {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            observed_generation: crate::OptionableConvert::into_optioned(
                self.observed_generation,
            ),
            type_checking: crate::OptionableConvert::into_optioned(self.type_checking),
        }
    }
    fn try_from_optioned(
        value: ValidatingAdmissionPolicyStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            observed_generation: crate::OptionableConvert::try_from_optioned(
                value.observed_generation,
            )?,
            type_checking: crate::OptionableConvert::try_from_optioned(
                value.type_checking,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingAdmissionPolicyStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(
            &mut self.observed_generation,
            other.observed_generation,
        )?;
        crate::OptionableConvert::merge(&mut self.type_checking, other.type_checking)?;
        Ok(())
    }
}
