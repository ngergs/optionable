pub struct ValidatingAdmissionPolicyBindingOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyBinding {
    type Optioned = ValidatingAdmissionPolicyBindingOpt;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyBindingOpt {
    type Optioned = ValidatingAdmissionPolicyBindingOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyBinding {
    fn into_optioned(self) -> ValidatingAdmissionPolicyBindingOpt {
        ValidatingAdmissionPolicyBindingOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: ValidatingAdmissionPolicyBindingOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingAdmissionPolicyBindingOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
