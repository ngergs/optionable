pub struct MutatingAdmissionPolicyBindingOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding {
    type Optioned = MutatingAdmissionPolicyBindingOpt;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicyBindingOpt {
    type Optioned = MutatingAdmissionPolicyBindingOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding {
    fn into_optioned(self) -> MutatingAdmissionPolicyBindingOpt {
        MutatingAdmissionPolicyBindingOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicyBindingOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicyBindingOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
